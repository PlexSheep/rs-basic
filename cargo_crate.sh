#!/bin/bash

CRATE="$1"
ROOT=$PWD
CARGO_TARGET_DIR=$PWD/target 

echo "using crate $CRATE"
cargo --config "target-dir=\"$ROOT/target\"" ${@:2} --manifest-path $ROOT/crates/${CRATE}/Cargo.toml 
