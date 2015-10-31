#!/bin/bash
clang -c src/get_host.c
ar rcs libhost.a src/get_host.o
cp libhost.a target/debug/deps/
rm libhost.a src/get_host.o
cargo build
