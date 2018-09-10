#!/bin/sh

curl -LkSs https://github.com/garkimasera/rusted-ruins-pak/archive/master.zip > rusted-ruins-pak.zip
unzip rusted-ruins-pak.zip > /dev/null
mkdir -p ./res/paks
mkdir -p ./res/sound
cp -r rusted-ruins-pak-master/paks/* -t ./res/paks
cp -r rusted-ruins-pak-master/text/* -t ./res/text
cp -r rusted-ruins-pak-master/sound/* -t ./res/sound

