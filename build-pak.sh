#!/bin/bash

cargo build --release -p rusted-ruins-makepak
git clone https://github.com/garkimasera/rusted-ruins-pak

pushd rusted-ruins-pak
./build.sh ../target/release/rusted-ruins-makepak
popd

mkdir -p ./assets/paks
mkdir -p ./assets/sound
cp -r rusted-ruins-pak/paks/* -t ./assets/paks
cp -r rusted-ruins-pak/text/* -t ./assets/text
cp -r rusted-ruins-pak/sound/* -t ./assets/sound
