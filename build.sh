#!/bin/sh

mkdir -p bin
cargo build --release --target-dir build/

cd auth
cargo build --release --target-dir ../build/

cd ..
cd static-fs
cargo build --release --target-dir ../build/

cd ..

cd api
cargo build --release --target-dir ../build/

cd ..

cp --update=all build/release/hyrebet bin/
cp --update=all build/release/auth bin/
cp --update=all build/release/api bin/
cp --update=all build/release/static-fs bin/
