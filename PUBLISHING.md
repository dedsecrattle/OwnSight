# Publishing Guide - Ownsight

This guide covers how to publish both the CLI tool and desktop application.

## Publishing the CLI Tool to crates.io

### Prerequisites

1. **Create a crates.io account**: https://crates.io/
2. **Get your API token**: https://crates.io/me
3. **Login via cargo**:
   ```bash
   cargo login <your-api-token>
   ```

### Prepare for Publishing

1. **Update package metadata** in `crates/ownsight-cli/Cargo.toml`:

   ```toml
   [package]
   name = "ownsight-cli"
   version = "0.1.0"
   description = "CLI tool for visualizing Rust ownership and borrowing"
   authors = ["Your Name <your.email@example.com>"]
   license = "MIT OR Apache-2.0"
   repository = "https://github.com/yourusername/ownsight"
   homepage = "https://github.com/yourusername/ownsight"
   documentation = "https://docs.rs/ownsight-cli"
   readme = "../../README.md"
   keywords = ["rust", "ownership", "borrow-checker", "visualization", "learning"]
   categories = ["development-tools", "visualization", "command-line-utilities"]
   edition = "2021"
   ```

2. **Add LICENSE files** (already have MIT OR Apache-2.0):

   ```bash
   # These should be in the root directory
   touch LICENSE-MIT LICENSE-APACHE
   ```

3. **Verify the package**:
   ```bash
   cd crates/ownsight-cli
   cargo package --list
   cargo package --allow-dirty
   ```

### Publish Dependencies First

Since `ownsight-cli` depends on `ownsight-core` and `ownsight-driver`, publish them first:

```bash
# 1. Publish ownsight-core
cd crates/ownsight-core
cargo publish

# 2. Publish ownsight-driver
cd ../ownsight-driver
cargo publish

# 3. Publish ownsight-cli
cd ../ownsight-cli
cargo publish
```

### Installation After Publishing

Users can install with:

```bash
cargo install ownsight-cli
```

Then use:

```bash
cargo ownership-viz --file example.rs
```

---

## Publishing the Desktop App

### Option 1: GitHub Releases (Recommended)

#### Setup GitHub Actions for Automated Builds

Create `.github/workflows/release.yml`:

```yaml
name: Release

on:
  push:
    tags:
      - "v*"

jobs:
  release:
    strategy:
      fail-fast: false
      matrix:
        platform: [macos-latest, ubuntu-20.04, windows-latest]
    runs-on: ${{ matrix.platform }}

    steps:
      - name: Checkout repository
        uses: actions/checkout@v4

      - name: Install dependencies (ubuntu only)
        if: matrix.platform == 'ubuntu-20.04'
        run: |
          sudo apt-get update
          sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.0-dev libappindicator3-dev librsvg2-dev patchelf

      - name: Rust setup
        uses: dtolnay/rust-toolchain@stable

      - name: Rust cache
        uses: swatinem/rust-cache@v2
        with:
          workspaces: "./ui/src-tauri -> target"

      - name: Install Bun
        uses: oven-sh/setup-bun@v1

      - name: Install frontend dependencies
        run: cd ui && bun install

      - name: Build the app
        uses: tauri-apps/tauri-action@v0
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        with:
          projectPath: ui
          tagName: ${{ github.ref_name }}
          releaseName: "Ownsight v__VERSION__"
          releaseBody: "See the assets to download this version and install."
          releaseDraft: true
          prerelease: false

  publish-cli:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo publish --token ${{ secrets.CARGO_TOKEN }} -p ownsight-core
      - run: cargo publish --token ${{ secrets.CARGO_TOKEN }} -p ownsight-driver
      - run: cargo publish --token ${{ secrets.CARGO_TOKEN }} -p ownsight-cli
```

#### Create a Release

1. **Tag a version**:

   ```bash
   git tag -a v0.1.0 -m "Release v0.1.0"
   git push origin v0.1.0
   ```

2. **GitHub Actions will automatically**:
   - Build for macOS, Linux, and Windows
   - Create installers (.dmg, .AppImage, .msi)
   - Attach them to the GitHub release

3. **Users download** from: `https://github.com/yourusername/ownsight/releases`

### Option 2: Manual Distribution

#### macOS

```bash
cd ui
bun run tauri build

# Output will be in:
# ui/src-tauri/target/release/bundle/dmg/Ownsight_0.1.0_x64.dmg
# ui/src-tauri/target/release/bundle/macos/Ownsight.app
```

**Distribute**:

- Upload `.dmg` to your website or GitHub releases
- Users drag to Applications folder

**Code Signing** (for distribution outside App Store):

```bash
# Get a Developer ID certificate from Apple
# Then sign:
codesign --deep --force --verify --verbose --sign "Developer ID Application: Your Name" \
  ui/src-tauri/target/release/bundle/macos/Ownsight.app

# Notarize for Gatekeeper
xcrun notarytool submit ui/src-tauri/target/release/bundle/dmg/Ownsight_0.1.0_x64.dmg \
  --apple-id "your@email.com" --password "app-specific-password" --team-id "TEAM_ID"
```

#### Linux

```bash
cd ui
bun run tauri build

# Output formats:
# - .deb (Debian/Ubuntu)
# - .AppImage (Universal)
# - .rpm (Fedora/RedHat)
```

**Distribute**:

- `.AppImage`: Direct download, no installation needed
- `.deb`: For Debian/Ubuntu users
- `.rpm`: For Fedora/RHEL users

**Publish to package managers**:

- **Flathub**: Create a Flatpak manifest
- **Snap Store**: Create a snapcraft.yaml
- **AUR** (Arch): Create a PKGBUILD

