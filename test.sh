#!/bin/sh
set -e

echo "Testing common..."
cd common
cargo test
cd ..

echo "\nTesting server..."
cd server
cargo test
cd ..

echo "\nTesting worker..."
cd worker
wasm-pack test --headless --chrome
cd ..

echo "\nTesting client..."
cd client
wasm-pack test --headless --chrome
cd ..

echo "\nTesting native client..."
cd client-native
cargo test
cd ..
