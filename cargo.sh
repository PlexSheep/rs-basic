#!/bin/bash

ROOT=$PWD
CARGO_TARGET_DIR=$PWD/target 

cargo --config "target-dir=\"$ROOT/target\"" $@
