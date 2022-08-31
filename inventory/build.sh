#!/bin/bash
set -e
cd "`dirname $0`"
source flags.sh

cargo build --lib --target wasm32-unknown-unknown --release
mkdir -p ./res
cp -v target/wasm32-unknown-unknown/release/*.wasm ./res/

wasm_opt=$(which wasm-opt)

if [ ! -z $wasm_opt ]; then
  echo "Optimize for size with wasm-opt..."
  old_size=$(ls -l res/inventory.wasm | awk '{print $5}')
  wasm-opt -Os -o res/os.wasm res/inventory.wasm
  chmod +x res/os.wasm
  mv res/os.wasm res/inventory.wasm
  new_size=$(ls -l res/inventory.wasm | awk '{print $5}')
  echo "Optimized size $old_size -> $new_size"
fi

