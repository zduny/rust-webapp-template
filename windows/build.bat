@echo off

cd ..

echo Building common...
cd common || exit /b
cargo build --release || exit /b
cd ..

echo:
echo Building server...
cd server || exit /b
cargo build --release || exit /b
cd ..

echo:
echo Building worker...
cd worker || exit /b
wasm-pack build --release --target no-modules || exit /b
cd ..

echo:
echo Building client...
cd client || exit /b
wasm-pack build --release --target web || exit /b
cd ..

echo:
echo Building native client...
cd client-native || exit /b
cargo build --release || exit /b
cd ..

copy "worker\pkg\worker.js" "www\worker_wasm.js" || exit /b
copy "worker\pkg\worker_bg.wasm" "www\worker_wasm_bg.wasm" || exit /b

copy "client\pkg\client.js" "www\client.js" || exit /b
copy "client\pkg\client_bg.wasm" "www\client_bg.wasm" || exit /b

copy "server\target\release\server.exe" ".\app_server.exe" || exit /b
copy "client-native\target\release\client-native.exe" ".\app_client.exe" || exit /b

cd windows || exit /b
