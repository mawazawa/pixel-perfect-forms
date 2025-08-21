#!/bin/bash
#
# ███████╗██╗███╗   ███╗██████╗ ██╗     ███████╗    ██████╗ ██╗   ██╗██╗██╗     ██████╗ 
# ██╔════╝██║████╗ ████║██╔══██╗██║     ██╔════╝    ██╔══██╗██║   ██║██║██║     ██╔══██╗
# ███████╗██║██╔████╔██║██████╔╝██║     █████╗      ██████╔╝██║   ██║██║██║     ██║  ██║
# ╚════██║██║██║╚██╔╝██║██╔═══╝ ██║     ██╔══╝      ██╔══██╗██║   ██║██║██║     ██║  ██║
# ███████║██║██║ ╚═╝ ██║██║     ███████╗███████╗    ██████╔╝╚██████╔╝██║███████╗██████╔╝
# ╚══════╝╚═╝╚═╝     ╚═╝╚═╝     ╚══════╝╚══════╝    ╚═════╝  ╚═════╝ ╚═╝╚══════╝╚═════╝ 
#                                                        simple-build.sh

set -e

echo "🦀 Building FL-100 Foundation Layer with Rust+WASM..."

# Source Rust environment (install if needed)
if [ ! -f "$HOME/.cargo/env" ]; then
    echo "🦀 Installing Rust for Vercel environment..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
else
    source "$HOME/.cargo/env"
fi

# Add WASM target if not already added
rustup target add wasm32-unknown-unknown

cd app

# Build for WASM target with maximum optimization
echo "📦 Building WASM module with maximum optimization..."
cargo build --target wasm32-unknown-unknown --release

# Copy the optimized WASM file to dist
echo "🎯 Copying optimized WASM files..."
cp ../target/wasm32-unknown-unknown/release/app.wasm dist/fl100_foundation.wasm

# Additional optimization with wasm-opt if available  
if command -v wasm-opt &> /dev/null; then
    echo "🚀 Further optimizing WASM with wasm-opt..."
    wasm-opt -Oz --enable-bulk-memory dist/fl100_foundation.wasm -o dist/fl100_foundation_opt.wasm
    mv dist/fl100_foundation_opt.wasm dist/fl100_foundation.wasm
    echo "✅ WASM size after optimization: $(ls -lah dist/fl100_foundation.wasm | awk '{print $5}')"
else
    echo "⚠️  wasm-opt not available, using cargo release optimization only"
fi

# Create dist directory
mkdir -p dist

# Copy static files
cp -r static/* dist/

# Copy HTML
cp index.html dist/

# Install and use Trunk for proper WASM build
echo "📦 Installing Trunk for WASM compilation..."
cargo install trunk --version 0.20.3 2>/dev/null || echo "Trunk already installed"

# Add wasm-pack if needed
cargo install wasm-pack 2>/dev/null || echo "wasm-pack already installed"

# Build with Trunk (proper WASM compilation)
echo "🦀 Building with Trunk for production..."
trunk build --release --dist dist

echo "✅ Build complete! Dist ready for deployment."
echo "📁 Built files in: app/dist/"
ls -la dist/