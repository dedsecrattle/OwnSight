# Publishing Checklist for Ownsight

## Quick Summary

**CLI Tool**: Publish to crates.io → Users install with `cargo install ownsight-cli`

**Desktop App**: Build installers for macOS/Linux/Windows → Distribute via GitHub Releases

---

## Step-by-Step Publishing Guide

### Phase 1: Prepare Repository (One-time setup)

- [ ] Create GitHub repository
- [ ] Add LICENSE-MIT and LICENSE-APACHE files
- [ ] Update all Cargo.toml files with proper metadata:
  - [ ] `crates/ownsight-core/Cargo.toml`
  - [ ] `crates/ownsight-driver/Cargo.toml`
  - [ ] `crates/ownsight-cli/Cargo.toml`
  - [ ] `ui/src-tauri/Cargo.toml`
- [ ] Create demo GIF/screenshots
- [ ] Write comprehensive README.md
- [ ] Set up GitHub Actions (`.github/workflows/release.yml`)

### Phase 2: Publish CLI to crates.io

**Prerequisites**:
```bash
# 1. Create account at https://crates.io/
# 2. Get API token from https://crates.io/me
# 3. Login
cargo login <your-token>
```

**Publish**:
```bash
# Use the automated script
./scripts/publish-cli.sh

# Or manually:
cd crates/ownsight-core && cargo publish
cd ../ownsight-driver && cargo publish
cd ../ownsight-cli && cargo publish
```

**Verify**:
```bash
# Wait a few minutes, then test
cargo install ownsight-cli
cargo ownership-viz --help
```

### Phase 3: Build Desktop App

**macOS**:
```bash
cd ui
bun run tauri build

# Output: ui/src-tauri/target/release/bundle/dmg/Ownsight_0.1.0_x64.dmg
```

**Linux** (on Ubuntu/Debian):
```bash
cd ui
bun run tauri build

# Output:
# - ui/src-tauri/target/release/bundle/deb/ownsight_0.1.0_amd64.deb
# - ui/src-tauri/target/release/bundle/appimage/ownsight_0.1.0_amd64.AppImage
```

**Windows** (on Windows):
```bash
cd ui
bun run tauri build

# Output: ui/src-tauri/target/release/bundle/msi/Ownsight_0.1.0_x64_en-US.msi
```

### Phase 4: Create GitHub Release

**Option A: Automated (Recommended)**
```bash
# 1. Tag version
git tag -a v0.1.0 -m "Release v0.1.0"
git push origin v0.1.0

# 2. GitHub Actions automatically builds and creates release
# 3. Go to GitHub Releases and publish the draft
```

**Option B: Manual**
```bash
# 1. Build on each platform (macOS, Linux, Windows)
# 2. Go to GitHub → Releases → Create new release
# 3. Upload installers:
#    - Ownsight_0.1.0_x64.dmg (macOS)
#    - ownsight_0.1.0_amd64.AppImage (Linux)
#    - Ownsight_0.1.0_x64_en-US.msi (Windows)
# 4. Write release notes
# 5. Publish
```

---

## Pre-Release Checklist

### Code Quality
- [ ] All tests pass: `cargo test --workspace`
- [ ] No clippy warnings: `cargo clippy --workspace`
- [ ] Code formatted: `cargo fmt --all`
- [ ] No compiler warnings

### Documentation
- [ ] README.md is complete and accurate
- [ ] CHANGELOG.md updated with release notes
- [ ] All public APIs documented
- [ ] Examples work correctly

### Testing
- [ ] CLI tested on sample files
- [ ] Desktop app tested on target platforms
- [ ] All features demonstrated in screenshots/GIFs
- [ ] Installation instructions verified

### Metadata
- [ ] Version numbers updated everywhere:
  - [ ] All Cargo.toml files
  - [ ] package.json
  - [ ] tauri.conf.json
- [ ] Authors and contact info correct
- [ ] Repository URLs correct
- [ ] Keywords and categories appropriate

---

## Post-Release Checklist

### Verification
- [ ] CLI installs correctly: `cargo install ownsight-cli`
- [ ] Desktop app downloads work
- [ ] All download links functional
- [ ] Documentation links work

