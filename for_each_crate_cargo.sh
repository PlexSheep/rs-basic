#!/bin/bash
set -e
ROOT=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
CRATES=$(ls ./crates)

echo "$CRATES"

for crate in $CRATES; do
	THE_CMD="$ROOT/cargo_crate.sh $crate $@";
	echo "$THE_CMD"
	eval "$THE_CMD"
done;
