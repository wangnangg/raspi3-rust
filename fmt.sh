#!/bin/bash

dirs="rpb3_lib uart_host uart_loader"
for d in $dirs
do
	pushd $d
	cargo fmt
	popd
done
