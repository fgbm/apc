use clap::{Arg, ArgAction, Command};
use ignore::{
    gitignore::{Gitignore, GitignoreBuilder},
    WalkBuilder,
};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

/// Structure for storing file information
#[derive(Clone)]
struct FileInfo {
    relative_path: String,
    content: String,
}

/// Structure for storing directory information
#[derive(Clone)]
struct DirInfo {
    relative_path: String,
    children: Vec<String>, // Names of child files and directories
}

/// Structure for storing project context
struct ProjectContext {
    files: Vec<FileInfo>,
    directories: Vec<DirInfo>,
}

/// Checks if a file is binary
fn is_binary(content: &[u8], sample_size: usize) -> bool {
    let sample = if content.len() > sample_size {
        &content[..sample_size]
    } else {
        content
    };

    sample.iter().any(|&byte| byte == 0)
}

/// Loads ignore rules from .apcignore file
fn load_apcignore(dir: &Path) -> Result<Option<Gitignore>, Box<dyn Error>> {
    let apcignore_path = dir.join(".apcignore");

    if apcignore_path.exists() && apcignore_path.is_file() {
        let mut builder = GitignoreBuilder::new(dir);
        builder.add(apcignore_path);
        let gitignore = builder.build()?;
        Ok(Some(gitignore))
    } else {
        Ok(None)
    }
}

/// Checks if a path should be ignored based on .apcignore rules
fn should_ignore_path(
    path: &Path,
    root_path: &Path,
    apcignores: &HashMap<PathBuf, Option<Gitignore>>,
) -> bool {
    // Always ignore .git, .idea, and .vscode directories
    if path.components().any(|c| {
        let comp = c.as_os_str();
        comp == ".git" || comp == ".idea" || comp == ".vscode"
    }) {
        return true;
    }

    // Ignore .gitignore and .apcignore files
    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
        if name == ".gitignore" || name == ".apcignore" {
            return true;
        }
    }

    // Check for ignore rules in the current path and all parent directories
    let mut current = Some(path.parent().unwrap_or(path));
    while let Some(dir) = current {
        if dir == root_path || dir.starts_with(root_path) {
            if let Some(Some(ignore)) = apcignores.get(dir) {
                let relative = path.strip_prefix(dir).unwrap_or(path);
                if ignore.matched(relative, path.is_dir()).is_ignore() {
                    return true;
                }
            }
            current = dir.parent();
        } else {
            break;
        }
    }

    false
}

/// Collects file information
fn collect_file_info(
    path: &Path,
    root_path: &Path,
    max_file_size: u64,
    include_binary: bool,
) -> Result<Option<FileInfo>, Box<dyn Error>> {
    let metadata = fs::metadata(path)?;

    if !metadata.is_file() || metadata.len() > max_file_size {
        return Ok(None);
    }

    let mut file = fs::File::open(path)?;
    let mut buffer = Vec::with_capacity(metadata.len() as usize);
    file.read_to_end(&mut buffer)?;

    if !include_binary && is_binary(&buffer, 1024) {
        return Ok(None);
    }

    let content = match String::from_utf8(buffer) {
        Ok(text) => text,
        Err(_) => {
            if include_binary {
                "[Binary content]".to_string()
            } else {
                return Ok(None);
            }
        }
    };

    let relative_path = path
        .strip_prefix(root_path)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/");

    Ok(Some(FileInfo {
        relative_path,
        content,
    }))
}

/// Collects project context
fn collect_project_context(
    root_path: &Path,
    max_file_size: u64,
    include_binary: bool,
) -> Result<ProjectContext, Box<dyn Error>> {
    let mut files = Vec::new();
    let mut dir_children: HashMap<String, Vec<String>> = HashMap::new();
    let mut apcignores: HashMap<PathBuf, Option<Gitignore>> = HashMap::new();

    // Add root directory
    dir_children.insert("".to_string(), Vec::new());

    // Load .apcignore from root directory
    apcignores.insert(root_path.to_path_buf(), load_apcignore(root_path)?);

    let mut builder = WalkBuilder::new(root_path);

    // Use .gitignore by default and in all subdirectories
    builder.git_ignore(true);
    builder.git_global(true);
    builder.git_exclude(true);
    builder.hidden(false);

    // Explicitly exclude .git directories
    builder.filter_entry(|entry| !entry.path().components().any(|c| c.as_os_str() == ".git"));

    for result in builder.build() {
        match result {
            Ok(entry) => {
                let path = entry.path();

                // Check if the current path is a directory and if so,
                // load .apcignore from it
                if path.is_dir() {
                    if !apcignores.contains_key(path) {
                        apcignores.insert(path.to_path_buf(), load_apcignore(path)?);
                    }
                }

                // Check if this path should be ignored based on .apcignore rules
                if should_ignore_path(path, root_path, &apcignores) {
                    continue;
                }

                let relative_path = path
                    .strip_prefix(root_path)
                    .unwrap_or(path)
                    .to_string_lossy()
                    .replace('\\', "/");

                if path.is_dir() {
                    add_directory(&mut dir_children, &relative_path);
                } else {
                    if let Ok(Some(file_info)) =
                        collect_file_info(path, root_path, max_file_size, include_binary)
                    {
                        add_file(&mut dir_children, &relative_path);
                        files.push(file_info);
                    }
                }
            }
            Err(err) => {
                eprintln!("Error accessing path: {}", err);
            }
        }
    }

    // Convert directory HashMap to Vec<DirInfo>
    let directories = dir_children
        .into_iter()
        .map(|(relative_path, children)| DirInfo {
            relative_path,
            children,
        })
        .collect();

    Ok(ProjectContext { files, directories })
}

