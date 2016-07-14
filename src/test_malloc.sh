#!/bin/sh
rm -f ./a.out;gcc example.c;LD_PRELOAD=../target/debug/libsimple_malloc.so ./a.out;rm a.out
