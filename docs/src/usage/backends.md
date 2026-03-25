# Analysis Backends

Ownsight provides two analysis backends, each optimized for different use cases.

## Backend Comparison

| Feature | Simple Backend | MIR Backend |
|---------|---------------|-------------|
| **Speed** | ⚡ Fast | 🐢 Slower (requires compilation) |
| **Accuracy** | Good for basic patterns | Excellent (compiler-backed) |
| **Setup** | None required | Included in pre-built binaries |
| **Rust Version** | Stable | Nightly (bundled) |
| **Use Case** | Learning, quick checks | Debugging, production code |
| **Partial Moves** | ❌ Not supported | ✅ Supported |
| **Closures** | ❌ Limited | ✅ Full capture analysis |
| **Async/Await** | ❌ Not supported | ✅ Suspension points |
| **NLL** | ❌ Not supported | ✅ Non-lexical lifetimes |

## Simple Backend (Layer 1)

The Simple backend uses syntax-based analysis without requiring compilation.

### When to Use
- **Learning Rust**: Clear, simplified explanations
- **Quick checks**: Fast analysis without compilation overhead
- **Exam preparation**: Focus on core ownership concepts
- **Simple code**: Basic ownership patterns

### Features
- Fast analysis (no compilation)
- Works with any Rust code (even invalid)
- Simplified explanations for teaching
- No setup required

### Example
```bash
# CLI
cargo-ownership-viz --file example.rs --backend simple

# Desktop app: Select "Simple" in backend settings
```

## MIR Backend (Layer 2)

The MIR backend uses Rust's Mid-level Intermediate Representation for compiler-accurate analysis.

### When to Use
- **Debugging production code**: Precise, compiler-backed analysis
- **Complex patterns**: Closures, async/await, partial moves
- **Understanding compiler errors**: See exactly what the compiler sees
- **Advanced features**: NLL, two-phase borrows, etc.

### Features
- Compiler-accurate analysis
- Partial move detection (struct fields)
- Closure capture analysis (ByValue, ByRef, ByMutRef)
- Async/await support (suspension points, Send/Sync)
- Non-lexical lifetimes (NLL)
- Precise type information

### Example
```bash
# CLI
cargo-ownership-viz --file example.rs --backend mir

# Desktop app: Select "MIR" in backend settings
```

## Installation & Setup

### Pre-built Binaries (Recommended)

Download pre-built binaries with MIR support included:

