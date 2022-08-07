#!/bin/sh
set -e

cd common
cargo clean
cd ..

cd server
cargo clean
cd ..

cd client
cargo clean
rm -rf pkg
cd ..

cd www
rm -f client.js
rm -f client_bg.wasm
cd ..

rm -f app_server
