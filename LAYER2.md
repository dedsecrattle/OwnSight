# Layer 2: MIR-Based Analysis Implementation

## Overview

Layer 2 provides advanced ownership analysis using Rust's MIR (Mid-level Intermediate Representation) through rustc internals. This enables accurate, compiler-backed analysis supporting all modern Rust features.

## Features Implemented

### ✅ Phase 1: MIR Foundation
- **Nightly Rust Integration**: Uses rustc internals for accurate analysis
- **MIR Traversal Engine**: Extracts ownership events from compiled MIR
- **Rustc Driver**: Custom compiler callbacks for analysis
- **Type Information**: Full type extraction from HIR/MIR

### ✅ Phase 6: Enhanced Data Model
- **MIR Locations**: Track basic blocks and statement indices
- **Lifetime Regions**: Support for NLL (Non-Lexical Lifetimes)
- **Closure Captures**: Track capture modes (ByValue, ByRef, ByMutRef)
- **Async Contexts**: Suspension points and Send/Sync tracking
- **Partial Moves**: Field-level ownership tracking

### ✅ Phase 7: Integration
- **Backend Selection**: Choose between Simple (syntax) or MIR (compiler) backends
- **CLI Support**: `--backend` flag for analyzer selection
- **Graceful Fallback**: Falls back to Simple analyzer if MIR unavailable

### 🚧 Phase 2-5: Advanced Features (Stub Implementation)
- **NLL Analysis**: Precise lifetime regions (foundation ready)
- **Partial Moves**: Struct field tracking (foundation ready)
- **Closure Analysis**: Capture mode detection (foundation ready)
- **Async/Await**: Suspension point tracking (foundation ready)
- **Function Summaries**: Cross-function analysis (foundation ready)

## Architecture

```
ownsight/
├── crates/
│   ├── ownsight-core/       # Core data model (extended for Layer 2)
│   │   └── src/
│   │       ├── model.rs     # Added: MirLocation, Lifetime, CaptureMode, etc.
│   │       └── events.rs    # Added: Layer 2 event kinds
│   │
│   ├── ownsight-mir/        # NEW: MIR-based analyzer
│   │   ├── rust-toolchain.toml  # Nightly toolchain config
│   │   └── src/
│   │       ├── lib.rs           # Module exports
│   │       ├── analyzer.rs      # Main MIR analyzer
│   │       ├── driver.rs        # Rustc driver integration
│   │       ├── mir_visitor.rs   # MIR traversal
│   │       ├── lifetime.rs      # NLL analysis (stub)
│   │       ├── closure.rs       # Closure captures (stub)
│   │       ├── async_await.rs   # Async analysis (stub)
│   │       ├── partial_move.rs  # Field tracking (stub)
│   │       └── function.rs      # Function summaries (stub)
│   │
│   ├── ownsight-driver/     # Analysis driver (extended)
│   │   └── src/
│   │       └── lib.rs       # Added: AnalyzerBackend enum, create_analyzer()
│   │
│   └── ownsight-cli/        # CLI tool (extended)
│       └── src/
│           ├── main.rs      # Added: --backend flag
│           └── commands/
│               └── analyze.rs  # Backend selection logic
```

## Usage

### Basic Usage (Simple Backend - Default)

```bash
# Uses syntax-based analyzer (Layer 1)
cargo ownership-viz --file example.rs
```

### MIR Backend (Layer 2)

```bash
# Uses MIR-based analyzer (requires nightly + rustc-dev)
cargo ownership-viz --file example.rs --backend mir
```

### Backend Options

- `simple`: Syntax-based analyzer (fast, no compilation needed)
- `mir`: MIR-based analyzer (accurate, requires compilation)

## Installation for MIR Backend

### Prerequisites

1. **Install Nightly Rust**:
   ```bash
   rustup toolchain install nightly
   ```

2. **Install rustc-dev Component**:
   ```bash
   rustup component add rustc-dev llvm-tools-preview --toolchain nightly
   ```

3. **Build with MIR Feature**:
   ```bash
   cd crates/ownsight-mir
   cargo +nightly build --features rustc
   ```

## Layer 2 Event Types

### New Event Kinds

