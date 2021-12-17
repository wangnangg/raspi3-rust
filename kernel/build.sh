#!/usr/bin/env bash
set -euo pipefail
set -x
rm -f target/aarch64/release/kernel*
rm -f target/aarch64/release/deps/kernel-*
cargo build --release -vvv
llvm-objcopy -O binary target/aarch64/release/kernel target/aarch64/release/kernel.img
