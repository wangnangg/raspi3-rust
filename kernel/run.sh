#!/bin/bash
set -x
ELF=`realpath $1`
BIN=${ELF}.img
llvm-objcopy -O binary $ELF $BIN
cd ../uart_host && cargo run -- $BIN
#screen /dev/ttyUSB0 115200
