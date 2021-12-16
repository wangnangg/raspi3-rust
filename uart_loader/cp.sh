#!/usr/bin/env bash
set -euo pipefail

cargo build --release
llvm-objcopy -O binary target/aarch64/release/uart_loader target/aarch64/kernel8.img
cp ./target/aarch64/kernel8.img /run/media/nanwa/boot/
