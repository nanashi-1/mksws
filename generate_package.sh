#!/bin/bash

#cargo build --release

mkdir -p build/root/usr/bin
cp target/release/mksws build/root/usr/bin

cd build
tar -czf "../mksws-1.0.tar.gz" *