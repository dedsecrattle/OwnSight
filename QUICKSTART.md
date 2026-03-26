# Ownsight - Quick Start Guide

## What You Just Built

A complete Rust ownership visualizer with:

- **CLI tool** for analyzing Rust code from terminal
- **Desktop app** with interactive timeline, graph view, and stepping
- **Two-layer architecture** ready to grow from exam tool to debugging platform

## Try It Now

### 1. Test the CLI Tool

Analyze a simple Rust snippet:

```bash
cd /Users/dedsec/Documents/ownsight

# Analyze from stdin
echo 'fn main() {
    let s = String::from("hello");
    let r = &s;
    println!("{}", r);
}' | cargo run --bin cargo-ownership-viz -- ownership-viz --stdin

# Analyze a file
cargo run --bin cargo-ownership-viz -- ownership-viz --file tests/snapshots/simple_move.rs

# Get JSON output
cargo run --bin cargo-ownership-viz -- ownership-viz --file tests/snapshots/borrow_example.rs --output json
```

### 2. Run the Desktop App

```bash
cd ui
bun run tauri dev
```

This will:

- Start the Vite dev server on port 1420
- Launch the Tauri desktop application
- Open the Ownsight visualizer

### 3. Try These Examples in the App

**Example 1: Simple Move**

```rust
fn main() {
    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s); // Error: s was moved
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}
```

**Example 2: Borrowing**

```rust
fn main() {
    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
}
```

**Example 3: Mutable Borrow**

```rust
fn main() {
    let mut s = String::from("hello");
    let r = &mut s;
    r.push_str(" world");
    println!("{}", r);
}
```

## Features to Explore

### Timeline View

- Click "Analyze" to process your code
- Use the step controller to navigate through ownership events
- See line-by-line highlighting in the source view
- Watch variables get created, moved, borrowed, and dropped

### Graph View

- Switch to "Graph View" tab
- See visual relationships between variables
- Nodes show variable state (created, moved, borrowed)
- Edges show ownership relationships

### Query Interface

- Select a variable from the dropdown
- Ask questions like:
  - "Why can't I use this?"
  - "Where was this moved?"
  - "What is borrowing this?"
- Get instant answers based on the analysis

### Interactive Stepping

- Play/Pause button for automatic stepping
- Previous/Next buttons for manual control
- Speed control (0.5x to 4x)
- Progress slider to jump to any step

## What's Working

✅ **Core Analysis Engine**

- Variable tracking (name, type, scope, mutability)
- Event extraction (create, move, borrow, drop)
- Ownership state at each line
- Graph generation

✅ **CLI Tool**

- Snippet mode (stdin)
- File mode
- JSON/timeline/text output
- Teaching vs debug modes

✅ **Desktop UI**

- Monaco code editor with syntax highlighting
- Timeline view with event icons and explanations
- Graph visualization with React Flow
- Interactive stepping with play/pause
- Query interface for debugging questions

## Next Steps to Enhance

### Short Term (Layer 1 Polish)

1. Add more test cases in `tests/snapshots/`
2. Improve error messages and explanations
3. Add syntax error handling
4. Create example gallery in the UI

### Medium Term

1. Enhance simple parser with improved analysis
2. Add Cargo workspace support
3. Implement function summaries
4. Handle partial moves (struct fields)
5. Support pattern matching flows
6. Implement NLL (non-lexical lifetimes)

### Long Term

1. VS Code extension
2. Rust Analyzer integration
3. CI/CD integration
4. Web-based version (WASM)
5. Collaborative debugging

## Architecture Overview

```
ownsight/
├── crates/
│   ├── ownsight-core/          # Core data model & analysis
│   │   ├── model.rs            # Variables, Events, Scopes
│   │   ├── events.rs           # Event builder & explanations
│   │   ├── graph.rs            # Ownership graph
│   │   └── analysis.rs         # Query interface
│   │
│   ├── ownsight-driver/        # Rust analysis
│   │   └── simple_analyzer.rs  # Syntax-based parser (MVP)
│   │
│   └── ownsight-cli/           # Command-line tool
│       ├── commands/           # Subcommands
│       └── output/             # Formatters
│
└── ui/                         # Tauri desktop app
    ├── src-tauri/              # Rust backend
    │   ├── main.rs             # Tauri setup
    │   └── commands.rs         # IPC commands
    │
    └── src/                    # React frontend
        ├── components/         # UI components
        ├── types/              # TypeScript types
        └── App.tsx             # Main app
```

## Build for Production

```bash
# Build Rust workspace
cargo build --release

# Build desktop app
cd ui
bun run tauri build

# The built app will be in:
# ui/src-tauri/target/release/bundle/
```

## Troubleshooting

**CLI not working?**

```bash
cargo clean
cargo build --workspace
```

**UI not starting?**

```bash
cd ui
rm -rf node_modules
bun install
bun run tauri dev
```

**TypeScript errors?**

- These are expected before `bun install`
- Run `bun install` in the `ui/` directory

## Contributing

The codebase is structured for easy extension:

1. **Add new event types**: Edit `EventKind` in `model.rs`
2. **Improve analysis**: Enhance `simple_analyzer.rs` with better pattern detection
3. **Add UI features**: Create components in `ui/src/components/`
4. **Add queries**: Extend `analysis.rs` with new query methods

## Resources

- Rust Ownership: https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html
- Tauri Docs: https://tauri.app/
- React Flow: https://reactflow.dev/
- Monaco Editor: https://microsoft.github.io/monaco-editor/

---

**You now have a fully functional Rust ownership visualizer!** 🎉

Start with the CLI to test analysis, then explore the desktop app for interactive visualization.
