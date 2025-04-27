#!/bin/bash

set -e

# 1. Build WASM release
echo "🚀 Building release pro wasm32-unknown-unknown..."
cargo build --release --target wasm32-unknown-unknown

# 2. Vytvoření čisté složky 'dist'
echo "🧹 Čistím / vytvářím dist/ složku..."
rm -rf dist
mkdir dist

# 3. Zkopírování potřebných souborů
echo "📦 Kopíruji build output do dist/"
cp target/wasm32-unknown-unknown/release/macroquad_snake.wasm dist/
cp js/index.html dist/
cp js/mq_js_bundle.js dist/
cp -r assets dist/

# 4. Spuštění HTTP serveru nad dist/
echo "🌐 Spouštím basic-http-server v dist/"
basic-http-server ./dist
