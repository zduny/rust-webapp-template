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
cargo test --target wasm32-unknown-unknown
cd ..

echo "\nTesting client..."
cd client
cargo test --target wasm32-unknown-unknown
cd ..

echo "\nTesting native client..."
cd client-native
cargo test
cd ..
