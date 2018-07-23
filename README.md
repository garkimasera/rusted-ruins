# Rusted Ruins [![Build Status](https://travis-ci.org/garkimasera/rusted-ruins.svg?branch=master)](https://travis-ci.org/garkimasera/rusted-ruins)
Extensible rouge like game with pixel art. Players can explore the wilderness and ruins.
This game is written in Rust.

## Screenshot
![exploring-caves](https://github.com/garkimasera/rusted-ruins/blob/master/screenshots/exploring-ruin.png)

## Status
This is a very early project. Many features for playing are not completed.

## Pak files
In this game, most of image data and many assets are handled as *XXObject*.
XXObject is packaged to pak files. Their file extension is "pak".
Pak files and the sources are under [rusted-ruins-pak](https://github.com/garkimasera/rusted-ruins-pak).

## How to try
Please install SDL2 libraries at first.
For Ubuntu users:
```shell
sudo apt install libsdl2-dev libsdl2-ttf-dev libsdl2-image-dev libsdl2-mixer-dev
```

Rusted Ruins is written in Rust, so please install Rust compilation tools. You can use [rustup](https://www.rustup.rs/) to install Rust.

After that, clone this repository, download pak files, and run.

```shell
git clone https://github.com/garkimasera/rusted-ruins.git
cd rusted-ruins
./download-pak.sh
RUSTED_RUINS_APP_DIR=./res cargo run -p rusted-ruins
```

## Keys

Arrow keys - Move
d - Drop items
e - Eat an item
i - View inventory
p - Pick up items
q - Drink an item
s - Open status window
t - Targettin mode
w - Open equipment window
f - Shot
escape - Open exit window

## License
GPL v3
