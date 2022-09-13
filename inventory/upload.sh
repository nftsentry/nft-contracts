#!/bin/bash

if [ "$1" == "--release" ];
then
  gsutil cp -a public-read res/inventory.wasm gs://kibernetika-dst/inventory.wasm
fi
gsutil cp -a public-read res/inventory.wasm gs://kibernetika-dst/inventory_dev.wasm

