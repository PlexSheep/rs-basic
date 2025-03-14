#!/bin/bash
root=$PWD
DIRS=$(ls ./crates)

echo $DIRS

for dir in $DIRS; do
	cd crates/$dir;
	eval "$@";
	cd $root;
done;
