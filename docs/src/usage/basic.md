# Basic Usage

## Desktop App

### Analyzing Code

1. **Enter Code**: Type or paste Rust code in the editor
2. **Select Mode**: Choose Teaching or Debug mode
3. **Click Analyze**: Press the "Analyze" button
4. **View Results**: Explore the timeline and graph views

### Understanding the Interface

#### Source View (Left Panel)
- Shows your Rust code with syntax highlighting
- Highlights active lines as you step through
- Click on variables to see their ownership state

#### Timeline View (Right Panel)
- Shows ownership events chronologically
- Each event has an explanation
- Step through with the controller at the bottom

#### Graph View
- Visual representation of ownership relationships
- Nodes represent variables
- Edges show moves and borrows

### Step Controller

- **Play**: Auto-advance through events
- **Previous/Next**: Step one event at a time
- **Slider**: Jump to any event

### Query Panel

Ask questions about your code:
- "Why can't I use this variable?"
- "Where was this variable moved?"
- "What borrows this variable?"

## CLI Tool

### Basic Analysis

```bash
# Analyze a file
cargo ownership-viz --file example.rs

# Analyze from stdin
echo 'fn main() { let s = String::from("hello"); }' | cargo ownership-viz --stdin
```

### Output Formats

```bash
# JSON output
cargo ownership-viz --file example.rs --output json

# Timeline format (default)
cargo ownership-viz --file example.rs
```

### Modes

```bash
# Teaching mode (simplified explanations)
cargo ownership-viz --file example.rs --mode teaching

# Debug mode (detailed information)
cargo ownership-viz --file example.rs --mode debug
```

## Common Patterns

### Move Semantics
```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;  // s1 is moved to s2
    // println!("{}", s1);  // Error: s1 has been moved
}
```

### Borrowing
```rust
fn main() {
    let s = String::from("hello");
    let r = &s;  // Immutable borrow
    println!("{}", r);
    println!("{}", s);  // OK: s is still valid
}
```

### Mutable Borrowing
```rust
fn main() {
    let mut s = String::from("hello");
    let r = &mut s;  // Mutable borrow
    r.push_str(" world");
    println!("{}", r);
}
```