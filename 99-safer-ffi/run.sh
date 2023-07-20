#!/bin/bash

set -e

# 1. Build a shared library written Rust => libffi_101.so
cd rust && cargo build
cp ./target/debug/libffi_101.so ../ && cd -

# 2. Build an executable written C++ => ffi_101
g++ cpp/main.cpp -o ffi -lffi_101 -L ./ -Wl,-rpath .

# 3. Run the executable
./ffi
