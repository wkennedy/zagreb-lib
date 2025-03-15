#!/bin/bash
set -e

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "Error: wasm-pack is not installed."
    echo "Please install it using: cargo install wasm-pack"
    exit 1
fi

# Build the WASM package
echo "Building WASM package..."
wasm-pack build --target web --out-dir pkg/web

# If building for Node.js is also needed
echo "Building Node.js package..."
wasm-pack build --target nodejs --out-dir pkg/node

# Copy web example files to a dist directory
echo "Setting up web example..."
mkdir -p dist
cp -r examples/web/* dist/
cp -r pkg/web/* dist/

echo "Build completed successfully!"
echo "To test the web example, serve the 'dist' directory with a web server."
echo "For example: npx http-server dist"