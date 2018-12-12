
use crate::array2d::*;
use common::objholder::*;
use common::gamedata::map::*;
use common::gobj;
use crate::map_generator::{MapGenerator, GeneratedMap, TileKind};

#[derive(Default)]
pub struct MapBuilder {
    w: u32, h: u32,
    floor: u32,
    is_deepest_floor: bool,
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
        generated_map_to_map(generated_map, self.tile, self.wall, self.floor, self.is_deepest_floor)
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

    pub fn deepest_floor(mut self, is_deepest_floor: bool) -> MapBuilder {
        self.is_deepest_floor = is_deepest_floor;
        self
    }
}

pub fn generated_map_to_map(gm: GeneratedMap, tile: TileIdx, wall: WallIdx,
                            floor: u32, is_deepest_floor: bool) -> Map {
    
    let size = gm.size;
    let mut map = Map::new(size.0 as u32, size.1 as u32);

    trace!("New map creating");

    for p in size.iter_from_zero() {
        map.tile[p].tile = tile.into();
        match gm.tile[p] {
            TileKind::Wall => {
                let piece_pattern = {
                    let f = |pos: Vec2d| {
                        if let Some(t) = gm.tile.get(pos) {
                            *t == TileKind::Wall
                        } else {
                            true
                        }
                    };
                    let mut piece_pattern_flags = PiecePatternFlags::new();
                    for dir in &Direction::EIGHT_DIRS {
                        piece_pattern_flags.set(*dir, f(p + dir.as_vec()));
                    }
                    let wall_obj = gobj::get_obj(wall);
                    piece_pattern_flags.to_piece_pattern(wall_obj.img.n_pattern)
                };
                map.tile[p].wall = WallIdxPP::with_piece_pattern(wall, piece_pattern);
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

    if !is_deepest_floor && gm.exit.is_some() {
        let dest_floor = floor + 1;
        map.tile[gm.exit.unwrap()].special = SpecialTileKind::Stairs { dest_floor, kind: exit_stairs };;
    }
    
    map
}