/// Adds a directory to the directory tree
fn add_directory(dir_children: &mut HashMap<String, Vec<String>>, relative_path: &str) {
    dir_children.insert(relative_path.to_string(), Vec::new());

    if let Some(parent) = Path::new(relative_path).parent() {
        let parent_path = parent.to_string_lossy().replace('\\', "/");
        if let Some(children) = dir_children.get_mut(&parent_path) {
            children.push(relative_path.to_string());
        }
    }
}

/// Adds a file to the directory tree
fn add_file(
    dir_children: &mut HashMap<String, Vec<String>>,
    relative_path: &str,
) {
    if let Some(parent) = Path::new(relative_path).parent() {
        let parent_path = parent.to_string_lossy().replace('\\', "/");
        if let Some(children) = dir_children.get_mut(&parent_path) {
            children.push(relative_path.to_string());
        }
    } else if let Some(children) = dir_children.get_mut("") {
        // File in the root directory
        children.push(relative_path.to_string());
    }
}

/// Formats directory tree
fn format_directory_tree(context: &ProjectContext) -> String {
    let mut result = String::new();

    // Find the root directory
    if let Some(root_dir) = context
        .directories
        .iter()
        .find(|d| d.relative_path.is_empty())
    {
        result.push_str("./\n");
        format_directory_branch(context, root_dir, "", &mut result, true);
    }

    result
}

/// Recursively formats a branch of the directory tree
fn format_directory_branch(
    context: &ProjectContext,
    dir: &DirInfo,
    prefix: &str,
    result: &mut String,
    is_root: bool,
) {
    let new_prefix = if is_root {
        "    "
    } else {
        &format!("{}    ", prefix)
    };

    // Sort children: directories first, then files
    let mut dirs = Vec::new();
    let mut files = Vec::new();

    for child in &dir.children {
        if context
            .directories
            .iter()
            .any(|d| d.relative_path == *child)
        {
            dirs.push(child);
        } else {
            files.push(child);
        }
    }

    // Sort directories and files by name
    dirs.sort();
    files.sort();

    // Process subdirectories
    for (i, child_path) in dirs.iter().enumerate() {
        let is_last_dir = i == dirs.len() - 1 && files.is_empty();
        let prefix_char = if is_last_dir { "└" } else { "├" };

        if let Some(child_dir) = context
            .directories
            .iter()
            .find(|d| d.relative_path == **child_path)
        {
            let dir_name = Path::new(&child_dir.relative_path)
                .file_name()
                .unwrap_or_default()
                .to_string_lossy();

            result.push_str(&format!("{}{}── {}/\n", new_prefix, prefix_char, dir_name));

            let child_prefix = if is_last_dir {
                format!("{}    ", new_prefix)
            } else {
                format!("{}│   ", new_prefix)
            };

            format_directory_branch(context, child_dir, &child_prefix, result, false);
        }
    }

    // Process files
    for (i, child_path) in files.iter().enumerate() {
        let is_last = i == files.len() - 1;
        let file_name = Path::new(child_path)
            .file_name()
            .unwrap_or_default()
            .to_string_lossy();

        let prefix_char = if is_last { "└" } else { "├" };
        result.push_str(&format!("{}{}── {}\n", new_prefix, prefix_char, file_name));
    }
}

/// Formats file contents
fn format_file_contents(context: &ProjectContext) -> String {
    let mut result = String::new();

    // Sort files by path for more predictable output
    let mut sorted_files = context.files.clone();
    sorted_files.sort_by(|a, b| a.relative_path.cmp(&b.relative_path));

    for file in &sorted_files {
        result.push_str(&format!("\n--- {} ---\n", file.relative_path));
        result.push_str(&file.content);

        // Only add newline if content doesn't end with one
        if !file.content.ends_with('\n') {
            result.push('\n');
        }
    }

    result
}

/// Formats the complete project context
fn format_project_context(context: &ProjectContext) -> String {
    let mut result = String::new();

    // Add directory tree
    result.push_str("Directory Structure:\n");
    result.push_str(&format_directory_tree(context));

    // Add file contents
    result.push_str("\nFile Contents:");
    result.push_str(&format_file_contents(context));

    result
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("apc")
        .version(env!("CARGO_PKG_VERSION"))
        .about("AI Project Context - Prepares project code context for AI consumption")
        // Positional argument for path (first argument)
        .arg(
            Arg::new("path")
                .help("Path to the project directory")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("OUTPUT")
                .help("Output file path (defaults to stdout)"),
        )
        .arg(
            Arg::new("max-file-size")
                .long("max-file-size")
                .value_name("SIZE")
                .help("Maximum file size in bytes to include")
                .default_value("1048576"), // 1MB default
        )
        .arg(
            Arg::new("include-binary")
                .long("include-binary")
                .help("Include binary files (with placeholder content)")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("structure-only")
                .long("structure-only")
                .help("Only output the directory structure without file contents")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let project_path = PathBuf::from(matches.get_one::<String>("path").unwrap());

    if !project_path.exists() || !project_path.is_dir() {
        return Err("Project path must be a valid directory".into());
    }

    let max_file_size = matches
        .get_one::<String>("max-file-size")
        .unwrap()
        .parse::<u64>()
        .unwrap_or(1048576);

    let include_binary = matches.get_flag("include-binary");
    let structure_only = matches.get_flag("structure-only");

    let context = collect_project_context(&project_path, max_file_size, include_binary)?;

    let formatted_output = if structure_only {
        format_directory_tree(&context)
    } else {
        format_project_context(&context)
    };

    match matches.get_one::<String>("output") {
        Some(output_path) => {
            fs::write(output_path, formatted_output)?;
            println!("Project context written to: {}", output_path);
        }
        None => {
            println!("{}", formatted_output);
        }
    }

    Ok(())
}
