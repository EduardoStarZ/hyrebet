#!/bin/sh

mkdir -p dev

cargo build --all --target-dir=build

cp --update=all build/release/hyrebet dev/
cp --update=all build/release/auth dev/
cp --update=all build/release/api dev/
cp --update=all build/release/static-fs dev/
