#!/bin/bash

set -e

pushd ./kernel
./build.sh
popd

pushd ./uart_host
cargo run -- ../kernel/target/aarch64/release/kernel.img
popd


