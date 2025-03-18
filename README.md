# APC - AI Project Context

Welcome to APC, your friendly command-line utility that prepares project code context for AI consumption. Think of it as
a personal assistant for your AI projects, ready to fetch, parse, and present your project's files and directories in a
well-structured format. Whether you're building the next AI revolution or just tinkering with code on a rainy afternoon,
APC is here to help.

## Overview

APC is designed to streamline the process of gathering and structuring project data, making it easier for AI models to
understand and process your codebase. By respecting ignore rules and providing options for handling large or binary
files, APC ensures that your AI gets only the context it needs. And remember, a well-fed AI is a happy AI!

## Features

- **Directory Structure Generation**: automatically generates a structured view of your project's files and directories.
  No more manual digging through folders!
- **File Content Inclusion**: includes file contents for context, ensuring your AI has all the information it needs.
- **Ignore Rules Respect**: honors `.gitignore` and `.apcignore` files, so only the files you care about are included.
- **Configurable File Size Limits**: set your own limits for file sizes to include, because sometimes less is more.
- **Binary File Handling**: choose whether to include binary files with placeholder content. Perfect for those who like
  living on the edge.

## Installation

### From Binaries

For those who value speed and efficiency:

#### Linux/macOS

```bash
curl -fsSL https://raw.githubusercontent.com/fgbm/apc/refs/heads/master/scripts/install.sh | sh
```

#### Windows

```powershell
iwr -useb https://raw.githubusercontent.com/fgbm/apc/refs/heads/master/scripts/install.ps1 | iex
```

### From Releases

Prefer the scenic route? Download the latest binary for your platform from
the [Releases page](https://github.com/fgbm/apc/releases).

### From Source

For the purists who enjoy the journey as much as the destination:

```bash
cargo install --git https://github.com/fgbm/apc
```

## Usage

Using APC is as easy as pie (or cake, if that's your preference). Below are some common scenarios where APC can save the
day:

### Basic Usage

To generate a complete project context, simply provide the path to your project directory:

```bash
apc /path/to/your/project
```

### Save Output to a File

Want to save the output for later? Use the `-o` or `--output` option:

```bash
apc /path/to/your/project -o project_context.txt
```

### Specify Maximum File Size

Got a project with some hefty files? Set a maximum file size (in bytes) to include:

```bash
apc /path/to/your/project --max-file-size 204800
```

### Include Binary Files

Feeling adventurous? Include binary files with placeholder content:

```bash
apc /path/to/your/project --include-binary
```

### Structure-Only Mode

If you're only interested in the directory structure and not the file contents, try structure-only mode:

```bash
apc /path/to/your/project --structure-only
```

## .apcignore File

The `.apcignore` file is a configuration file used by the APC tool to specify files and directories that should be
excluded from the project context. This is similar to a `.gitignore` file but specifically tailored for APC's
functionality. By defining a `.apcignore` file in your project's root or any subdirectory, you can control which parts
of your codebase are included in the context generation process. This helps in reducing noise and focusing only on the
relevant files for AI consumption.

### Usage

To create a `.apcignore` file, simply add it to the root of your project or in any subdirectory where you want specific
ignore rules to apply. The syntax is the same as `.gitignore`, allowing you to specify patterns for files and
directories to be ignored.

### Example

```plaintext
# Ignore all log files
*.log

# Ignore all files in the tmp directory
/tmp/

# Specific file to ignore
secret_config.toml
```

### Predefined Ignores

By default, APC will automatically ignore the following directories and files:

- `.git/`
- `.idea/`
- `.vscode/`
- `.gitignore`
- `.apcignore`

These defaults ensure that common version control and IDE-specific files do not interfere with the project context.

## Example Scenarios

1. **Preparing for AI Training**: before feeding your project to an AI model, use APC to create a comprehensive context
   that highlights all the necessary files and their contents.

2. **Code Review and Auditing**: quickly generate a structured overview of a project's files and contents for code
   reviews or audits.

3. **Project Documentation**: use the output to help document the structure and key components of your project for team
   members or stakeholders.

## Contributing

Got ideas? Found a bug? Feel like adding a dash of humor to the code comments? We welcome contributions! Check out our
repository on [GitHub](https://github.com/fgbm/apc) and join the fun.

## License

APC is licensed under the MIT License. Share it, modify it, or just enjoy it as is — the choice is yours!