### Announcements
- [ ] Post to r/rust
- [ ] Post to r/learnrust
- [ ] Tweet with #rustlang hashtag
- [ ] Submit to "This Week in Rust"
- [ ] Post on Hacker News (Show HN)
- [ ] Update personal website/portfolio

### Monitoring
- [ ] Watch GitHub issues for bug reports
- [ ] Monitor crates.io download stats
- [ ] Track GitHub release downloads
- [ ] Respond to community feedback

---

## Quick Commands Reference

### Prepare Release
```bash
./scripts/prepare-release.sh 0.1.0
```

### Publish CLI
```bash
./scripts/publish-cli.sh
```

### Build Desktop App
```bash
cd ui && bun run tauri build
```

### Create Git Tag
```bash
git tag -a v0.1.0 -m "Release v0.1.0"
git push origin v0.1.0
```

### Test Installation
```bash
# CLI
cargo install ownsight-cli
cargo ownership-viz --version

# Desktop (download from releases)
# macOS: Open .dmg and drag to Applications
# Linux: chmod +x *.AppImage && ./ownsight*.AppImage
# Windows: Run .msi installer
```

---

## Distribution Channels

### CLI Tool
- **Primary**: crates.io
- **Installation**: `cargo install ownsight-cli`
- **Updates**: Users run `cargo install --force ownsight-cli`

### Desktop App
- **Primary**: GitHub Releases
- **macOS**: .dmg installer
- **Linux**: .AppImage (universal) or .deb (Debian/Ubuntu)
- **Windows**: .msi installer
- **Updates**: Manual download (auto-update can be added later)

### Future Channels
- **Homebrew** (macOS): `brew install ownsight`
- **Snap Store** (Linux): `snap install ownsight`
- **Chocolatey** (Windows): `choco install ownsight`
- **Flathub** (Linux): `flatpak install ownsight`

---

## Versioning Strategy

Follow [Semantic Versioning](https://semver.org/):

- **0.1.0**: Initial release (current)
- **0.1.x**: Bug fixes and patches
- **0.2.0**: MIR-based analysis
- **0.3.0**: Advanced features (closures, async)
- **0.4.0**: Workspace support
- **0.5.0**: Editor integration
- **1.0.0**: Production-ready, stable API

---

## Support Resources

### For Users
- GitHub Issues: Bug reports and feature requests
- GitHub Discussions: Q&A and community support
- Documentation: README.md and inline docs
- Examples: `tests/snapshots/` directory

### For Contributors
- CONTRIBUTING.md: Contribution guidelines
- ARCHITECTURE.md: System design
- Code comments: Implementation details
- Issue labels: good-first-issue, help-wanted

---

## Emergency Procedures

### Yank a Bad Release (CLI)
```bash
cargo yank --vers 0.1.0 ownsight-cli
```

### Delete a Bad Release (Desktop)
```bash
# Go to GitHub Releases
# Click on the release
# Click "Delete"
```

### Hotfix Process
```bash
# 1. Fix the bug
# 2. Bump patch version (0.1.0 → 0.1.1)
# 3. Follow normal release process
# 4. Announce the fix
```

---

## Success Metrics

Track these to measure adoption:

- **CLI**: crates.io download count
- **Desktop**: GitHub release download count
- **Community**: GitHub stars, forks, watchers
- **Engagement**: Issues opened, PRs submitted
- **Reach**: Blog posts, tweets, mentions

---

## Next Steps After v0.1.0

1. **Gather Feedback**: Monitor issues and discussions
2. **Fix Bugs**: Address critical issues quickly
3. **Plan v0.2.0**: MIR integration and enhanced analysis
4. **Build Community**: Respond to users, accept PRs
5. **Improve Docs**: Based on user questions
6. **Add Features**: According to roadmap

---

## Resources

- **Publishing Guide**: See PUBLISHING.md for detailed instructions
- **Changelog**: See CHANGELOG.md for version history
- **Architecture**: See ARCHITECTURE.md for technical details
- **Quick Start**: See QUICKSTART.md for usage examples

Good luck with your release! 🚀
