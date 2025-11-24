#!/bin/sh

mkdir -p dev

cargo build --all --target-dir=build

cp --update=all build/debug/vue-launcher dev/
cp --update=all build/debug/hyrebet dev/
cp --update=all build/debug/auth dev/
cp --update=all build/debug/api dev/
cp --update=all build/debug/static-fs dev/
