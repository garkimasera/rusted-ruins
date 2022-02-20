# Rusted Ruins [![Build Status](https://github.com/garkimasera/rusted-ruins/actions/workflows/ci.yml/badge.svg)](https://github.com/garkimasera/rusted-ruins/actions)
Extensible open world rogue like game with pixel art. Players can explore the wilderness and ruins.
This game is written in Rust.

## Screenshot
Ruin (Example of auto generated map)

![exploring-ruin](https://raw.githubusercontent.com/wiki/garkimasera/rusted-ruins/images/screenshot-exploring-ruin.png)

Town (Example of created map by map-editor)

![town](https://raw.githubusercontent.com/wiki/garkimasera/rusted-ruins/images/screenshot-town.png)

## Video

https://youtu.be/t3Fo2ujrYIo

## Game Objective

The player arrives at a recently discovered continent where a lot of ruins remain. The player will explore ruins and fight against monsters. By collecting relics in ruins, the player can earn money and fame, and solve the mystery of the ruined nation.

## Status
This is a very early project. Many features for playing are not completed.

Binary format of pak files and save files may be changed before version 1.0.

## Changelog

[See this wiki.](https://github.com/garkimasera/rusted-ruins/wiki/Changelog)

## Features

* 2D graphics.
* Easy to extend by the pak file system. Most of the assets are packaged as pak file. Pak file can be created by makepak. Users can add new characters, items and dungeons easily by pak file system.
* Map editor to create new map.
* Script to describe talks and events in game.
* Open world. Provide many playing style for players. The game objective will be different by players.

### Implemented Features

- Random dungeon generation
- Item creation
  - Crafting, Cooking
- Agriculture
- Mining
- Town
- Home building
- Wilderness map

### Planned Features

- Allies
  - Employ NPCs
  - Livestock
- Relationships with factions
- Town economy
  - Growable economic scale
  - Effects the amount and quality of goods in shops
- Various quests
  - Main quest

## Pak files
In this game, most of the image data and many assets are handled as *XXObject*.
XXObject is packaged to pak files. Their file extension is "pak".
Pak files and the sources are under [rusted-ruins-pak](https://github.com/garkimasera/rusted-ruins-pak).

## Precompiled packages

For Windows user, you can download from [Releases page](https://github.com/garkimasera/rusted-ruins/releases).
Deb package is also available.

## How to build and try
Please install SDL2 libraries at first.  

Ubuntu
```shell
sudo apt install libsdl2-dev libsdl2-ttf-dev libsdl2-image-dev libsdl2-mixer-dev
```

Fedora
```shell
sudo dnf install SDL2-devel.x86_64 SDL2_image-devel.x86_64 SDL2_ttf-devel.x86_64 SDL2_mixer-devel.x86_64
```

Rusted Ruins is written in Rust, so please install Rust compilation tools. You can use [rustup](https://www.rustup.rs/) to install Rust.

After that, clone this repository, download pak files, and run.

```shell
git clone https://github.com/garkimasera/rusted-ruins.git
cd rusted-ruins
./build-pak.sh
RUSTED_RUINS_ASSETS_DIR=./assets cargo run --release -p rusted-ruins
```

## How to operate

Operate the player character with the keyboard and mouse.

Left click on tiles - Move to the tile, melee attack, or start talking.

Left click + Ctrl - Shoot by the ranged weapon.

Left click + Shift - Use the equipped tool.

Right click - Open action menu. Actions for specified tile are available through the menu. For example, you can use stairs and enter/exit the map by opening menu at the tile that player is on.

WASD or Arrow key - Move

Enter key - Enter towns or dungeons, walk up/down stairs, and select an answer when talking.

### Sidebar

There are some icons on the sidebar. Click icons to open windows.

Icon list

* Inventory window
* Status window
* Active skill window
* Creation window
* Game information window
* Save / Exit game

### Shortcut keys

e - Eat an item

g - Pick up items

h - Help

q - Drink an item

r - Release an magical device item

v - Drop items

0..9 - Call shortcut registered by player

f1 - Open item window

f2 - Open status window

f3 - Open active skill window

f4 - Open creation window

f5 - Open game info window

f6 / escape - Open exit window

f12 - Debug command

## License
GPL v3
