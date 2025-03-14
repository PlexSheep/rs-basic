#!/bin/bash
set -e

ROOT=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

CARGO_TARGET_DIR=$ROOT/target cargo $@
