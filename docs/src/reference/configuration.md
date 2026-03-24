# Configuration Reference

Configuration options for Ownsight.

## CLI Configuration

### Command-Line Flags

```bash
cargo ownership-viz [OPTIONS]
```

#### Required (one of)
- `--file <PATH>`: Analyze a Rust source file
- `--stdin`: Read code from standard input

#### Optional
- `--mode <MODE>`: Analysis mode
  - `teaching` (default): Simplified explanations
  - `debug`: Detailed technical information

- `--backend <BACKEND>`: Analysis backend
  - `simple` (default): Fast syntax-based
  - `mir`: Compiler-accurate (requires nightly)

- `--output <FORMAT>`: Output format
  - `timeline` (default): Human-readable timeline
  - `json`: Machine-readable JSON

- `--function <NAME>`: Analyze specific function (coming soon)

### Environment Variables

```bash
# Set default backend
export OWNSIGHT_BACKEND=mir

# Set default mode
export OWNSIGHT_MODE=debug

# Enable debug logging
export RUST_LOG=debug
```

## Desktop App Configuration

### Settings (Coming Soon)

Future configuration options:
- Default backend
- Default mode
- Theme preferences
- Editor settings
- Keyboard shortcuts

### Current Defaults

- **Mode**: Teaching
- **Backend**: Simple
- **Theme**: Dark
- **Font**: Monaco/Consolas
- **Font Size**: 14px

## Build Configuration

### Feature Flags

#### ownsight-mir
```toml
[features]
default = []
rustc = []  # Enable rustc internals
```

#### ownsight-driver
```toml
[features]
default = []
mir = ["ownsight-mir"]  # Enable MIR backend
```

#### ownsight-ui (Tauri)
```toml
[features]
default = []
mir = ["ownsight-driver/mir"]  # Enable MIR in desktop app
custom-protocol = ["tauri/custom-protocol"]
```

### Build Profiles

#### Development
```toml
[profile.dev]
opt-level = 0
debug = true
```

#### Release
```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
```

## Rust Toolchain

### Simple Backend
```toml
# rust-toolchain.toml
[toolchain]
channel = "stable"
```

### MIR Backend
```toml
# rust-toolchain.toml (in ownsight-mir/)
[toolchain]
channel = "nightly"
components = ["rustc-dev", "llvm-tools-preview"]
```

## Documentation Configuration

### mdBook
```toml
# book.toml
[book]
title = "Ownsight Documentation"
src = "docs/src"

[output.html]
default-theme = "dark"
git-repository-url = "https://github.com/dedsecrattle/ownsight"
```

## CI/CD Configuration

### GitHub Actions

#### Release Build
```yaml
# .github/workflows/release.yml
- uses: dtolnay/rust-toolchain@stable
- uses: tauri-apps/tauri-action@v0
```

#### Documentation
```yaml
# .github/workflows/docs.yml
- uses: actions/configure-pages@v4
- uses: actions/deploy-pages@v4
```

## Advanced Configuration

### Custom Analyzers

To add a custom analyzer:

1. Create analyzer crate
2. Implement `OwnershipAnalyzer` trait
3. Register in `ownsight-driver`
4. Add to backend enum

### Custom Event Types

To add custom events:

1. Extend `EventKind` enum
2. Implement detection logic
3. Add explanation generation
4. Update UI rendering

## Performance Tuning

### Simple Backend
- Already optimized
- O(n) parsing
- Minimal memory

### MIR Backend
- Compilation overhead
- Can cache results
- Trade accuracy for speed

### Desktop App
- Lazy loading
- Virtual scrolling
- Debounced analysis

## Security Configuration

### Tauri Allowlist
```json
{
  "allowlist": {
    "all": false,
    "shell": {
      "open": true
    }
  }
}
```

### CSP (Content Security Policy)
```json
{
  "security": {
    "csp": "default-src 'self'"
  }
}
```

## Troubleshooting

### Configuration Issues

**Problem**: MIR backend not available  
**Solution**: Build with `--features mir`

**Problem**: Wrong Rust version  
**Solution**: Check `rust-toolchain.toml`

**Problem**: Missing components  
**Solution**: `rustup component add rustc-dev`

See [Troubleshooting](../troubleshooting.md) for more.