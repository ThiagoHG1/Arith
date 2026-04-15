#!/bin/bash

set -e

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"

echo "Building Arith..."

cargo build --release

echo "Installing..."

sudo cp "$ROOT_DIR/target/release/Arith" /usr/local/bin/Arith

sudo chmod +x /usr/local/bin/Arith

echo "Done."
