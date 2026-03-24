# Quick Start

Get up and running with Ownsight in minutes!

## Installation Options

### Option 1: Desktop App (Easiest)

1. **Download** from [GitHub Releases](https://github.com/dedsecrattle/ownsight/releases)
   - macOS: `.dmg` file
   - Linux: `.AppImage` file  
   - Windows: `.msi` installer

2. **Install** and run the application

3. **Start analyzing** - No setup required!

### Option 2: CLI Tool

```bash
# Install from crates.io
cargo install ownsight-cli

# Or build from source
cargo install --path crates/ownsight-cli
```

### Option 3: Build from Source

```bash
git clone https://github.com/dedsecrattle/ownsight
cd ownsight

# Desktop app
cd ui
bun install
bun run tauri dev

# CLI tool
cargo run --bin ownsight-cli -- --help
```

## First Analysis

### Desktop App

1. Open the app
2. You'll see example code already loaded
3. Click **"Analyze"** button
4. Watch the ownership timeline appear!

### CLI Tool

Create a file `example.rs`:
```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}", s2);
}
```

Run analysis:
```bash
cargo ownership-viz --file example.rs
```

## Understanding the Interface

### Timeline View

The timeline shows ownership events step by step:

```
Step 1: Create variable s1
Step 2: Move s1 to s2
Step 3: Use s2
Step 4: Drop s2
```

- **Green boxes**: Variable is created
- **Red arrows**: Ownership moves
- **Blue boxes**: Borrows are created
- **Gray boxes**: Variable is dropped

### Graph View

Shows ownership relationships as a graph:
- Nodes are variables
- Arrows show ownership/borrowing
- Hover to see details

### Query Panel

Ask questions about your code:
- "Why can't I use this variable?"
- "Where was this variable moved?"
- "What borrows this variable?"

## Next Steps

1. Try the [Examples](examples.md) to see common ownership patterns
2. Learn about [Layer 2: MIR Backend](usage/layer2.md) for advanced features
3. Read the [User Guide](usage/basic.md) for detailed usage
4. Check out [Contributing](contributing.md) if you want to help improve Ownsight

## Need Help?

- Check [Troubleshooting](troubleshooting.md) for common issues
- Open an issue on [GitHub](https://github.com/dedsecrattle/ownsight/issues)
- Join our discussions for questions and ideas
