#!/bin/bash
set -e

CRATE="$1"
ROOT=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

echo "using crate $CRATE"
"$ROOT/cargo.sh" ${@:2} --manifest-path "$ROOT"/crates/"${CRATE}"/Cargo.toml