**Desktop App:**
- Download from [GitHub Releases](https://github.com/dedsecrattle/ownsight/releases)
- MIR backend is included and ready to use
- No additional setup required

**CLI Tool:**
- Download from [CLI Releases](https://github.com/dedsecrattle/ownsight/releases)
- Extract and add to PATH
- MIR backend works immediately

### From crates.io

```bash
# Install CLI (Simple backend only)
cargo install ownsight-cli

# For MIR support, build from source:
git clone https://github.com/dedsecrattle/ownsight.git
cd ownsight
cargo +nightly build --release --features mir
```

### Building from Source with MIR

**Prerequisites:**
```bash
# Install nightly Rust
rustup toolchain install nightly

# Install rustc-dev component
rustup component add rustc-dev llvm-tools-preview --toolchain nightly
```

**Build:**
```bash
# CLI with MIR support
cd crates/ownsight-cli
cargo +nightly build --release --features mir

# Desktop app with MIR support
cd ui
bun install
bun run tauri build -- --features mir
```

## Usage Examples

### CLI: Comparing Backends

**Simple Backend:**
```bash
$ cargo-ownership-viz --file complex.rs --backend simple
✓ Using Simple backend (syntax-based analysis)

Timeline:
  Line 5: Variable `data` created
  Line 6: Variable `data` moved
  ...
```

**MIR Backend:**
```bash
$ cargo-ownership-viz --file complex.rs --backend mir
✓ Using MIR backend (compiler-backed analysis)

Timeline:
  Line 5: Variable `data` storage allocated
  Line 5: Variable `data` initialized
  Line 6: Field `data.field1` moved (partial move)
  Line 7: Variable `data` partially moved (cannot use)
  ...
```

### Desktop App: Backend Selection

1. Open Settings panel
2. Select backend:
   - **Simple**: Fast, syntax-based
   - **MIR**: Accurate, compiler-based
3. Status indicator shows active backend
4. Analyze button uses selected backend

## Backend Status Indicators

### CLI
```bash
✓ Using MIR backend (compiler-backed analysis)
⚠ MIR backend unavailable, using Simple backend
  Reason: MIR feature not enabled in this build
```

### Desktop App
```
┌─────────────────────────────┐
│ Backend: MIR ✓              │
│ Status: Compiler-backed     │
└─────────────────────────────┘
```

## Troubleshooting

### "MIR backend not available"

**Cause**: Binary was built without MIR support

**Solution**:
1. Download pre-built binary from GitHub releases (recommended)
2. Or build from source with `--features mir`

### "MIR backend test failed"

**Cause**: Rustc-dev component missing or incompatible

**Solution**:
```bash
# Reinstall rustc-dev
rustup component remove rustc-dev --toolchain nightly
rustup component add rustc-dev llvm-tools-preview --toolchain nightly

# Rebuild
cargo +nightly build --features mir
```

### "Compilation failed"

**Cause**: Code has syntax errors

**Solution**:
- Fix syntax errors in your code
- Or use Simple backend which works with invalid code

### Performance Issues

**MIR backend is slow:**
- Expected: MIR requires compilation
- Use Simple backend for quick checks
- MIR is worth it for complex analysis

## Advanced: Auto-Detection

Ownsight automatically detects MIR availability:

```rust
// In your code
use ownsight_driver::{check_mir_availability, AnalyzerBackend};

let status = check_mir_availability();
if status.mir_available {
    println!("MIR backend ready!");
} else {
    println!("MIR unavailable: {:?}", status.mir_error);
}
```

## Feature Comparison Examples

### Partial Moves

**Code:**
```rust
struct Data {
    field1: String,
    field2: String,
}

fn main() {
    let data = Data {
        field1: String::from("hello"),
        field2: String::from("world"),
    };
    let f1 = data.field1; // Partial move
    // Can still use data.field2
    println!("{}", data.field2);
}
```

**Simple Backend:**
- Shows: `data` moved at line 10
- Limitation: Doesn't track field-level moves

**MIR Backend:**
- Shows: `data.field1` moved at line 10
- Shows: `data.field2` still valid
- Accurate partial move tracking

### Closures

**Code:**
```rust
fn main() {
    let x = String::from("hello");
    let closure = || println!("{}", x); // Capture by reference
    closure();
    println!("{}", x); // Still valid
}
```

**Simple Backend:**
- Shows: Basic closure detection
- Limitation: Doesn't show capture mode

**MIR Backend:**
- Shows: `x` captured by reference (ByRef)
- Shows: `x` still valid after closure call
- Precise capture mode analysis

### Async/Await

**Code:**
```rust
async fn process(data: String) {
    println!("Processing: {}", data);
}

async fn main() {
    let s = String::from("hello");
    process(s).await;
}
```

**Simple Backend:**
- Shows: Basic move detection
- Limitation: No async-specific analysis

**MIR Backend:**
- Shows: Suspension points
- Shows: Send/Sync requirements
- Shows: Async state machine details

## Best Practices

### For Students
1. Start with Simple backend for learning
2. Use MIR backend when confused by compiler errors
3. Compare both backends to understand differences

### For Professionals
1. Use MIR backend for production code analysis
2. Use Simple backend for quick sanity checks
3. Enable MIR in CI for ownership verification

### For Contributors
1. Test changes with both backends
2. Add test cases for MIR-specific features
3. Document backend-specific behavior

## FAQ

**Q: Which backend should I use?**
A: Simple for learning, MIR for debugging production code.

**Q: Is MIR backend slower?**
A: Yes, it requires compilation. But it's much more accurate.

**Q: Can I switch backends in the desktop app?**
A: Yes, use the Settings panel to switch anytime.

**Q: Do pre-built binaries include MIR?**
A: Yes! Download from releases for MIR support out-of-the-box.

**Q: Why does MIR need nightly Rust?**
A: MIR uses rustc internals which are only available on nightly.

**Q: Will MIR work on stable Rust someday?**
A: Unlikely. But pre-built binaries make this transparent to users.
