# Changelog

All notable changes to Ownsight will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Initial implementation of Ownsight
- Core analysis engine with ownership event tracking
- Simple syntax-based Rust parser
- CLI tool with multiple output formats (JSON, timeline, text)
- Tauri desktop application with React UI
- Monaco code editor with syntax highlighting
- Interactive timeline view with step-by-step events
- Graph visualization using React Flow
- Query interface for debugging questions
- Teaching and debug modes
- Support for basic ownership patterns (moves, borrows, drops)

### Features

#### CLI (`ownsight-cli`)

- Analyze Rust code from files or stdin
- Output formats: JSON, timeline, text
- Teaching vs debug mode explanations
- Colored terminal output

#### Desktop App

- Interactive code editor
- Timeline view with event icons and explanations
- Graph view showing ownership relationships
- Step controller (play/pause/speed control)
- Query panel for asking ownership questions
- Real-time analysis

#### Core Engine (`ownsight-core`)

- Variable tracking (name, type, scope, mutability)
- Event extraction (Create, Move, Borrow, Drop, etc.)
- Ownership graph generation
- Query interface for debugging
- Human-readable explanations

## [0.1.0] - TBD

### Initial Release

First public release of Ownsight - Rust Ownership Visualizer.

**Layer 1 Features** (Learning & Exam Tool):

- ✨ Interactive timeline visualization
- 📊 Source code highlighting
- 🎯 Teaching mode with simplified explanations
- 🔄 Step-by-step playback controls

**What's Included**:

- CLI tool for terminal-based analysis
- Desktop application for interactive visualization
- Support for basic ownership patterns
- Example snippets and documentation

**Known Limitations**:

- Uses simple syntax-based parser
- Limited to single functions
- Does not support: closures, async/await, complex pattern matching
- No cargo workspace integration yet

**Coming in Layer 2**:

- Enhanced analysis capabilities
- Full cargo workspace support
- Advanced features (closures, async, NLL)
- VS Code extension
- Enhanced debugging capabilities

---

## Version History

### Versioning Strategy

- **0.x.x**: Pre-1.0 releases, API may change
- **1.x.x**: Stable API, production-ready
- **x.0.0**: Major features or breaking changes
- **x.x.0**: New features, backward compatible
- **x.x.x**: Bug fixes and patches

### Roadmap

**v0.2.0** - Enhanced Features

- Enhance simple parser with improved analysis
- Accurate borrow checking
- Support for more Rust constructs

**v0.3.0** - Advanced Features

- Partial moves (struct fields)
- Pattern matching flows
- Non-lexical lifetimes (NLL)
- Closure capture analysis

**v0.4.0** - Workspace Support

- Multi-crate analysis
- Cargo integration
- Function summaries
- Cross-crate ownership tracking

**v0.5.0** - Editor Integration

- VS Code extension
- Rust Analyzer integration
- Inline ownership hints
- Real-time analysis

**v1.0.0** - Production Ready

- Stable API
- Comprehensive documentation
- Full test coverage
- Performance optimizations
- Web version (WASM)

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on how to contribute to Ownsight.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.
