# Ownsight - Rust Ownership Visualizer

<div align="center">
  <img src="https://raw.githubusercontent.com/dedsecrattle/ownsight/main/ui/src-tauri/icons/icon.png" alt="Ownsight Logo" width="120">
  
  **Interactive visualization tool for understanding Rust ownership and borrowing**
  
  [![GitHub release](https://img.shields.io/github/release/dedsecrattle/ownsight.svg)](https://github.com/dedsecrattle/ownsight/releases)
  [![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](https://github.com/dedsecrattle/ownsight/blob/main/LICENSE)
</div>

## Overview

Ownsight helps you understand Rust's ownership system through interactive visualizations. It shows how ownership moves, borrows are created, and lifetimes work in your code.

### Two Analysis Backends

1. **Simple Backend** - Fast syntax-based analysis (default)
   - Perfect for learning
   - Instant results
   - No compilation needed

2. **MIR Backend (Layer 2)** - Compiler-accurate analysis
   - Uses Rust's MIR (Mid-level IR)
   - Detects partial moves, closure captures, async/await
   - Requires nightly Rust

## Quick Start

### Desktop App (Recommended)

1. [Download the latest release](https://github.com/dedsecrattle/ownsight/releases)
2. Install and run the app
3. Write or paste Rust code
4. Click "Analyze" to see the ownership timeline

### CLI Tool

```bash
# Install
cargo install ownsight-cli

# Analyze a file
cargo ownership-viz --file example.rs

# Analyze from stdin
echo "fn main() { let s = String::from(\"hello\"); drop(s); }" | cargo ownership-viz --stdin
```

## Features

- 🎯 **Interactive Timeline** - Step through ownership events
- 📊 **Graph Visualization** - See ownership relationships
- 🔍 **Query Interface** - Ask "why can't I use this variable?"
- 🏫 **Teaching Mode** - Explanations for beginners
- 🐛 **Debug Mode** - Detailed information for developers
- 🚀 **Layer 2 Analysis** - Advanced MIR-based features

## Example

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;  // s1 is moved to s2
    println!("{}", s2);  // OK
    // println!("{}", s1);  // Error: s1 has been moved
}
```

Ownsight will show:
1. `s1` is created
2. `s1` is moved to `s2`
3. `s2` is used
4. `s2` is dropped

## Screenshots

<div align="center">
  <img src="https://raw.githubusercontent.com/dedsecrattle/ownsight/main/screenshots/timeline-view.png" alt="Timeline View" width="600">
</div>

## Documentation

- [Quick Start Guide](quickstart.md)
- [Layer 2: MIR Backend](usage/layer2.md)
- [Desktop App Guide](usage/desktop.md)
- [CLI Tool Guide](usage/cli.md)
- [Examples](examples.md)

## Contributing

We welcome contributions! See [Contributing Guide](contributing.md) for details.

### Quick Development Setup

```bash
git clone https://github.com/dedsecrattle/ownsight
cd ownsight

# Desktop app
cd ui
bun install
bun run tauri dev

# CLI tool
cargo run --bin ownsight-cli -- --file examples/hello.rs
```

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.

## Acknowledgments

- Built with [Tauri](https://tauri.app/) for the desktop app
- Uses [Monaco Editor](https://microsoft.github.io/monaco-editor/) for code editing
- [React Flow](https://reactflow.dev/) for graph visualization
