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
  old_size=$(ls -l res/nftsentry.wasm | awk '{print $5}')
  wasm-opt -Os -o res/os.wasm res/nftsentry.wasm
  chmod +x res/os.wasm
  mv res/os.wasm res/nftsentry.wasm
  new_size=$(ls -l res/nftsentry.wasm | awk '{print $5}')
  echo "Optimized size $old_size -> $new_size"
fi

