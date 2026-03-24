# Installation

## Desktop App

### macOS
1. Download the `.dmg` file from [GitHub Releases](https://github.com/dedsecrattle/ownsight/releases/latest)
2. Open the `.dmg` file
3. Drag Ownsight to Applications folder
4. Launch from Applications

### Linux
1. Download the `.AppImage` file from [GitHub Releases](https://github.com/dedsecrattle/ownsight/releases/latest)
2. Make it executable: `chmod +x Ownsight*.AppImage`
3. Run: `./Ownsight*.AppImage`

### Windows
1. Download the `.msi` installer from [GitHub Releases](https://github.com/dedsecrattle/ownsight/releases/latest)
2. Run the installer
3. Follow the installation wizard
4. Launch from Start Menu

## CLI Tool

### From crates.io
```bash
cargo install ownsight-cli
```

### From Source
```bash
git clone https://github.com/dedsecrattle/ownsight
cd ownsight
cargo install --path crates/ownsight-cli
```

## Building from Source

### Prerequisites
- Rust (stable or nightly)
- Node.js and Bun (for desktop app)
- Git

### Desktop App
```bash
git clone https://github.com/dedsecrattle/ownsight
cd ownsight/ui
bun install
bun run tauri dev
```

### CLI Tool
```bash
git clone https://github.com/dedsecrattle/ownsight
cd ownsight
cargo build --release --bin ownsight-cli
```

## MIR Backend (Optional)

For advanced Layer 2 features, see [Layer 2: MIR Backend](usage/layer2.md).