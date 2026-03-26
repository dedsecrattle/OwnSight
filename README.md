# Ownsight - Rust Ownership Visualizer

[![Crates.io](https://img.shields.io/crates/v/ownsight-cli)](https://crates.io/crates/ownsight-cli)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)](LICENSE-MIT)
[![GitHub Release](https://img.shields.io/github/v/release/dedsecrattle/ownsight)](https://github.com/dedsecrattle/ownsight/releases/latest)
[![Documentation](https://img.shields.io/badge/docs-latest-blue)](https://dedsecrattle.github.io/ownsight/)

A dynamic, interactive tool for visualizing Rust ownership, borrowing, and lifetimes. Built with a focus on learning and teaching Rust's ownership system.

## 🚀 Quick Start

### CLI Tool

```bash
# Install from crates.io
cargo install ownsight-cli

# Analyze a Rust file
cargo ownership-viz --file example.rs

# Or analyze from stdin
echo 'fn main() { let s = String::from("hello"); }' | cargo ownership-viz --stdin
```

### Desktop App

Download the latest release for your platform:

- **macOS**: [Download .dmg](https://github.com/dedsecrattle/ownsight/releases/latest)
- **Linux**: [Download .AppImage](https://github.com/dedsecrattle/ownsight/releases/latest)
- **Windows**: [Download .msi](https://github.com/dedsecrattle/ownsight/releases/latest)

## 📚 [Documentation](https://dedsecrattle.github.io/ownsight/)

Complete documentation including:

- [Quick Start Guide](https://dedsecrattle.github.io/ownsight/quickstart.html)
- [Desktop App Guide](https://dedsecrattle.github.io/ownsight/usage/desktop.html)
- [CLI Tool Guide](https://dedsecrattle.github.io/ownsight/usage/cli.html)

## ✨ Features

### Layer 1 - Learning & Exam Tool (Stable)

- 🎯 **Interactive Timeline View**: Step through ownership events line by line
- 📊 **Visual Source Highlighting**: See ownership changes directly in your code
- 💡 **Teaching Mode**: Simplified explanations optimized for learning
- ⏯️ **Step Controller**: Play, pause, and navigate through ownership events
- 📝 **Event Explanations**: Human-readable descriptions of what happens to each variable
- ⚡ **Fast Analysis**: Syntax-based, no compilation required

### Query Interface

- 🔍 **Interactive Questions**: Ask "Why can't I use X here?", "Where was X moved?"
- 🐛 **Debug Mode**: Detailed analysis for debugging ownership issues
- � **Event Timeline**: Step through ownership changes line by line

## Architecture

```
ownsight/
├── crates/
│   ├── ownsight-core/      # Core data model (variables, events, graphs)
│   ├── ownsight-driver/    # Analysis driver (backend selection)
│   └── ownsight-cli/       # Command-line interface (cargo ownership-viz)
└── ui/                     # Tauri desktop app (React + TypeScript)
```

## 📦 Installation

### Option 1: Download Pre-Built Binaries (Recommended)

**Desktop App:**

Visit the [releases page](https://github.com/dedsecrattle/ownsight/releases) and download the installer for your platform:

- **macOS**: `Ownsight_x.x.x_universal.dmg` (Intel + Apple Silicon)
- **Linux**: `ownsight_x.x.x_amd64.AppImage` (Universal)
- **Windows**: `Ownsight_x.x.x_x64_en-US.msi` (Installer)

**CLI Tool:**

Download from [CLI releases](https://github.com/dedsecrattle/ownsight/releases) for your platform:

- **macOS**: `cargo-ownership-viz-macos-x64` or `cargo-ownership-viz-macos-arm64`
- **Linux**: `cargo-ownership-viz-linux-x64`
- **Windows**: `cargo-ownership-viz-windows-x64.exe`

All binaries include the **Simple backend** for educational ownership visualization.

### Option 2: Install from crates.io

```bash
cargo install ownsight-cli
```

### Option 4: Build from Source

**Prerequisites:**

- Rust (stable)
- Bun (for desktop app development)
- Platform-specific dependencies (see below)

**Clone and build:**

```bash
git clone https://github.com/dedsecrattle/ownsight.git
cd ownsight

# Build CLI tool
cargo build --release -p ownsight-cli

# Build desktop app
cd ui
bun install
bun run tauri build
```

**Platform-specific dependencies:**

- **Ubuntu/Debian:**

  ```bash
  sudo apt-get install libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf
  ```

- **macOS:** No additional dependencies (uses native WebKit)

- **Windows:** No additional dependencies

## 🎯 Usage

### CLI Tool

**Basic analysis:**

```bash
cargo ownership-viz --file example.rs
```

**Analyze from stdin:**

```bash
echo 'fn main() { let s = String::from("hello"); }' | cargo ownership-viz --stdin
```

**Output formats:**

```bash
# Timeline format (default)
cargo ownership-viz --file example.rs

# JSON format
cargo ownership-viz --file example.rs --output json
```

**Teaching vs Debug mode:**

```bash
# Teaching mode: simplified explanations for learning
cargo ownership-viz --file example.rs --mode teaching

# Debug mode: precise, technical analysis
cargo ownership-viz --file example.rs --mode debug
```

### Desktop App

Launch the installed application, then:

1. **Write or paste Rust code** in the Monaco editor
2. **Click "Analyze"** to process the code
3. **Use the timeline view** to step through ownership events
4. **Switch to graph view** to see ownership relationships
5. **Try the query panel** to ask questions about ownership

**Features:**

- 📝 **Monaco Code Editor**: Syntax highlighting and editing
- ⏯️ **Step Controller**: Play/pause with adjustable speed
- 📊 **Timeline View**: Event-by-event breakdown with icons
- 🎨 **Graph View**: Visual ownership relationships
- 🔍 **Query Panel**: Interactive debugging questions

**Development mode:**

```bash
cd ui
bun run tauri dev
```

- **Interactive Stepping**: Play/pause through events with speed control

## Example

```rust
fn main() {
    let s = String::from("hello");
    let r1 = &s;
    println!("{}", r1);
    let s2 = s;  // Move happens here
    // println!("{}", s);  // Error: s was moved
}
```

Ownsight will show:

1. Line 2: `s` created and owns the String
2. Line 3: `r1` borrows `s` immutably
3. Line 4: `r1` is used (borrow is valid)
4. Line 5: `s` is moved into `s2`
5. Query: "Why can't I use `s` at line 6?" → "`s` was moved at line 5"

## Data Model

The core analysis produces:

- **Variables**: Name, type, scope, mutability
- **Events**: Create, Move, Borrow, Drop, etc.
- **Ownership Graph**: Nodes (variables, references) and edges (owns, borrows, moves)
- **Diagnostics**: Compiler errors and suggestions
- **Ownership State**: Valid/moved status at each line

## Development

### Running Tests

```bash
cargo test --workspace
```

### Snapshot Tests

```bash
cargo insta test
cargo insta review
```

### Development Workflow

```bash
# Terminal 1: Watch Rust changes
cargo watch -x "build --workspace"

# Terminal 2: Run UI in dev mode
cd ui && bun run tauri dev
```

## 🗺️ Roadmap

### ✅ v0.1.0 (Released - Layer 1)

- [x] Core data model (variables, events, graphs)
- [x] Simple syntax-based analysis
- [x] CLI tool with multiple output formats
- [x] **Published to crates.io** 📦
- [x] Tauri desktop app with React UI
- [x] **Desktop releases for macOS/Linux/Windows** 🚀
- [x] Monaco code editor with syntax highlighting
- [x] Timeline view with step-by-step events
- [x] Interactive stepping with play/pause controls
- [x] Graph visualization with React Flow
- [x] Query interface for debugging questions

### ✅ v0.2.0 (Current)

- [x] Improved syntax-based analysis
- [x] Enhanced query interface
- [x] Better error messages
- [x] Performance optimizations
- [x] Comprehensive test suite

### Next

- [ ] Pattern matching flow analysis
- [ ] Cargo workspace support
- [ ] Cross-file analysis
- [ ] Performance optimizations
- [ ] Incremental analysis

### Future

- [ ] VS Code extension
- [ ] Rust Analyzer integration
- [ ] Web-based version (WASM)
- [ ] CI integration
- [ ] Team collaboration features

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

## 🙏 Acknowledgments

Built with:

- [Rust](https://www.rust-lang.org/) - Systems programming language
- [Tauri](https://tauri.app/) - Cross-platform desktop framework
- [React Flow](https://reactflow.dev/) - Interactive graph visualization
- [Monaco Editor](https://microsoft.github.io/monaco-editor/) - Code editor
- [Bun](https://bun.sh/) - Fast JavaScript runtime and package manager

## 📊 Project Stats

- **CLI Downloads**: [![Crates.io](https://img.shields.io/crates/d/ownsight-cli)](https://crates.io/crates/ownsight-cli)
- **Latest Version**: [![Crates.io](https://img.shields.io/crates/v/ownsight-cli)](https://crates.io/crates/ownsight-cli)
- **License**: MIT OR Apache-2.0
- **Repository**: [github.com/dedsecrattle/ownsight](https://github.com/dedsecrattle/ownsight)

## 📚 Documentation

- [Quick Start Guide](QUICKSTART.md) - Get started in 5 minutes
- [Installation Guide](INSTALLATION.md) - Detailed installation instructions
- [Architecture](ARCHITECTURE.md) - System design and internals
- [Publishing Guide](PUBLISHING.md) - How to release new versions
- [Changelog](CHANGELOG.md) - Version history and updates

## 💬 Community & Support

- **Issues**: [GitHub Issues](https://github.com/dedsecrattle/ownsight/issues)
- **Discussions**: [GitHub Discussions](https://github.com/dedsecrattle/ownsight/discussions)
- **Contributing**: See [CONTRIBUTING.md](CONTRIBUTING.md)

---

Made with ❤️ for the Rust community
