# Layer 2: MIR Backend

Layer 2 provides advanced ownership analysis using Rust's MIR (Mid-level Intermediate Representation). It offers compiler-accurate analysis for complex scenarios.

## What is MIR?

MIR is Rust's intermediate representation - a simplified form of your code that the compiler actually analyzes. By using MIR, Layer 2 can understand ownership exactly as the compiler does.

## Key Features

### 🔍 Partial Move Detection

See when only parts of a struct are moved:

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 1, y: 2 };
    let x = p.x;  // Partial move!
    // println!("{}", p.y);  // Error: p is partially moved
}
```

**Layer 1 (Simple)**: Shows `p` is fully moved  
**Layer 2 (MIR)**: Shows only `p.x` is moved, `p.y` is inaccessible

### 📦 Closure Capture Analysis

Understand how closures capture variables:

```rust
let s = String::from("hello");

// ByRef capture
let r1 = || println!("{}", s);

// ByValue capture
let r2 = move || drop(s);

// ByMutRef capture
let r3 = || s.push_str(" world");
```

Layer 2 shows exactly how each closure captures its variables.

### ⏸️ Async/Await Analysis

Track variables across await points:

```rust
async fn process() {
    let data = vec![1, 2, 3];
    
    // Suspension point 1
    fetch().await;
    
    // data is still valid here
    process(&data).await;
}
```

Layer 2 identifies suspension points and which variables survive them.

### 🔄 Non-Lexical Lifetimes (NLL)

Understand modern lifetime rules:

```rust
let mut v = vec![1, 2, 3];
let r = &v[0];
v.push(4);  // OK with NLL!
println!("{}", r);
```

Layer 2 uses the same analysis as the compiler.

## Enabling Layer 2

### Option 1: Build from Source

1. **Install Nightly Rust**
   ```bash
   rustup toolchain install nightly
   rustup default nightly
   ```

2. **Install rustc-dev**
   ```bash
   rustup component add rustc-dev llvm-tools-preview
   ```

3. **Build with MIR**
   ```bash
   git clone https://github.com/dedsecrattle/ownsight
   cd ownsight/ui
   cargo build --release --features mir
   bun run tauri dev
   ```

### Option 2: CLI Tool

```bash
# Build CLI with MIR
cargo build --release --bin ownsight-cli --features mir

# Use MIR backend
cargo run --bin ownsight-cli --features mir -- \
  analyze --backend mir example.rs
```

### Option 3: Check Availability

In the desktop app:
- Look at the backend selector dropdown
- If "MIR Backend" is disabled, click the info icon (ℹ️)
- Follow the setup guide to enable it

## Comparison

| Feature | Simple (Layer 1) | MIR (Layer 2) |
|---------|------------------|---------------|
| Basic ownership | ✅ | ✅ |
| Moves & borrows | ✅ | ✅ |
| Partial moves | ❌ | ✅ |
| Closure captures | ❌ | ✅ |
| Async/await | ❌ | ✅ |
| NLL support | ❌ | ✅ |
| Performance | Fast | Slower |
| Requirements | Stable Rust | Nightly + rustc-dev |

## Examples

### Example 1: Partial Moves

```rust
struct User {
    name: String,
    age: u32,
    email: String,
}

fn main() {
    let user = User {
        name: String::from("Alice"),
        age: 30,
        email: String::from("alice@example.com"),
    };
    
    let name = user.name;  // Partial move
    
    // Error: can't use user anymore
    println!("Age: {}", user.age);
}
```

**Layer 2 Output:**
- Shows `user.name` is moved
- Marks `user` as partially moved
- Explains why `user.age` is inaccessible

### Example 2: Closure Captures

```rust
fn main() {
    let mut count = 0;
    
    let increment = || {
        count += 1;  // Captures by mutable reference
        count
    };
    
    let value = increment();
    println!("Count: {}", value);
}
```

**Layer 2 Output:**
- Identifies `count` is captured by `&mut`
- Shows capture mode: `ByMutRef`
- Tracks lifetime of closure

### Example 3: Async Suspension

```rust
async fn fetch_data() -> String {
    String::from("data")
}

async fn process() {
    let data = String::from("hello");
    
    // Suspension point
    let fetched = fetch_data().await;
    
    // data is still valid
    println!("{} {}", data, fetched);
}
```

**Layer 2 Output:**
- Marks suspension point at `await`
- Shows `data` survives across await
- Identifies async context

## Performance Considerations

Layer 2 requires:
1. **Compilation**: Code must be compiled to MIR
2. **Memory**: Uses more memory for MIR data
3. **Time**: Analysis is slower but more accurate

**Tips:**
- Use Simple backend for quick checks
- Switch to MIR for complex cases
- Cache results for repeated analysis

## Troubleshooting

### "MIR backend not available"

**Cause:** Built without MIR feature  
**Fix:** Rebuild with `--features mir`

### "Can't find rustc crates"

**Cause:** Missing rustc-dev component  
**Fix:** `rustup component add rustc-dev --toolchain nightly`

### "Compilation failed"

**Cause:** Code has syntax errors or missing dependencies  
**Fix:** Check code syntax, use Simple backend as fallback

## Future Development

- [ ] Separate MIR CLI tool for dynamic installation
- [ ] Lifetime visualization
- [ ] Cross-function analysis
- [ ] Performance optimizations
- [ ] Cargo workspace support

## Technical Details

Layer 2 works by:
1. Parsing code with rustc
2. Lowering to MIR
3. Traversing MIR statements
4. Extracting ownership events
5. Building timeline with precise locations

For more technical details, see [Layer 2 Development](../development/layer2.md).
