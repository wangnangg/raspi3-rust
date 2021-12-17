#!/usr/bin/env bash
set -euo pipefail
set -x
cargo build --release
llvm-objcopy -O binary target/aarch64/release/kernel target/aarch64/release/kernel.img
