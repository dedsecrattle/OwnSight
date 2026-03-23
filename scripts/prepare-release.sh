#!/bin/bash
# Prepare Ownsight for release
# Usage: ./scripts/prepare-release.sh <version>

set -e

VERSION=${1:-"0.1.0"}

echo "🚀 Preparing Ownsight v$VERSION for release..."

# Update version in all Cargo.toml files
echo "📝 Updating versions..."
sed -i '' "s/^version = \".*\"/version = \"$VERSION\"/" crates/ownsight-core/Cargo.toml
sed -i '' "s/^version = \".*\"/version = \"$VERSION\"/" crates/ownsight-driver/Cargo.toml
sed -i '' "s/^version = \".*\"/version = \"$VERSION\"/" crates/ownsight-cli/Cargo.toml
sed -i '' "s/^version = \".*\"/version = \"$VERSION\"/" ui/src-tauri/Cargo.toml
sed -i '' "s/\"version\": \".*\"/\"version\": \"$VERSION\"/" ui/package.json

# Run tests
echo "🧪 Running tests..."
cargo test --workspace --quiet

# Run clippy
echo "🔍 Running clippy..."
cargo clippy --workspace --quiet -- -D warnings

# Format code
echo "✨ Formatting code..."
cargo fmt --all

# Build everything
echo "🔨 Building workspace..."
cargo build --workspace --release --quiet

# Build desktop app
echo "🖥️  Building desktop app..."
cd ui
bun install --silent
bun run tauri build --quiet
cd ..

echo "✅ Release preparation complete!"
echo ""
echo "Next steps:"
echo "1. Review CHANGELOG.md and add release notes"
echo "2. Commit changes: git commit -am 'Release v$VERSION'"
echo "3. Tag release: git tag -a v$VERSION -m 'Release v$VERSION'"
echo "4. Push: git push origin main --tags"
echo "5. Publish CLI: ./scripts/publish-cli.sh"
echo "6. Create GitHub release with desktop app binaries"
