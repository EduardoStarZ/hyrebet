#!/bin/sh

mkdir -p bin
cargo build --all --release --target-dir build/

cp --update=all build/release/hyrebet .bin/
cp --update=all build/release/auth .bin/
cp --update=all build/release/api .bin/
cp --update=all build/release/static-fs .bin/
