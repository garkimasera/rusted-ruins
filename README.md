# Rusted Ruins
Extensible rouge like game. Players can explore the wilderness and ruins.
This game is written in Rust.

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

## License
GPL v3
