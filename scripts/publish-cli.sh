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
cargo publish
echo "⏳ Waiting for crates.io to index..."
sleep 30

echo "2/3 Publishing ownsight-driver..."
cd ../ownsight-driver
cargo publish
echo "⏳ Waiting for crates.io to index..."
sleep 30

echo "3/3 Publishing ownsight-cli..."
cd ../ownsight-cli
cargo publish

cd ../..

echo "✅ All crates published successfully!"
echo ""
echo "Users can now install with:"
echo "  cargo install ownsight-cli"
