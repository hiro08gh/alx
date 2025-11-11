# alx

<img width="355" height="172" alt="スクリーンショット 0007-11-11 18 07 14" src="https://github.com/user-attachments/assets/58245b9a-406b-4847-8b21-9c3530ef68c4" />

A simple alias manager for multiple shells written in Rust.

## Features

- 🚀 Manage aliases across multiple shells (Bash, Zsh, Fish)
- 📦 Group aliases by category
- 🔍 Search aliases by keyword
- 💾 Import / Export aliases (JSON, TOML)
- 🔄 Automatic sync to shell configuration

## Installation

using cargo install:

```bash
cargo install alx
```

## Quick Start

### 1. Initialize

```bash
alx init
```

### 2. Add aliases

```bash
# Simple alias
alx add ll "ls -la"

# With description
alx add gs "git status" --description "Show git status"

# With group
alx add gp "git push" --group git
```

### 3. Add to your shell config

First, your running `alx info` command. Please check `Shell aliases` path.

Add the following line to your shell configuration file:

**Bash** (`~/.bashrc`):

```bash
[ -f ~/shell_aliases_path/aliases.sh ] && source ~/shell_aliases_path/aliases.sh
```

**Zsh** (`~/.zshrc`):

```bash
[ -f ~/shell_aliases_path/aliases.sh ] && source ~/shell_aliases_path/aliases.sh
```

**Fish** (`~/.config/fish/config.fish`):

```fish
source ~/shell_aliases_path/aliases.sh
```

## Usage

### Add an alias

```bash
alx add <name> <command> [--description] [--group]
```

Example:

```bash
alx add ll "ls -la" --description "List all files" --group general
alx add gs "git status" --group git
```

### List aliases

```bash
# List all aliases
alx list

# List aliases in a specific group
alx list --group git
```

### Search aliases

```bash
alx search git
```

### Edit an alias

```bash
alx edit <name> [--command] [--description] [--group]
```

Example:

```bash
alx edit ll --command "ls -lah"
alx edit gs --description "Check git status"
```

### Remove an alias

```bash
alx remove <name>
```

### Export/Import

```bash
# Export to JSON
alx export --output my-aliases.json --format json

# Export to TOML
alx export --output my-aliases.toml --format toml

# Import from file
alx import my-aliases.json
```

### View groups

```bash
alx groups
```

### Show info

```bash
alx info
```

## Configuration

Structure:

```
~/.config/alx/
├── config.toml       # Main configuration
├── aliases.toml      # Aliases database
├── shell/
│   └── aliases.sh    # Generated shell aliases
└── backups/          # Backup directory
```

## Example Workflow

```bash
# Initialize
alx init

# Add some git aliases
alx add gs "git status" --group git
alx add ga "git add" --group git
alx add gc "git commit" --group git
alx add gp "git push" --group git

# Add some system aliases
alx add ll "ls -la" --group system
alx add c "clear" --group system

# List all aliases
alx list

# Search for git-related aliases
alx search git

# Export for backup
alx export --output ~/my-aliases.json
```

## Development

### Build

```bash
cargo build
```

### Run tests

```bash
cargo test
```

### Run

```bash
cargo run -- <command>
```

## License

MIT License.© [hiro08gh](https://github.com/hiro08gh)
