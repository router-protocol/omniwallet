#!/bin/bash

# exit when any command fails
set -e

if [ $# -ne 0 ] 
then
    echo "Invalid Arguments"
    echo "sh build.sh"
fi

# building the wasm artifacts
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="${pwd}",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/workspace-optimizer:0.12.10

echo "wasm build created"