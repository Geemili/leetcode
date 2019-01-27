#!/usr/bin/env bash

target="target"
file=$1
outfile="$target/${file%.*}"
outdir=$(dirname $outfile)

mkdir -p $outdir

nix-shell -p gcc --run "rustc --test $file -o $outfile && ./$outfile"
