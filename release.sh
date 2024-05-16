#!/usr/bin/env bash

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
cd $SCRIPT_DIR
mkdir -p release

targets=( "x86_64-unknown-linux-gnu" "i686-unknown-linux-gnu" )

if which i686-w64-mingw32-gcc &> /dev/null; then
    targets+=("i686-pc-windows-gnu")
else 
    echo "Missing i686-w64-mingw32-gcc, skipping i686-pc-windows-gnu..."
fi

if which x86_64-w64-mingw32-gcc &> /dev/null; then
    targets+=("x86_64-pc-windows-gnu")
else
    echo "Missing x86_64-w64-mingw32-gcc, skipping x86_64-pc-windows-gnu..."
fi

for target in ${targets[@]}; do
    rustup target add $target
    cargo build --release --target $target
    if test -f target/$target/release/albion-island-balancer; then
        cp target/$target/release/albion-island-balancer release/albion-island-balancer
        zip -m release/albion-island-balancer_$target.zip release/albion-island-balancer
        rm release/albion-island-balancer
    else
        cp target/$target/release/albion-island-balancer.exe release/albion-island-balancer.exe
        zip -m release/albion-island-balancer_$target.zip release/albion-island-balancer.exe
        rm release/albion-island-balancer.exe
    fi
done
