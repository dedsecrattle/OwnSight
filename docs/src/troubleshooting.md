# Troubleshooting

## Desktop App Issues

### App Won't Launch

**macOS:**
- Right-click and select "Open" if security warning appears
- Go to System Preferences → Security & Privacy → Allow

**Linux:**
- Ensure AppImage is executable: `chmod +x Ownsight*.AppImage`
- Install FUSE if needed: `sudo apt install fuse`

**Windows:**
- Run as Administrator if installation fails
- Check Windows Defender hasn't blocked it

### Analysis Fails

**Syntax Errors:**
- Check code for valid Rust syntax
- Look for missing semicolons, braces
- Try compiling with `rustc` first

**Backend Issues:**
- Try Simple backend first
- Check MIR backend availability (info icon)
- Restart the app

### MIR Backend Not Available

The MIR backend requires building from source:

1. Install nightly Rust:
   ```bash
   rustup toolchain install nightly
   ```

2. Install rustc-dev:
   ```bash
   rustup component add rustc-dev llvm-tools-preview --toolchain nightly
   ```

3. Build from source:
   ```bash
   git clone https://github.com/dedsecrattle/ownsight
   cd ownsight/ui
   cargo build --release --features mir
   bun run tauri dev
   ```

See [Layer 2: MIR Backend](usage/layer2.md) for details.

### Slow Performance

- Use Simple backend for faster analysis
- Close other applications
- Analyze smaller code snippets
- MIR backend is inherently slower

## CLI Tool Issues

### Command Not Found

```bash
# Add cargo bin to PATH
export PATH="$HOME/.cargo/bin:$PATH"

# Or reinstall
cargo install ownsight-cli --force
```

### Build Errors

**Missing Dependencies:**
```bash
# Update Rust
rustup update

# Clean and rebuild
cargo clean
cargo build --release
```

**Nightly Required (for MIR):**
```bash
rustup default nightly
cargo build --release --features mir
```

### Analysis Errors

**File Not Found:**
- Check file path is correct
- Use absolute path if needed
- Ensure file is readable

**Invalid Rust Code:**
- Verify syntax with `rustc --check`
- Look for compilation errors
- Try simpler code first

## Common Error Messages

### "value borrowed after move"

**Cause:** Trying to use a variable after ownership moved

**Solution:**
- Clone the value: `let s2 = s1.clone();`
- Borrow instead: `let s2 = &s1;`
- Restructure code to avoid move

### "cannot borrow as mutable more than once"

**Cause:** Multiple mutable borrows at same time

**Solution:**
- Use only one mutable borrow
- Drop first borrow before creating second
- Use interior mutability (`RefCell`, `Mutex`)

### "MIR backend not available"

**Cause:** App not built with MIR feature

**Solution:**
- Use Simple backend
- Or build from source with `--features mir`
- See [Layer 2 setup](usage/layer2.md)

### "rustc crates not found"

**Cause:** Missing rustc-dev component

**Solution:**
```bash
rustup component add rustc-dev --toolchain nightly
```

## Documentation Issues

### Pages Not Loading

- Check internet connection
- Try refreshing the page
- Clear browser cache
- Visit [GitHub repo](https://github.com/dedsecrattle/ownsight)

### Broken Links

- Report on [GitHub Issues](https://github.com/dedsecrattle/ownsight/issues)
- Check if page exists in navigation

## Getting Help

### Before Asking

1. Check this troubleshooting guide
2. Read relevant documentation
3. Search existing GitHub issues
4. Try with Simple backend first

### Where to Ask

- **GitHub Issues**: Bug reports and feature requests
- **GitHub Discussions**: Questions and ideas
- **Documentation**: Check all guides first

### What to Include

1. **Version**: App/CLI version
2. **OS**: Operating system and version
3. **Code**: Minimal example that reproduces issue
4. **Error**: Full error message
5. **Steps**: What you did before the error
6. **Expected**: What you expected to happen

### Example Bug Report

```
**Version:** Ownsight Desktop v0.1.0
**OS:** macOS 14.0
**Backend:** Simple

**Issue:** Analysis fails with error "unexpected token"

**Code:**
```rust
fn main() {
    let s = String::from("hello")
    println!("{}", s);
}
```

**Error:** "unexpected token: `println`"

**Expected:** Should show syntax error for missing semicolon
```

## Known Issues

- MIR backend requires nightly Rust
- Some advanced Rust features not yet supported
- Large files may be slow to analyze
- Graph view performance with many variables

See [GitHub Issues](https://github.com/dedsecrattle/ownsight/issues) for current known issues.