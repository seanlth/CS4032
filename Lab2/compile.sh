#!/bin/bash
clang -c src/get_host.c
ar rcs libhost.a get_host.o
cp src/libhost.a target/debug/deps/
rm src/libhost.a src/get_host.o
cargo build
