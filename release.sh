#!/usr/bin/env bash

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
cd $SCRIPT_DIR
mkdir -p release

for target in $(rustup target list | sed 's/ (installed)//g'); do
    rustup target add $target
    cargo build --release --target $target
    mv target/$target/release/albion-island-balancer release/albion-island-balancer_$target
    rm -rf target/$target
done
