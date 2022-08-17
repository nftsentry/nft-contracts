#!/bin/bash
set -e
cd "`dirname $0`"
source flags.sh

cargo build --lib --target wasm32-unknown-unknown --release
mkdir -p ./res
cp -v target/wasm32-unknown-unknown/release/*.wasm ./res/

