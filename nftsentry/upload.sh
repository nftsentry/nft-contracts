#!/bin/bash

if [ "$1" == "--release" ];
then
  gsutil cp -a public-read res/nftsentry.wasm gs://kibernetika-dst/nftsentry.wasm
fi
gsutil cp -a public-read res/nftsentry.wasm gs://kibernetika-dst/nftsentry_dev.wasm
#gsutil acl ch -u AllUsers:R gs://kibernetika-dst/nftsentry.wasm
