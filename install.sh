#!/bin/bash

echo Cloning repo.
cd
git clone https://github.com/Tarun-Sri-Sai/jsondiff.git

echo Building release binary.
cd jsondiff
git checkout release
cargo build --release

$destDirectory = '/usr/local/bin'
echo "Moving binary to $destDirectory."
sudo mkdir -p $destDirectory
mv ./target/release/jsondiff $destDirectory

echo Cleaning up.
cd ..
rm -rf jsondiff
