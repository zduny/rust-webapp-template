#!/bin/sh
set -e

sleep 1 && open http://localhost:8080 &
./app_server