```rust
pub enum EventKind {
    // Layer 1 (existing)
    Create, MoveOut, MoveIn, BorrowShared, BorrowMut,
    Reborrow, Use, Drop, StorageLive, StorageDead,
    Reinit, Conflict,
    
    // Layer 2 (new)
    PartialMove,        // Field of struct moved
    ClosureCapture,     // Variable captured by closure
    AwaitSuspend,       // Async suspension point
    AwaitResume,        // Async resume point
    TwoPhaseActivate,   // Two-phase borrow activated
    ReborrowShared,     // Immutable reborrow
    ReborrowMut,        // Mutable reborrow
    FieldAccess,        // Struct field accessed
    MethodCall,         // Method invoked
}
```

## Data Structures

### MIR Location
```rust
pub struct MirLocation {
    pub basic_block: usize,
    pub statement_index: usize,
}
```

### Lifetime Region
```rust
pub struct Lifetime {
    pub id: LifetimeId,
    pub name: Option<String>,
    pub region: Region,
}

pub struct Region {
    pub start: MirLocation,
    pub end: MirLocation,
}
```

### Closure Capture
```rust
pub enum CaptureMode {
    ByValue,    // move || ...
    ByRef,      // || ...
    ByMutRef,   // || ... (with &mut)
}

pub struct ClosureCapture {
    pub var_id: VariableId,
    pub capture_mode: CaptureMode,
    pub by_ref: bool,
}
```

### Async Context
```rust
pub struct AsyncContext {
    pub is_async: bool,
    pub await_points: Vec<usize>,
    pub send_required: bool,
    pub sync_required: bool,
}
```

### Partial Move
```rust
pub struct PartialMoveInfo {
    pub base_var: VariableId,
    pub field_path: Vec<String>,
    pub moved_fields: Vec<String>,
}
```

## Example: MIR Analysis

```rust
// example.rs
fn main() {
    let s = String::from("hello");
    let r = &s;
    println!("{}", r);
    drop(s);
}
```

**Simple Backend Output** (syntax-based):
- Basic move/borrow detection
- Line-based analysis
- Limited type information

**MIR Backend Output** (compiler-based):
- Precise MIR locations
- Full type information
- Accurate lifetime regions
- Drop elaboration
- Storage live/dead events

## Development Status

### ✅ Completed
- [x] Nightly toolchain setup
- [x] MIR crate structure
- [x] Rustc driver integration
- [x] Basic MIR visitor
- [x] Backend selection
- [x] CLI integration
- [x] Extended data model
- [x] Event type extensions

### 🚧 In Progress
- [ ] Full MIR traversal implementation
- [ ] NLL/Polonius integration
- [ ] Closure capture extraction
- [ ] Async/await analysis
- [ ] Partial move tracking
- [ ] Function summary generation

### 📋 Planned
- [ ] Cargo workspace support
- [ ] Cross-crate analysis
- [ ] Pattern matching flows
- [ ] Comprehensive test suite
- [ ] Performance optimization
- [ ] Documentation examples

## Limitations

### Current Limitations

1. **Requires Nightly**: MIR backend needs nightly Rust
2. **Compilation Required**: Must compile code to extract MIR
3. **Stub Implementations**: Advanced features have foundation but need full implementation
4. **No Polonius Yet**: NLL analysis uses basic regions, not full Polonius
5. **Single File**: Currently analyzes single files, not full crates

### Future Enhancements

1. **Polonius Integration**: Full NLL support
2. **Workspace Analysis**: Multi-crate projects
3. **Incremental Analysis**: Cache compiled artifacts
4. **Better Error Handling**: Graceful compilation failures
5. **Performance**: Parallel analysis, caching

## Contributing

To contribute to Layer 2 development:

1. **Set up nightly toolchain**
2. **Implement stub modules** in `ownsight-mir/src/`
3. **Add tests** for new features
4. **Update documentation**

See the implementation plan at `.windsurf/plans/layer2-mir-implementation-bb5304.md`

## Troubleshooting

### "MIR backend not available"
- Install nightly: `rustup toolchain install nightly`
- Install rustc-dev: `rustup component add rustc-dev --toolchain nightly`
- Build with feature: `cargo +nightly build --features rustc`

### Compilation Errors
- Check Rust syntax is valid
- Ensure dependencies are available
- Try Simple backend as fallback

### Performance Issues
- Use Simple backend for quick analysis
- MIR backend is slower but more accurate
- Consider caching for repeated analysis

## References

- [Rust MIR Documentation](https://rustc-dev-guide.rust-lang.org/mir/index.html)
- [Polonius NLL](https://github.com/rust-lang/polonius)
- [Rustc Dev Guide](https://rustc-dev-guide.rust-lang.org/)
