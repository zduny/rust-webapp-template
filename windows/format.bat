@echo off

cd ..

echo Formatting common...
cd common || exit /b
cargo fmt || exit /b
cd ..

echo Formatting server...
cd server || exit /b
cargo fmt || exit /b
cd ..

echo Formatting worker...
cd worker || exit /b
cargo fmt || exit /b
cd ..

echo Formatting client...
cd client || exit /b
cargo fmt || exit /b
cd ..

echo Formatting native client...
cd client-native || exit /b
cargo fmt || exit /b
cd ..

echo Done.

cd windows || exit /b
