# MIR Backend Troubleshooting

This guide helps resolve common issues with the MIR backend.

## Quick Diagnostics

### Check Backend Availability

**CLI:**
```bash
cargo-ownership-viz --file test.rs --backend mir
```

Look for:
- ✓ Using MIR backend (compiler-backed analysis) - **Working**
- ⚠ MIR backend unavailable, using Simple backend - **Issue**

**Desktop App:**
- Check the backend status indicator in the top bar
- Green dot = MIR active
- Blue dot = Simple active
- Yellow warning = MIR requested but unavailable

## Common Issues

### Issue 1: "MIR backend not available"

**Symptoms:**
```
⚠ MIR backend requested but not available. Falling back to Simple analyzer.
  Reason: MIR backend not compiled. Binary was built without --features mir
```

**Cause:** You're using a binary built without MIR support.

**Solutions:**

**Option A: Download Pre-Built Binary (Recommended)**
1. Visit [GitHub Releases](https://github.com/dedsecrattle/ownsight/releases)
2. Download the latest release for your platform
3. Replace your current binary

**Option B: Build from Source**
```bash
# Install nightly Rust
rustup toolchain install nightly

# Install rustc-dev component
rustup component add rustc-dev llvm-tools-preview --toolchain nightly

# Build CLI with MIR
cd crates/ownsight-cli
cargo +nightly build --release --features mir

# Or build desktop app with MIR
cd ui
bun run tauri build -- --features mir
```

### Issue 2: "MIR backend test failed"

**Symptoms:**
```
⚠ MIR backend unavailable, using Simple backend
  Reason: MIR backend test failed: compilation error
```

**Cause:** MIR backend is compiled but can't run properly.

**Possible Reasons:**
1. Rustc-dev component missing
2. Incompatible nightly version
3. System library issues

**Solutions:**

**Step 1: Verify rustc-dev installation**
```bash
rustup component list --toolchain nightly | grep rustc-dev
```

Should show:
```
rustc-dev-x86_64-unknown-linux-gnu (installed)
```

If not installed:
```bash
rustup component add rustc-dev llvm-tools-preview --toolchain nightly
```

**Step 2: Check nightly version**
```bash
rustup show
```

The MIR backend requires a specific nightly version. Check `crates/ownsight-mir/rust-toolchain.toml` for the pinned version.

**Step 3: Update to correct nightly**
```bash
# Remove old nightly
rustup toolchain remove nightly

# Install specific version (check rust-toolchain.toml)
rustup toolchain install nightly-2026-03-23
rustup component add rustc-dev llvm-tools-preview --toolchain nightly-2026-03-23

# Rebuild
cargo +nightly clean
cargo +nightly build --release --features mir
```

### Issue 3: Compilation Errors with MIR Backend

**Symptoms:**
```
Error: Compilation failed
```

**Cause:** Your Rust code has syntax errors or compilation issues.

**Solutions:**

**Step 1: Verify code compiles**
```bash
rustc --edition 2021 your_file.rs
```

**Step 2: Fix compilation errors**
- MIR backend requires valid Rust code
- Fix syntax errors, missing dependencies, etc.

**Step 3: Use Simple backend for invalid code**
```bash
# Simple backend works with invalid code
cargo-ownership-viz --file your_file.rs --backend simple
```

### Issue 4: Desktop App Shows "MIR Unavailable"

**Symptoms:**
- Desktop app backend indicator shows warning
- MIR option grayed out or shows error

**Cause:** Desktop app binary not built with MIR support.

**Solutions:**

**Option A: Download Official Release**
1. Download from [GitHub Releases](https://github.com/dedsecrattle/ownsight/releases)
2. Official releases include MIR support

**Option B: Build Desktop App with MIR**
```bash
cd ui

# Install dependencies
bun install

# Build with MIR support
bun run tauri build -- --features mir

# On macOS, the app will be in:
# src-tauri/target/release/bundle/macos/

# On Linux:
# src-tauri/target/release/bundle/appimage/

# On Windows:
# src-tauri/target/release/bundle/msi/
```

### Issue 5: Slow Analysis with MIR Backend

**Symptoms:**
- Analysis takes a long time
- Desktop app freezes during analysis

**Cause:** MIR backend requires compilation, which is slower than Simple backend.

**Solutions:**

**This is expected behavior:**
- MIR backend compiles your code to extract MIR
- Compilation takes time, especially for large files

**Workarounds:**
1. Use Simple backend for quick checks
2. Use MIR backend only when you need precise analysis
3. Analyze smaller code snippets

**Performance Tips:**
```bash
# For quick checks: Simple backend
cargo-ownership-viz --file large.rs --backend simple

# For precise analysis: MIR backend
cargo-ownership-viz --file complex.rs --backend mir
```

### Issue 6: Platform-Specific Issues

#### macOS: "Cannot verify developer"

**Symptoms:**
```
"Ownsight" cannot be opened because the developer cannot be verified.
```

**Solution:**
```bash
# Remove quarantine attribute
xattr -d com.apple.quarantine /Applications/Ownsight.app

# Or right-click > Open > Open anyway
```

#### Linux: AppImage won't run

**Symptoms:**
```
Permission denied
```

**Solution:**
```bash
# Make executable
chmod +x ownsight_*.AppImage

# Run
./ownsight_*.AppImage
```

#### Windows: Missing DLL errors

**Symptoms:**
```
VCRUNTIME140.dll was not found
```

**Solution:**
1. Install [Visual C++ Redistributable](https://aka.ms/vs/17/release/vc_redist.x64.exe)
2. Restart and try again

### Issue 7: CI/CD Build Failures

**Symptoms:**
- GitHub Actions fails to build MIR backend
- "rustc-dev not found" error

**Cause:** CI workflow not configured for nightly + rustc-dev.

**Solution:**

Update `.github/workflows/release.yml`:
```yaml
- name: Setup Rust (Nightly for MIR support)
  uses: dtolnay/rust-toolchain@nightly
  with:
    components: rustc-dev, llvm-tools-preview
```

### Issue 8: Crates.io Installation Doesn't Have MIR

**Symptoms:**
```bash
cargo install ownsight-cli
# MIR backend not available
```

**Cause:** Crates.io doesn't support nightly-only features.

**Solution:**

**This is expected.** Use one of these alternatives:

**Option A: Pre-built binaries (Recommended)**
```bash
# Download from GitHub releases
wget https://github.com/dedsecrattle/ownsight/releases/download/cli-v0.2.0/cargo-ownership-viz-linux-x64
chmod +x cargo-ownership-viz-linux-x64
sudo mv cargo-ownership-viz-linux-x64 /usr/local/bin/cargo-ownership-viz
```

**Option B: Build from source**
```bash
git clone https://github.com/dedsecrattle/ownsight.git
cd ownsight/crates/ownsight-cli
cargo +nightly install --path . --features mir
```

## Advanced Debugging

### Enable Verbose Logging

**CLI:**
```bash
RUST_LOG=debug cargo-ownership-viz --file test.rs --backend mir
```

**Desktop App:**
Open DevTools (Cmd+Option+I on macOS, Ctrl+Shift+I on Windows/Linux) to see console logs.

### Check MIR Backend Health

**Programmatic Check:**
```rust
use ownsight_driver::check_mir_availability;

let status = check_mir_availability();
println!("Simple available: {}", status.simple_available);
println!("MIR available: {}", status.mir_available);
if let Some(err) = status.mir_error {
    println!("MIR error: {}", err);
}
```

### Verify Nightly Toolchain

```bash
# Show active toolchain
rustup show

# List installed components
rustup component list --toolchain nightly

# Verify rustc-dev
rustc +nightly -vV
```

Should show:
```
rustc 1.XX.0-nightly (hash date)
binary: rustc
commit-hash: ...
commit-date: ...
host: x86_64-unknown-linux-gnu
release: 1.XX.0-nightly
LLVM version: XX.X.X
```

## Getting Help

### Before Asking for Help

1. **Check this troubleshooting guide**
2. **Verify your setup:**
   ```bash
   rustup show
   rustup component list --toolchain nightly | grep rustc-dev
   cargo-ownership-viz --file test.rs --backend mir
   ```
3. **Try Simple backend** to isolate MIR-specific issues

### Where to Get Help

1. **GitHub Issues**: [Report a bug](https://github.com/dedsecrattle/ownsight/issues/new)
2. **GitHub Discussions**: [Ask a question](https://github.com/dedsecrattle/ownsight/discussions)
3. **Documentation**: [Read the docs](https://dedsecrattle.github.io/ownsight/)

### What to Include in Bug Reports

```markdown
**Environment:**
- OS: [e.g., macOS 14.0, Ubuntu 22.04, Windows 11]
- Rust version: [output of `rustc --version`]
- Nightly version: [output of `rustc +nightly --version`]
- Ownsight version: [e.g., 0.2.0]
- Installation method: [pre-built binary / crates.io / source]

**Issue:**
[Describe the issue]

**Steps to Reproduce:**
1. [First step]
2. [Second step]
3. [...]

**Expected Behavior:**
[What you expected to happen]

**Actual Behavior:**
[What actually happened]

**Logs/Output:**
```
[Paste relevant logs or error messages]
```

**Additional Context:**
[Any other relevant information]
```

## FAQ

**Q: Do I need nightly Rust to use Ownsight?**
A: No! Pre-built binaries include MIR support. You only need nightly if building from source with MIR.

**Q: Can I use both backends?**
A: Yes! Switch between them anytime:
```bash
# Simple backend
cargo-ownership-viz --file test.rs --backend simple

# MIR backend
cargo-ownership-viz --file test.rs --backend mir
```

**Q: Which backend should I use?**
A: Simple for learning/quick checks, MIR for debugging/production code.

**Q: Why is MIR backend slower?**
A: It compiles your code to extract MIR. This is necessary for compiler-accurate analysis.

**Q: Will MIR work on stable Rust someday?**
A: Unlikely. MIR uses rustc internals which are nightly-only. But pre-built binaries make this transparent.

**Q: How often should I update the nightly version?**
A: Only when the project updates `rust-toolchain.toml`. Pre-built binaries handle this automatically.
