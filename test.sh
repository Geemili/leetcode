#!/usr/bin/env bash

target="target"
file=$1
outfile="$target/${file%.*}"
outdir=$(dirname $outfile)

mkdir -p $outdir

rustc --test $file -o $outfile && ./$outfile
