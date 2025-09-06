#!/usr/bin/env bash
set -euo pipefail

echo "[1/2] Building (debug)..."
cargo build

echo "[2/2] Running jpg2ascii on samples/baozou.jpg"
./target/debug/jpg2ascii samples/baozou.jpg | sed -n '1,20p'

echo "[ok] Command executed successfully."

