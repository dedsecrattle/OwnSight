# Ownsight - Rust Ownership Visualizer

A dynamic, interactive tool for visualizing Rust ownership, borrowing, and lifetimes. Built with a two-layer architecture: Layer 1 for exam/learning use cases, Layer 2 for real debugging and code intelligence.

## Features

### Layer 1 - Learning & Exam Tool
- ✨ **Interactive Timeline View**: Step through ownership events line by line
- 📊 **Visual Source Highlighting**: See ownership changes directly in your code
- 🎯 **Teaching Mode**: Simplified explanations optimized for learning
- 🔄 **Step Controller**: Play, pause, and navigate through ownership events
- 📝 **Event Explanations**: Human-readable descriptions of what happens to each variable

### Layer 2 - Debugging & Intelligence
- 🔍 **Query Interface**: Ask "Why can't I use X here?", "Where was X moved?", "What is borrowing Y?"
- 📈 **Ownership Graph**: Visualize relationships between variables, references, and scopes
- 🐛 **Debug Mode**: Precise, compiler-backed analysis
- 🎨 **Graph Visualization**: Interactive graph showing ownership relationships

## Architecture

```
ownsight/
├── crates/
│   ├── ownsight-core/      # Core analysis engine (data model, events, graphs)
│   ├── ownsight-driver/    # Rust compiler integration (MIR-based analysis)
│   └── ownsight-cli/       # Command-line interface (cargo ownership-viz)
└── ui/                     # Tauri desktop app (React + TypeScript)
```

## Installation

### Prerequisites
- Rust (stable + nightly)
- Bun (for UI development)
- Tauri CLI

### Build from Source

1. Clone the repository:
```bash
git clone https://github.com/yourusername/ownsight.git
cd ownsight
```

2. Build the Rust workspace:
```bash
cargo build --release
```

3. Install the CLI tool:
```bash
cargo install --path crates/ownsight-cli
```

4. Build the desktop app:
```bash
cd ui
bun install
bun run tauri build
```

## Usage

### CLI Tool

Analyze a Rust snippet:
```bash
cargo ownership-viz --file example.rs
```

Analyze from stdin:
```bash
echo 'fn main() { let s = String::from("hello"); }' | cargo ownership-viz --stdin
```

Output as JSON:
```bash
cargo ownership-viz --file example.rs --output json
```

Teaching mode vs Debug mode:
```bash
cargo ownership-viz --file example.rs --mode teaching
cargo ownership-viz --file example.rs --mode debug
```

### Desktop App

Run in development mode:
```bash
cd ui
bun run tauri dev
```

The desktop app provides:
- **Source Editor**: Write or paste Rust code
- **Timeline View**: Step-by-step ownership events
- **Graph View**: Visual ownership relationships
- **Query Panel**: Ask questions about ownership
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

## Roadmap

### Current (MVP - Layer 1)
- [x] Core data model
- [x] Simple syntax-based analysis
- [x] CLI tool with timeline output
- [x] Tauri desktop app
- [x] Source view with highlighting
- [x] Timeline view
- [x] Interactive stepping
- [x] Graph visualization
- [x] Query interface

### Next (Layer 2 Enhancements)
- [ ] MIR-based analysis using rustc internals
- [ ] Cargo workspace support
- [ ] Function summaries
- [ ] Partial moves (struct fields)
- [ ] Pattern matching flows
- [ ] Non-lexical lifetimes (NLL)
- [ ] Closure capture analysis
- [ ] Async/await support

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

## Acknowledgments

Built with:
- Rust compiler internals for accurate analysis
- Tauri for cross-platform desktop apps
- React Flow for graph visualization
- Monaco Editor for code editing
