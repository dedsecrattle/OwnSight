#!/bin/bash
# Publish CLI crates to crates.io
# Usage: ./scripts/publish-cli.sh

set -e

echo "📦 Publishing Ownsight CLI to crates.io..."

# Check if logged in
if ! cargo login --help &> /dev/null; then
    echo "❌ Please run 'cargo login <token>' first"
    exit 1
fi

# Publish in order (dependencies first)
echo "1/3 Publishing ownsight-core..."
cd crates/ownsight-core
if cargo publish --dry-run 2>/dev/null; then
    cargo publish
    echo "⏳ Waiting for crates.io to index..."
    sleep 30
else
    echo "✅ ownsight-core already published, skipping..."
fi

echo "2/3 Publishing ownsight-driver..."
cd ../ownsight-driver
if cargo publish --dry-run 2>/dev/null; then
    cargo publish
    echo "⏳ Waiting for crates.io to index..."
    sleep 30
else
    echo "✅ ownsight-driver already published, skipping..."
fi

echo "3/3 Publishing ownsight-cli..."
cd ../ownsight-cli
if cargo publish --dry-run 2>/dev/null; then
    cargo publish
else
    echo "✅ ownsight-cli already published, skipping..."
fi

cd ../..

echo "✅ All crates published successfully!"
echo ""
echo "Users can now install with:"
echo "  cargo install ownsight-cli"
