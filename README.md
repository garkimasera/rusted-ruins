# Rusted Ruins
Extensible rouge like game. Players can explore the wilderness and ruins.
This game is written in Rust.

## Pak files
In this game, most of image data and many assets are handled as *XXObject*.
XXObject is packaged to pak files. Their file extension is "pak".
Pak files and the sources are under [rusted-ruins-pak](https://github.com/garkimasera/rusted-ruins-pak).

## How to try
```shell
git clone https://github.com/garkimasera/rusted-ruins.git
cd rusted-ruins
./download-pak.sh
RUSTED_RUINS_APP_DIR=./res cargo run -p rusted-ruins
```

## License
GPL v3
