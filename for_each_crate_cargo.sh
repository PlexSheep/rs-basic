#!/bin/bash
ROOT=$PWD
DIRS=$(ls ./crates)

echo $DIRS

for dir in $DIRS; do
	cd crates/$dir;
	eval "bash $ROOT/cargo.sh $@";
	cd $ROOT;
done;
