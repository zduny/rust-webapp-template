#!/bin/sh
set -e

echo "Building common..."
cd common
cargo build --release
cd ..

echo "\nBuilding server..."
cd server
cargo build --release
cd ..

echo "\nBuilding client..."
cd client
wasm-pack build --release --target web
cd ..

cp client/pkg/client.js www/client.js
cp client/pkg/client_bg.wasm www/client_bg.wasm
