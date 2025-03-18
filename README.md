# APC - AI Project Context

A command-line utility that prepares project code context for AI consumption.

## Features

- Generates a structured view of your project's files and directories
- Includes file contents for context
- Respects `.gitignore` and `.apcignore` files
- Configurable file size limits and binary file handling

## Installation

### From Binaries

#### Linux/macOS

```bash
curl -fsSL https://raw.githubusercontent.com/fgbm/apc/refs/heads/master/scripts/install.sh | sh
```

#### Windows

```powershell
iwr -useb https://raw.githubusercontent.com/fgbm/apc/refs/heads/master/scripts/install.ps1 | iex
```

### From Releases

Download the latest binary for your platform from the [Releases page](https://github.com/fgbm/apc/releases).

### From Source

```bash
cargo install --git https://github.com/fgbm/apc
```