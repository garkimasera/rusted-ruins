#!/bin/sh

curl -LkSs https://github.com/garkimasera/rusted-ruins-pak/archive/master.zip > rusted-ruins-pak.zip
unzip rusted-ruins-pak.zip > /dev/null
mkdir -p ./assets/paks
mkdir -p ./assets/sound
cp -r rusted-ruins-pak-master/paks/* -t ./assets/paks
cp -r rusted-ruins-pak-master/text/* -t ./assets/text
cp -r rusted-ruins-pak-master/sound/* -t ./assets/sound

