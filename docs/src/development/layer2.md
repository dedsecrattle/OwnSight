# Layer 2 Development

Development guide for the MIR-based analysis backend.

## Setup

### Prerequisites

```bash
# Install nightly Rust
rustup toolchain install nightly
rustup default nightly

# Install rustc-dev
rustup component add rustc-dev llvm-tools-preview
```

### Building

```bash
# Build MIR crate
cd crates/ownsight-mir
cargo build --features rustc

# Run tests
cargo test --features rustc
```

## Architecture

### Components

```
ownsight-mir/
├── src/
│   ├── lib.rs           # Module exports
│   ├── analyzer.rs      # Main MIR analyzer
│   ├── driver.rs        # Rustc driver integration
│   ├── mir_visitor.rs   # MIR traversal
│   ├── lifetime.rs      # NLL analysis
│   ├── closure.rs       # Closure captures
│   ├── async_await.rs   # Async/await
│   ├── partial_move.rs  # Field tracking
│   └── function.rs      # Function summaries
└── tests/               # Integration tests
```

### Key Concepts

#### MIR (Mid-level IR)
- Simplified representation of Rust code
- Used by compiler for borrow checking
- Control flow graph of basic blocks

#### Rustc Driver
- Custom compiler driver
- Intercepts compilation after MIR generation
- Extracts ownership information

#### Analysis Flow

```
Source → HIR → MIR → Analysis → Events
```

## Implementing Analyzers

### Example: Partial Move Analyzer

```rust
pub struct PartialMoveAnalyzer {
    partial_moves: Vec<PartialMoveInfo>,
}

impl PartialMoveAnalyzer {
    pub fn analyze_body(&mut self, body: &Body<'_>) {
        for (bb, data) in body.basic_blocks().iter_enumerated() {
            for (idx, statement) in data.statements.iter().enumerate() {
                self.analyze_statement(statement, bb, idx);
            }
        }
    }
    
    fn analyze_statement(&mut self, stmt: &Statement<'_>, bb: BasicBlock, idx: usize) {
        // Detect field moves
        // Generate PartialMoveInfo
        // Add to partial_moves
    }
}
```

### Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_partial_move_detection() {
        let analyzer = PartialMoveAnalyzer::new();
        // Test logic
    }
}
```

## Working with Rustc Internals

### Accessing MIR

```rust
use rustc_middle::mir::{Body, BasicBlock, Statement};
use rustc_middle::ty::TyCtxt;

fn analyze_mir<'tcx>(tcx: TyCtxt<'tcx>, body: &Body<'tcx>) {
    // Access MIR data
}
```

### Type Information

```rust
use rustc_middle::ty::{Ty, TyKind};

fn get_type_info<'tcx>(ty: Ty<'tcx>) -> String {
    match ty.kind() {
        TyKind::Adt(def, _) => def.did().to_string(),
        TyKind::Ref(_, ty, _) => format!("&{}", get_type_info(ty)),
        _ => ty.to_string(),
    }
}
```

## Common Tasks

### Adding New Event Type

1. **Define in core:**
   ```rust
   // ownsight-core/src/events.rs
   pub enum EventKind {
       // ...
       NewEventType,
   }
   ```

2. **Detect in MIR:**
   ```rust
   // ownsight-mir/src/analyzer.rs
   fn detect_new_event(&self, stmt: &Statement) -> Option<OwnershipEvent> {
       // Detection logic
   }
   ```

3. **Generate explanation:**
   ```rust
   // ownsight-core/src/events.rs
   EventKind::NewEventType => "Explanation".to_string(),
   ```

### Debugging

```bash
# Enable rustc logging
RUSTC_LOG=debug cargo test --features rustc

# Print MIR
cargo rustc -- -Z dump-mir=all

# Use rust-analyzer (limited support)
```

## Best Practices

### Feature Gating

Always use `#[cfg(feature = "rustc")]`:

```rust
#[cfg(feature = "rustc")]
use rustc_middle::mir::Body;

#[cfg(feature = "rustc")]
pub fn analyze_mir(body: &Body) {
    // Implementation
}

#[cfg(not(feature = "rustc"))]
pub fn analyze_mir(_body: &()) {
    // Stub
}
```

### Error Handling

```rust
use anyhow::{Result, Context};

fn analyze() -> Result<ProgramAnalysis> {
    let body = get_mir_body()
        .context("Failed to get MIR body")?;
    
    // Analysis
    Ok(analysis)
}
```

### Testing

- Test with and without `rustc` feature
- Use integration tests for full pipeline
- Unit test individual analyzers

## Roadmap

### Current Status
- ✅ Basic MIR traversal
- ✅ Event generation
- ✅ Backend integration
- 🚧 Advanced analyzers (stubs)

### Next Steps
1. Implement full NLL analysis
2. Complete closure capture detection
3. Async/await suspension points
4. Function summaries
5. Cross-crate analysis

### Future
- Polonius integration
- Incremental analysis
- Performance optimization
- More event types

## Resources

- [Rustc Dev Guide](https://rustc-dev-guide.rust-lang.org/)
- [MIR Documentation](https://rustc-dev-guide.rust-lang.org/mir/index.html)
- [Polonius](https://github.com/rust-lang/polonius)
- [Chalk](https://github.com/rust-lang/chalk)

## Contributing

See [Contributing Guide](../contributing.md) for general guidelines.

### MIR-Specific
- Requires nightly Rust knowledge
- Familiarity with rustc internals helpful
- Test thoroughly with `rustc` feature
- Document rustc version compatibility