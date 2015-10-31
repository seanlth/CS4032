#!/bin/bash
clang -c src/get_host.c
ar rcs libhost.a get_host.o
cp libhost.a target/debug/deps/
rm libhost.a get_host.o
cargo build
