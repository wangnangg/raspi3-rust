#!/bin/bash

pushd ../kernel
./build.sh
popd

cargo run -- ../kernel/target/aarch64/release/kernel.img


