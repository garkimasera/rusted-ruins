
use array2d::*;
use common::objholder::*;
use common::gamedata::map::Map;
use map_generator::{MapGenerator, GeneratedMap, TileKind};

pub struct MapBuilder {
    map: Map,
}

impl MapBuilder {
    pub fn new(w: u32, h: u32) -> MapBuilder {
        let generated_map = MapGenerator::new((w, h)).flat().generate();
        let map = generated_map_to_map(generated_map, TileIdx(0), WallIdx(0));
        
        MapBuilder {
            map: map,
        }
    }

    pub fn build(self) -> Map {
        let map = self.map;
        
        map
    }
}

pub fn generated_map_to_map(gm: GeneratedMap, tile: TileIdx, wall: WallIdx) -> Map {
    let size = gm.size;
    let mut map = Map::new(size.0 as u32, size.1 as u32);

    for p in size.iter_from_zero() {
        map.tile[p].tile = tile;
        match gm.tile[p] {
            TileKind::Wall => {
                map.tile[p].wall = Some(wall);
            },
            _ => (),
        }
    }
    map
}

