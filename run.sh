#!/bin/sh
set -e

rm -f app_server
cp server/target/release/server ./app_server
sleep 1 && open http://localhost:8080 &
./app_server
