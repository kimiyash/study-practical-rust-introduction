#!/bin/sh
clang -c -o target/debug/native/fib.o c_src/fib.c
ar r target/debug/deps/libfib.a target/debug/native/fib.o
