@echo off

cd ..

cd common || exit /b
cargo clean || exit /b
cd ..

cd server || exit /b
cargo clean || exit /b
cd ..

cd worker || exit /b
cargo clean || exit /b
del /S /Q pkg || exit /b
cd ..

cd client || exit /b
cargo clean || exit /b
del /S /Q pkg || exit /b
cd ..

cd client-native || exit /b
cargo clean || exit /b
cd ..

cd www || exit /b
del worker_wasm.js || exit /b
del worker_wasm_bg.wasm || exit /b
del client.js || exit /b
del client_bg.wasm || exit /b
cd ..

del app_server.exe || exit /b
del app_client.exe || exit /b

cd windows || exit /b
