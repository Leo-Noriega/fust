# 🚀 Fust - Terminal Fuzzy Finder

A fast, ergonomic terminal-based fuzzy finder and file manager built in Rust with Vim-like navigation.

## ✨ Features

- **🔍 Deep Directory Navigation (`fud`)**: Search and navigate to any directory on your system with full path display
- **📄 Fuzzy File Navigation (`fuf`)**: Search and open files with your preferred editor
- **⌨️ Vim-like Interface**: Normal and Insert modes for efficient keyboard navigation
- **🏎️ Fast Performance**: Built in Rust with async file system operations
- **🎨 Modern TUI**: Beautiful terminal interface with real-time filtering
- **🔧 Native Shell Integration**: Like zoxide - generates shell-specific code for seamless `cd` integration

## 🏗️ Installation

### 1. Build from Source

```bash
git clone <your-repo>
cd fust
cargo build --release
```

### 2. Install Binary

```bash
# Copy to a directory in your PATH
sudo cp ./target/release/fust /usr/local/bin/

# Or add the target directory to your PATH
echo 'export PATH="$PATH:/path/to/fust/target/release"' >> ~/.bashrc
```

### 3. Shell Integration (Required for `cd` functionality)

```bash
# Generate integration for your shell and add to config
fust init zsh >> ~/.zshrc    # For Zsh
# OR
fust init bash >> ~/.bashrc  # For Bash
# OR  
fust init fish >> ~/.config/fish/config.fish  # For Fish

# Reload your shell
source ~/.zshrc  # or ~/.bashrc
```

## 🎯 Usage

### Commands

With shell integration installed:
- `fud` - Navigate to any directory (with proper `cd`)
- `fuf` - Find and open files
- `fd` - Short alias for `fud`
- `ff` - Short alias for `fuf`

Raw commands (without shell integration):
- `fust fud` - Directory navigation (prints path only)
- `fust fuf` - File navigation
- `fust init <shell>` - Generate shell integration
- `fust install` - Show installation instructions

### Interface Modes

#### Normal Mode (Default)
- `j/k` - Navigate up/down
- `i` - Enter Insert mode
- `/` - Enter Insert mode (search)
- `c` - Clear filter
- `g` - Go to top
- `G` - Go to bottom
- `Enter` - Select item
- `q/Esc` - Quit

#### Insert Mode
- Type to filter results (searches in full paths)
- `↑/↓` - Navigate up/down
- `Enter` - Select item
- `Esc` - Return to Normal mode

## 🎨 Interface

```
┌─ [NORMAL] Filter ──────────────────────┐
│ clase                                  │
└────────────────────────────────────────┘
┌─ 🔍 Fuzzy Directory Navigation ────────┐
│ >> 📁 ~/Projects/clase                 │
│    📁 ~/Documents/clase-notes          │
│    📁 ~/Downloads/clases               │
│    📁 ~/Projects/react-clase           │
└────────────────────────────────────────┘
┌─ Help ─────────────────────────────────┐
│ NORMAL: j/k=move, i=insert mode...     │
└────────────────────────────────────────┘
```

## ⚡ Examples

### Directory Navigation
```bash
# Type 'fud' to open directory finder
fud

# Type "clase" to filter directories like:
# ~/Projects/clase
# ~/Documents/clase-notes  
# ~/Downloads/clases

# Navigate with j/k, Enter to cd to selected directory
```

### File Navigation  
```bash
# Type 'fuf' to open file finder
fuf

# Filter files and select to open with your editor
```

## ⚙️ Configuration

Fust uses your environment variables:
- `$VISUAL` or `$EDITOR` - Preferred text editor
- Falls back to: `nvim`, `vim`, `nano`, `code`

Built-in ignored directories:
- `.git`, `node_modules`, `target`, `.DS_Store`
- `/Library/` (on macOS)
- `/.Trash`, `/__pycache__/`

## 🛠️ Development

```bash
# Run in development
cargo run -- fud
cargo run -- fuf
cargo run -- init zsh

# Run tests
cargo test

# Format code
cargo fmt

# Lint
cargo clippy
```

## 📝 License

MIT License 