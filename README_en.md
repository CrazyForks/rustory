# Rustory

**[English](README_en.md)** | [简体中文](README.md)

> 🚀 **Lightweight Local Version Control Tool** - A high-performance version control system written in Rust

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![Platform](https://img.shields.io/badge/platform-Linux%20%7C%20macOS%20%7C%20Windows-lightgrey.svg)](https://github.com/uselibrary/rustory)

## ✨ Project Overview

Rustory (**Rust** + His**tory**) is a Rust-based version control tool designed to help developers easily manage project snapshots, history, and configuration. It provides Git-like features but focuses on simplicity and ease of use. Rustory is a lightweight local version control tool supporting Linux, macOS, and Windows, with no external command or script interpreter dependencies. It tracks, snapshots, and restores project file changes efficiently.

> **Note**: Rustory is a local version management tool, **not a replacement for Git**. It is mainly for personal developers and simple projects, and does not support distributed development, remote repositories, or advanced features like branch merging. For team collaboration or complex workflows, please use Git.

### 🎯 Design Goals
- **Local-first**: Designed for individual developers and script authors, no distributed collaboration
- **Lightweight & Efficient**: Pure Rust, no external dependencies, fast startup
- **Simple CLI**: Intuitive command-line interface, easy to get started
- **Storage Optimization**: Content deduplication + compression to save disk space

### 🏗️ Core Features
- ✅ **Snapshot Management**: Quickly create and restore project snapshots
- ✅ **Numbering System**: Assigns a number to each snapshot for easy reference
- ✅ **Snapshot Deletion**: Precisely delete single or ranged snapshots
- ✅ **Diff Comparison**: Smart file diff detection and display
- ✅ **Tag System**: Add descriptive tags to important snapshots
- ✅ **Ignore Rules**: Git-style file ignore patterns
- ✅ **Garbage Collection**: Auto-cleanup of expired data, storage optimization
- ✅ **Integrity Verification**: Data integrity check and repair
- ✅ **Rich Statistics**: Detailed repository usage statistics

## 📦 Installation Guide

### Method 1: Download Precompiled Binary (Recommended)

Download the precompiled binary for your system from [GitHub Releases](https://github.com/uselibrary/rustory/releases):

#### Supported Platforms
- **Windows**: x64, ARM64
- **macOS**: x64 (Intel), ARM64 (Apple Silicon)
- **Linux**: x64, ARM64 (Built with `musl` are recommended for better compatibility across distributions)

#### Download & Verify
1. Visit the [latest release page](https://github.com/uselibrary/rustory/releases/latest)
2. Choose the file for your system:
   - Windows: `rustory-x86_64-pc-windows-msvc.zip` or `rustory-aarch64-pc-windows-msvc.zip`
   - macOS: `rustory-x86_64-apple-darwin.tar.gz` or `rustory-aarch64-apple-darwin.tar.gz`
   - Linux: `rustory-x86_64-unknown-linux-musl.tar.gz` or `rustory-aarch64-unknown-linux-musl.tar.gz`

3. **Verify file integrity** (recommended):
   ```bash
   # macOS/Linux
   shasum -a 256 rustory-*.tar.gz
   # Windows (PowerShell)
   Get-FileHash -Algorithm SHA256 rustory-*.zip
   ```
   Compare with the SHA256 value on the release page.

4. **Extract and install**:
   ```bash
   # Linux/macOS
   tar -xzf rustory-*.tar.gz
   sudo mv rustory /usr/local/bin/
   # Windows: Extract ZIP and move rustory.exe to a directory in PATH
   ```

### Method 2: One-Click Install Script (Linux/macOS)

For Linux and macOS users, we provide a convenient one-click installation script:

#### Quick Install
```bash
# Install or update Rustory
curl -fsSL https://raw.githubusercontent.com/uselibrary/rustory/refs/heads/master/install.sh | sudo bash
```

#### Manual Download and Install
```bash
# Download the script
curl -fsSL https://raw.githubusercontent.com/uselibrary/rustory/refs/heads/master/install.sh -o install.sh
# Make it executable and run
chmod +x install.sh
sudo ./install.sh install
```

#### Script Features
- ✅ **Auto-detection**: Automatically detects OS and architecture
- ✅ **Version management**: Checks for updates and installs the latest version
- ✅ **Dependency check**: Verifies required tools (curl, tar, jq)
- ✅ **Safe installation**: Downloads from official GitHub releases
- ✅ **Uninstall support**: Easy removal with `sudo ./install.sh uninstall`

#### Usage Options
```bash
# Install or update
sudo ./install.sh install      # or just: sudo ./install.sh

# Uninstall
sudo ./install.sh uninstall

# Show help
./install.sh --help

# Show script version
./install.sh --version
```

#### Supported Systems
- **Linux**: x86_64, aarch64 (with musl builds for better compatibility)
- **macOS**: x86_64 (Intel), aarch64 (Apple Silicon)

### Method 3: Build from Source

#### Prerequisites
- **Rust version**: 1.70 or above
- **OS**: Linux, macOS, or Windows

#### Build Steps

1. **Ensure Rust is installed**
   ```bash
   # Install Rust (if not already installed)
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Clone and build the project**
   ```bash
   git clone https://github.com/uselibrary/rustory.git
   cd rustory
   cargo build --release
   ```

3. **Install to system path (optional)**
   ```bash
   # Linux/macOS
   sudo cp target/release/rustory /usr/local/bin/
   # Windows
   copy target\release\rustory.exe C:\Windows\System32\
   ```

4. **Verify installation**
   ```bash
   rustory --version
   ```

## 🏛️ System Architecture

### Storage Structure
Rustory creates a `.rustory` folder in the working directory, containing:

```
.rustory/
├── config.toml           # User config: ignore rules, output format, backup policy, etc.
├── ignore                # Ignore rules file (Git style)
├── objects/              # Content stored by SHA-1 hash
│   ├── ab/               # Hash prefix as subdirectory
│   │   └── cdef123...    # Compressed file content
│   └── ...
├── index.json            # File path to hash mapping
├── history.log           # Snapshot log: ID, time, stats, message
└── snapshots/            # Snapshot metadata JSON files
    ├── abc123.json
    └── ...
```

### Core Concepts

1. **Object Storage**: File contents are stored as binary objects, named by SHA-1 hash, for deduplication
2. **Index Management**: Maps workspace file paths to hashes for fast change detection
3. **Snapshot System**: Saves index state, metadata in `snapshots/`, and logs in `history.log`
4. **Compression**: Uses gzip to reduce storage space

### Storage Optimization
- **Deduplication**: Identical content stored only once
- **Compression**: All objects are gzip-compressed
- **Directory Sharding**: Hash prefix avoids too many files in one directory
- **Large File Limit**: Configurable file size limit (default 100MB)

## 🚀 Quick Start

### Initialize Project
```bash
# Initialize in current directory
rustory init
# Initialize in specified directory
rustory init /path/to/project
```

### Basic Workflow
```bash
# 1. Check status
rustory status
# 2. Create snapshot
rustory add -m "Initial version"
# 3. View history
rustory history
# 4. Compare differences
rustory diff
# 5. Rollback changes
rustory back abc123
# 6. Delete unwanted snapshots
rustory rm 3                    # Delete snapshot #3
rustory rm 1-5                  # Delete snapshots 1 to 5
# 7. Clean up storage
rustory rm --aggressive         # Garbage collection
```

## 📋 Command Reference

### Core Commands

#### `rustory init` - Initialize Repository
```bash
rustory init [path]
```
- **Function**: Create a new Rustory repository
- **Argument**: `[path]` - optional, target path (default: current directory)
- **Effect**: Creates `.rustory` structure and default config

#### `rustory add` - Create Snapshot
```bash
rustory add -m "message" [--json]
```
- **Function**: Save current workspace as a new snapshot
- **Alias**: `commit` (for compatibility)
- **Arguments**:
  - `-m, --message <MSG>` - Snapshot description
  - `--json` - Output in JSON format
- **Example**:
  ```bash
  rustory add -m "Fix parser bug"
  # Output: [snapshot ab12cd] 2025-06-18T15:30:00  added=2 modified=1 deleted=0
  # Using old alias (compatibility)
  rustory commit -m "Fix parser bug"
  ```

#### `rustory status` - Show Status
```bash
rustory status [--verbose] [--json]
```
- **Function**: Show changes relative to latest snapshot
- **Arguments**:
  - `--verbose` - Show details (size, mtime)
  - `--json` - Output in JSON format
- **Example Output**:
  ```
  Modified: src/lib.rs (1.2KB)
  Added: tests/test_api.rs (0.8KB)
  Deleted: docs/old.md
  ```

#### `rustory history` - View History
```bash
rustory history [--json]
```
- **Function**: Show all snapshot history with numbers
- **Example Output**:
  ```
  #   ID       Time                     +  ~  -  Message
  ─────────────────────────────────────────────────────────────
  3   ab12cd   2025-06-18T15:30:00      2  1  0  "Fix parser bug"
  2   ef34gh   2025-06-17T10:15:30      5  0  2  "Add new feature"
  1   xy56ij   2025-06-16T09:45:00      0  0  0  "Initial commit"
  ```
- **Fields**:
  - `#` - Snapshot number (for rm, etc.)
  - `ID` - Snapshot hash ID
  - `+/-/~` - Added/Deleted/Modified file count

#### `rustory diff` - Compare Differences
```bash
rustory diff [snapshot1] [snapshot2]
```
- **Function**: Show file differences
- **Arguments**: Accepts snapshot number or ID
- **Usage**:
  - No argument: current state vs latest snapshot
  - One argument: specified snapshot vs current state
  - Two arguments: compare two snapshots
- **Example**:
  ```bash
  rustory diff                    # Current vs latest snapshot
  rustory diff 3                  # Snapshot #3 vs current
  rustory diff 1 3                # Snapshot #1 vs #3
  rustory diff abc123 def456      # Compare two IDs
  ```
- **Output**: Colored line-level diff

#### `rustory back` - Rollback Changes
```bash
rustory back <snapshot_number|snapshot_id> [--restore] [--keep-index]
```
- **Function**: Restore to specified snapshot
- **Alias**: `rollback` (compatibility)
- **Arguments**:
  - `<snapshot_number>` - Snapshot number (e.g. 1, 2, 3)
  - `<snapshot_id>` - Snapshot ID or tag (e.g. abc123, v1.0)
  - `--restore` - Restore directly to workspace (backup current state first)
  - `--keep-index` - Do not update index file
- **Safety**: By default, exports to `backup-<timestamp>/` directory
- **Example**:
  ```bash
  rustory back 3                  # Rollback to snapshot #3
  rustory back abc123             # Rollback to specified ID
  # Using old alias (compatibility)
  rustory rollback abc123
  ```

### Management Commands

#### `rustory tag` - Tag Management
```bash
rustory tag <tag_name> <snapshot_number|snapshot_id>
```
- **Function**: Add descriptive tag to snapshot
- **Arguments**: Accepts snapshot number or ID
- **Example**:
  ```bash
  rustory tag v1.0 3              # Tag snapshot #3
  rustory tag v1.0 ab12cd         # Tag by ID
  rustory back v1.0               # Rollback by tag
  ```

#### `rustory ignore` - Ignore Rules
```bash
rustory ignore [show|edit]
```
- **Function**: Manage ignore rules
- **Rule Format**: Git-style glob patterns
- **Example Rules**:
  ```
  *.log
temp/
node_modules/
target/
  ```

#### `rustory config` - Config Management
```bash
rustory config get <key>           # Get config
rustory config set <key> <value>   # Set config
```
- **Common Configs**:
  - `output_format`: Output format (table/json)
  - `max_file_size_mb`: File size limit (default 100MB)
  - `gc_keep_days`: GC keep days (default 30)
  - `gc_keep_snapshots`: GC keep snapshot count (default 50)
  - `gc_auto_enabled`: Auto GC (default false)

### Utility Commands

#### `rustory rm` - Delete Snapshots & GC
```bash
# Delete specific snapshot
rustory rm <number>              # By number
rustory rm <snapshot_id>         # By ID
rustory rm <start>-<end>         # By range
rustory rm <id1>-<id2>           # By ID range
# GC mode (compatible with gc command)
rustory rm [--dry-run] [--aggressive] [--prune-expired]
```
- **Function**: Delete snapshots or run garbage collection
- **Alias**: `gc` (compatibility)
- **Delete Mode Args**:
  - `<number>` - Snapshot number (e.g. 1, 5, 10)
  - `<snapshot_id>` - Snapshot ID (e.g. abc123ef)
  - `<range>` - Range (e.g. 1-5, abc123-def456)
- **GC Args**:
  - `--dry-run`: Preview mode
  - `--aggressive`: More aggressive cleanup
  - `--prune-expired`: Remove expired snapshots
- **Example**:
  ```bash
  rustory rm 3                    # Delete snapshot #3
  rustory rm abc123ef             # Delete by ID
  rustory rm 1-5                  # Delete snapshots 1-5
  rustory rm abc123-def456        # Delete by ID range
  rustory rm --dry-run            # Preview cleanup
  rustory gc --aggressive         # Old alias
  ```
- **Safety**: Deletion is irreversible, backup important data first

#### `rustory stats` - Statistics
```bash
rustory stats [--json]
```
- **Function**: Show detailed repo stats
- **Includes**:
  - Repo size & compression ratio
  - File type distribution
  - Snapshot/object count
  - Storage usage

#### `rustory verify` - Integrity Check
```bash
rustory verify [--fix]
```
- **Function**: Verify repo data integrity
- **Checks**:
  - Snapshot file format
  - Object file readability
  - Index consistency
- **Arg**: `--fix` - Try to auto-fix issues

## 🔧 Advanced Features

### Snapshot Deletion & GC Strategy

Rustory provides flexible snapshot management and GC:

```bash
# Configure retention
rustory config set gc_keep_days 14      # Keep snapshots from last 14 days
rustory config set gc_keep_snapshots 20 # Keep max 20 snapshots
# Enable auto GC
rustory config set gc_auto_enabled true
# Manually delete snapshots
rustory rm 5                    # Delete snapshot #5
rustory rm abc123               # Delete by ID
rustory rm 3-8                  # Delete snapshots 3-8
rustory rm abc123-def456        # Delete by ID range
# GC and cleanup
rustory rm --dry-run            # Preview
rustory rm --aggressive         # Deep clean
rustory rm --prune-expired      # Remove expired
# Old alias
rustory gc --dry-run            # Same as rustory rm --dry-run
rustory gc                      # Same as rustory rm
```

### Batch Operations

```bash
# Batch commit changes
find . -name "*.rs" -newer .rustory/index.json | rustory add -m "Batch update"
# Batch delete snapshots
rustory rm 1-10                 # Delete first 10 snapshots
rustory rm --prune-expired      # Remove expired
```

### Config Optimization

```bash
# Performance tuning
rustory config set max_file_size_mb 50          # Limit large files
rustory config set compression_level 6          # Set compression level
rustory config set parallel_threads 4           # Set parallel threads
# Output format
rustory config set output_format json           # Default JSON output
rustory config set colored_output true          # Colored output
```

## 🔍 Troubleshooting

### Common Issues

#### Snapshot Creation Fails
```bash
# Check disk space
df -h .
# Check file permissions
ls -la .rustory/
# Verify ignore rules
rustory ignore show
# Check large files
rustory status --verbose | grep "large"
```

#### Rollback Conflicts
```bash
# Save current work before rollback
rustory add -m "Temp save"
rustory back <target_snapshot>
# Or use backup mode
rustory back <target_snapshot> --restore
```

#### Storage Issues
```bash
# Check repo stats
rustory stats
# Delete unwanted snapshots
rustory rm 1-5                  # Delete first 5 snapshots
rustory rm old_snapshot_id      # Delete by ID
# Run GC
rustory rm --dry-run            # Preview
rustory rm --prune-expired      # Remove expired
# Clean large file history
rustory config set max_file_size_mb 10
rustory rm --aggressive         # Deep clean
# Use old alias
rustory gc --aggressive         # Same as rustory rm --aggressive
```

### Data Recovery

If you encounter data corruption:

```bash
# Verify repo integrity
rustory verify
# Try auto-fix
rustory verify --fix
# Manual recovery (last resort)
cp .rustory/snapshots/*.json backup/
rustory init --force
```

## 🚀 Performance Optimization

### Storage Optimization Tips

1. **Regular snapshot cleanup and GC**
   ```bash
   # Set auto cleanup
   rustory config set gc_auto_enabled true
   rustory config set gc_keep_days 30
   # Manual cleanup
   rustory rm 1-10                 # Delete first 10 snapshots
   rustory rm --prune-expired      # Remove expired
   ```
2. **File size limit**
   ```bash
   # Avoid tracking large files
   rustory config set max_file_size_mb 50
   ```
3. **Ignore rule optimization**
   ```bash
   # Exclude build artifacts and temp files
   echo "target/" >> .rustory/ignore
   echo "*.tmp" >> .rustory/ignore
   echo "node_modules/" >> .rustory/ignore
   ```

### Performance Monitoring

```bash
# Check operation time
time rustory add -m "Performance test"
# Monitor storage usage
rustory stats | grep "Size"
# Check compression efficiency
rustory stats | grep "Compression"
```

## 🛠️ Integration & Extension

### Editor Integration

#### VS Code
```json
// settings.json
{
  "rustory.autoAdd": true,
  "rustory.addInterval": 3600,
  "rustory.showStatus": true
}
```

#### Vim
```vim
" .vimrc
autocmd BufWritePost * silent! !rustory add -m "Auto save"
```

### CI/CD Integration

#### GitHub Actions
```yaml
name: Rustory Snapshot
on: [push]
jobs:
  snapshot:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Create snapshot
        run: |
          rustory init
          rustory add -m "CI Build ${{ github.run_number }}"
```

#### Shell Script Integration
```bash
#!/bin/bash
# Automated deployment script
set -e

echo "Creating pre-deploy snapshot..."
rustory add -m "Pre-deploy snapshot $(date)"

echo "Deploying..."
./deploy.sh

echo "Creating post-deploy snapshot..."
rustory add -m "Post-deploy snapshot $(date)"
```

## 🎯 Comparison with Other Tools

### Rustory vs Git - Key Differences

**Rustory is not a replacement for Git**; they have fundamentally different goals:

#### 🎯 Philosophy
- **Rustory**: Focuses on **local snapshot management** for individuals, simple and intuitive
- **Git**: Designed as a **distributed VCS** for complex team collaboration

#### 🔧 Feature Comparison

| Feature         | Rustory | Git | Use Case                  |
|----------------|---------|-----|---------------------------|
| Local Snapshots| ✅ Opt. | ✅   | Personal versioning       |
| Startup Speed  | ✅ Fast | ⚠️ Slow | Frequent small changes    |
| Learning Curve | ✅ Low  | ❌ High | VCS beginners            |
| Storage Eff.   | ✅ High | ✅   | Disk space sensitive      |
| Binary Files   | ✅ Native| ⚠️ Limited | Media/data versioning   |
| Remote Repo    | ❌      | ✅   | Team collaboration        |
| Branching      | ❌      | ✅   | Complex feature dev       |
| Merge Conflicts| ❌      | ✅   | Multi-dev workflows       |
| Commit History | ✅ Linear| ✅ DAG | Project history          |
| Tag System     | ✅ Simple| ✅ Signed | Release management     |

#### 🚀 Rustory Best Use Cases
- **Personal projects**: Scripts, configs, docs
- **Rapid prototyping**: Experimental code
- **Learning**: VCS concepts and practice
- **Lightweight needs**: Small projects
- **Binary files**: Images, videos, data

#### 💼 Git Best Use Cases
- **Team collaboration**: Multi-dev projects
- **Open source**: Community contributions
- **Enterprise**: Complex branching/release
- **CI/CD**: Deep platform integration
- **Code review**: PR/MR workflows

### Usage Suggestions

#### 🎯 When to Use Rustory
```bash
# Personal script versioning
cd ~/scripts
rustory init
rustory add -m "Add backup script"
# Config file snapshots
cd ~/.config
rustory init
rustory add -m "System config baseline"
# Rapid prototyping
cd ~/experiments/ml-model
rustory init
rustory add -m "Initial model version"
```

#### 🎯 When to Use Git
```bash
# Team project development
git clone https://github.com/team/project.git
git checkout -b feature/new-api
git commit -m "Implement new API"
git push origin feature/new-api
# Open source contribution
git fork https://github.com/opensource/project.git
git commit -m "Fix memory leak"
git pull-request
```

#### 🔄 Both Can Coexist
In some cases, you can use both tools in the same project:
```bash
# Use Git for main version control
git commit -m "Finish new feature"
# Use Rustory for frequent local snapshots
rustory add -m "Temp save: debugging"
```

### Migration Guide

#### Migrate from Git to Rustory
For projects no longer needing remote collaboration:
```bash
# Export Git history as snapshots
git log --oneline | while read commit; do
    git checkout $commit
    rustory add -m "Migrate: $commit"
done
```

#### Migrate from Rustory to Git
When you need to expand to team collaboration:
```bash
# Initialize Git repo
git init
# Create initial commit from Rustory snapshot
rustory back <latest_snapshot> --restore
git add .
git commit -m "Initial version from Rustory"
```

| Tool      | Best Use Case         | Learning | Performance | Collaboration |
|-----------|----------------------|----------|-------------|---------------|
| Rustory   | Personal, prototyping| ⭐⭐      | ⭐⭐⭐⭐⭐      | ❌            |
| Git       | Team, open source    | ⭐⭐⭐⭐    | ⭐⭐⭐⭐       | ⭐⭐⭐⭐⭐        |
| SVN       | Enterprise, central  | ⭐⭐⭐     | ⭐⭐⭐        | ⭐⭐⭐          |

## 📈 Roadmap

### Current Version (v0.1.5)
- ✅ Core version control
- ✅ Basic storage optimization
- ✅ GC mechanism
- ✅ Config system

### Next Version (v0.2.0)
- 🚧 Parallel processing
- 🚧 Incremental backup

### Future
- 📋 Sync support
- 📋 API
- 📋 Plugin system

### Development Environment
```bash
# Clone repo
git clone https://github.com/uselibrary/rustory.git
cd rustory
# Install dev dependencies
cargo install cargo-watch
cargo install cargo-tarpaulin
# Run tests
cargo test
# Format code
cargo fmt
# Lint
cargo clippy
```

### Code Style
- Use `rustfmt` for formatting
- Use `clippy` for linting
- Add tests for new features
- Update docs accordingly

## 📄 License

This project is licensed under the [GNU General Public License v3.0](LICENSE).

## 🙏 Acknowledgements

Thanks to these great Rust crates:
- [clap](https://crates.io/crates/clap) - CLI argument parsing
- [serde](https://crates.io/crates/serde) - Serialization/deserialization
- [walkdir](https://crates.io/crates/walkdir) - Directory traversal
- [flate2](https://crates.io/crates/flate2) - Compression
- [sha1](https://crates.io/crates/sha1) - Hashing
- [chrono](https://crates.io/crates/chrono) - Time handling
- [colored](https://crates.io/crates/colored) - Colored output

---

<div align="center">

**[⬆ Back to Top](#rustory)**

Made with ❤️ by the Rustory Team
