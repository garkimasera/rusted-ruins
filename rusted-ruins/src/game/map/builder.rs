
use common::objholder::*;
use common::gamedata::map::{Map, SpecialTileKind, StairsKind, FLOOR_OUTSIDE};
use map_generator::{MapGenerator, GeneratedMap, TileKind};

#[derive(Default)]
pub struct MapBuilder {
    w: u32, h: u32,
    floor: u32,
    tile: TileIdx,
    wall: WallIdx,
}

impl MapBuilder {
    pub fn new(w: u32, h: u32) -> MapBuilder {
        let mut map_builder = MapBuilder::default();
        map_builder.w = w;
        map_builder.h = h;
        map_builder
    }
    
    pub fn build(self) -> Map {
        let generated_map = MapGenerator::new((self.w, self.h)).fractal().generate();
        generated_map_to_map(generated_map, self.tile, self.wall, self.floor)
    }

    pub fn floor(mut self, floor: u32) -> MapBuilder {
        self.floor = floor;
        self
    }

    pub fn tile(mut self, tile: TileIdx) -> MapBuilder {
        self.tile = tile;
        self
    }

    pub fn wall(mut self, wall: WallIdx) -> MapBuilder {
        self.wall = wall;
        self
    }
}

pub fn generated_map_to_map(gm: GeneratedMap, tile: TileIdx, wall: WallIdx, floor: u32) -> Map {
    let size = gm.size;
    let mut map = Map::new(size.0 as u32, size.1 as u32);

    trace!("New map creating");

    for p in size.iter_from_zero() {
        map.tile[p].tile = tile;
        match gm.tile[p] {
            TileKind::Wall => {
                map.tile[p].wall = Some(wall);
            },
            _ => (),
        }
    }

    // Set stairs
    let entrance_stairs = StairsKind::UpStairs;
    let exit_stairs = StairsKind::DownStairs;

    let dest_floor = if floor == 0 { FLOOR_OUTSIDE } else { floor - 1 };
    map.entrance = gm.entrance;
    map.tile[gm.entrance].special = SpecialTileKind::Stairs { dest_floor, kind: entrance_stairs };

    if gm.exit.is_some()  {
        let dest_floor = floor + 1;
        map.tile[gm.exit.unwrap()].special = SpecialTileKind::Stairs { dest_floor, kind: exit_stairs };;
    }
    
    map
}