#### Windows

```bash
cd ui
bun run tauri build

# Output:
# ui/src-tauri/target/release/bundle/msi/Ownsight_0.1.0_x64_en-US.msi
# ui/src-tauri/target/release/bundle/nsis/Ownsight_0.1.0_x64-setup.exe
```

**Distribute**:

- `.msi`: Windows Installer
- `.exe`: NSIS installer (recommended)

**Code Signing** (optional but recommended):

```bash
# Get a code signing certificate
# Use signtool.exe to sign the installer
```

### Option 3: Web Distribution (Future)

For a web version using WASM:

1. **Compile analysis engine to WASM**:

   ```bash
   cd crates/ownsight-core
   wasm-pack build --target web
   ```

2. **Create web-only UI**:
   - Remove Tauri dependencies
   - Use WASM module for analysis
   - Deploy to Vercel/Netlify/GitHub Pages

3. **Benefits**:
   - No installation required
   - Works on any platform
   - Easy to share (just a URL)

---

## Distribution Checklist

### Before First Release

- [ ] Add LICENSE-MIT and LICENSE-APACHE files
- [ ] Update all Cargo.toml files with proper metadata
- [ ] Create comprehensive README.md
- [ ] Add CHANGELOG.md
- [ ] Set up GitHub repository
- [ ] Create demo GIFs/screenshots
- [ ] Write documentation
- [ ] Test on all platforms

### CLI Publishing Checklist

- [ ] Update version in Cargo.toml
- [ ] Run `cargo test --workspace`
- [ ] Run `cargo clippy --workspace`
- [ ] Update CHANGELOG.md
- [ ] Publish ownsight-core
- [ ] Publish ownsight-driver
- [ ] Publish ownsight-cli
- [ ] Test installation: `cargo install ownsight-cli`
- [ ] Update documentation

### Desktop App Publishing Checklist

- [ ] Update version in ui/src-tauri/Cargo.toml
- [ ] Update version in ui/package.json
- [ ] Update version in ui/src-tauri/tauri.conf.json
- [ ] Test build on all platforms
- [ ] Create release notes
- [ ] Tag version in git
- [ ] Build installers
- [ ] Sign binaries (macOS/Windows)
- [ ] Upload to GitHub releases
- [ ] Announce release

---

## Marketing & Distribution

### 1. GitHub

- Create an attractive README with:
  - Demo GIF showing the visualizer in action
  - Feature list
  - Installation instructions
  - Screenshots
- Add topics: `rust`, `ownership`, `visualization`, `learning`, `borrow-checker`
- Pin repository

### 2. crates.io

- Good description and keywords
- Link to documentation
- Link to GitHub repository

### 3. Community Announcements

**Reddit**:

- r/rust
- r/learnrust
- r/programming

**Hacker News**:

- Show HN: Ownsight - Interactive Rust Ownership Visualizer

**Twitter/X**:

- Tag @rustlang
- Use hashtags: #rustlang #rust

**This Week in Rust**:

- Submit to newsletter

**Rust Blog**:

- Write a blog post about the project

### 4. Documentation Sites

- **docs.rs**: Automatically generated from crates.io
- **GitHub Pages**: Host interactive demos
- **YouTube**: Create tutorial videos

---

## Update Strategy

### Versioning (Semantic Versioning)

- **0.1.0**: Initial release (MVP)
- **0.2.0**: Enhanced analysis features
- **0.3.0**: Add VS Code extension
- **1.0.0**: Production-ready, stable API

### Release Cadence

- **Patch releases** (0.1.x): Bug fixes, weekly if needed
- **Minor releases** (0.x.0): New features, monthly
- **Major releases** (x.0.0): Breaking changes, when ready

### Auto-Update (Desktop App)

Tauri supports auto-updates. Configure in `tauri.conf.json`:

```json
{
  "tauri": {
    "updater": {
      "active": true,
      "endpoints": [
        "https://github.com/yourusername/ownsight/releases/latest/download/latest.json"
      ],
      "dialog": true,
      "pubkey": "YOUR_PUBLIC_KEY"
    }
  }
}
```

---

## Quick Start Commands

### Publish CLI

```bash
# One-time setup
cargo login

# For each release
cd crates/ownsight-core && cargo publish
cd ../ownsight-driver && cargo publish
cd ../ownsight-cli && cargo publish
```

### Build Desktop App

```bash
# macOS
cd ui && bun run tauri build

# The installer will be in:
# ui/src-tauri/target/release/bundle/dmg/
```

### Create GitHub Release

```bash
# Tag and push
git tag -a v0.1.0 -m "Release v0.1.0"
git push origin v0.1.0

# GitHub Actions will handle the rest
```

---

## Support & Maintenance

### Issue Tracking

- Use GitHub Issues
- Label: bug, enhancement, question, documentation
- Create issue templates

### Community

- GitHub Discussions for Q&A
- Discord server (optional)
- Twitter for updates

### Analytics (Optional)

- Track downloads from GitHub releases
- Monitor crates.io download stats
- Google Analytics for documentation site

---

## Legal Considerations

- **License**: MIT OR Apache-2.0 (Rust standard)
- **Trademark**: Consider registering "Ownsight" if it becomes popular
- **Privacy**: Desktop app doesn't collect data
- **Code Signing**: Required for macOS/Windows distribution

---

## Success Metrics

- **CLI**: crates.io downloads
- **Desktop**: GitHub release downloads
- **Community**: GitHub stars, forks, issues
- **Adoption**: Blog posts, tutorials, mentions

Good luck with your launch! 🚀
