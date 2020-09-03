#!/usr/bin/sh

cargo bootimage
qemu-system-x86_64 -drive format=raw,file=target/x86_64-krill/debug/bootimage-krill.bin -S -s -daemonize
