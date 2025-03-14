#!/bin/bash
set -e
ROOT=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
DIRS=$(ls ./crates)

echo $DIRS

for dir in $DIRS; do
	cd crates/$dir;
	eval "$@";
	cd $ROOT;
done;
