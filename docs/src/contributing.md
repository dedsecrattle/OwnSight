# Contributing

We welcome contributions to Ownsight! This guide will help you get started.

## Getting Started

### Prerequisites
- Rust (stable for Layer 1, nightly for Layer 2)
- Node.js and Bun (for desktop app)
- Git

### Development Setup

```bash
# Clone the repository
git clone https://github.com/dedsecrattle/ownsight
cd ownsight

# Desktop app
cd ui
bun install
bun run tauri dev

# CLI tool
cargo run --bin ownsight-cli -- --file examples/hello.rs
```

## Project Structure

```
ownsight/
├── crates/
│   ├── ownsight-core/      # Core data model
│   ├── ownsight-driver/    # Analysis driver
│   ├── ownsight-mir/       # MIR backend (Layer 2)
│   └── ownsight-cli/       # CLI tool
├── ui/                     # Desktop app (Tauri + React)
└── docs/                   # Documentation
```

## How to Contribute

### Reporting Bugs

1. Check existing [GitHub Issues](https://github.com/dedsecrattle/ownsight/issues)
2. Create a new issue with:
   - Clear title
   - Steps to reproduce
   - Expected vs actual behavior
   - Version and OS information

### Suggesting Features

1. Check [GitHub Discussions](https://github.com/dedsecrattle/ownsight/discussions)
2. Describe the feature and use case
3. Explain why it would be valuable

### Code Contributions

1. **Fork the repository**
2. **Create a branch**: `git checkout -b feature/your-feature`
3. **Make changes** following our coding standards
4. **Test your changes**
5. **Commit**: `git commit -m "Add feature: description"`
6. **Push**: `git push origin feature/your-feature`
7. **Create Pull Request**

## Development Guidelines

### Code Style

- Follow Rust standard formatting: `cargo fmt`
- Run clippy: `cargo clippy`
- Add tests for new features
- Document public APIs

### Testing

```bash
# Run all tests
cargo test

# Test specific crate
cargo test -p ownsight-core

# Test with MIR backend
cargo +nightly test -p ownsight-mir --features rustc
```

### Documentation

- Update docs for new features
- Add examples where helpful
- Keep README.md current

## Areas to Contribute

### Layer 1 (Simple Backend)
- Improve pattern detection
- Add more event types
- Better error messages
- Performance optimizations

### Layer 2 (MIR Backend)
- Implement stub analyzers
- Add Polonius integration
- Cross-function analysis
- Async/await improvements

### Desktop App
- UI/UX improvements
- New visualization types
- Keyboard shortcuts
- Export features

### CLI Tool
- Output formats
- Integration with other tools
- Performance improvements

### Documentation
- More examples
- Video tutorials
- Blog posts
- Translations

## Pull Request Process

1. **Update documentation** if needed
2. **Add tests** for new functionality
3. **Ensure CI passes** (formatting, tests, clippy)
4. **Request review** from maintainers
5. **Address feedback** promptly

## Code of Conduct

- Be respectful and inclusive
- Welcome newcomers
- Focus on constructive feedback
- Help others learn

## License

By contributing, you agree that your contributions will be licensed under MIT OR Apache-2.0.

## Questions?

- Open a [GitHub Discussion](https://github.com/dedsecrattle/ownsight/discussions)
- Check the [documentation](https://dedsecrattle.github.io/ownsight/)
- Ask in pull request comments