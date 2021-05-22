use crate::map_generator::*;
use common::gamedata::map::*;
use common::gobj;
use common::objholder::*;
use geom::*;
use rules::RULES;

#[derive(Default)]
pub struct MapBuilder {
    floor: u32,
    is_deepest_floor: bool,
    map_gen_param: MapGenParam,
    tile: TileIdx,
    wall: WallIdx,
    map_boundary: Option<MapBoundary>,
    music: String,
}

impl MapBuilder {
    pub fn new(w: u32, h: u32) -> MapBuilder {
        MapBuilder {
            map_gen_param: MapGenParam::Flat { w, h },
            ..MapBuilder::default()
        }
    }

    pub fn from_map_gen_id(id: &str) -> Self {
        let map_gen_param = &RULES.map_gen.map_gen_params[id];
        let (w, h) = map_gen_param.size();
        let mut builder = Self::new(w, h);
        builder.map_gen_param = map_gen_param.clone();
        builder
    }

    pub fn build(self) -> Map {
        let generated_map = self.map_gen_param.generate();
        let mut map = generated_map_to_map(
            generated_map,
            self.tile,
            self.wall,
            self.floor,
            self.is_deepest_floor,
        );
        if let Some(map_boundary) = self.map_boundary {
            map.boundary = map_boundary;
        }
        map.music = self.music;
        map
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

    pub fn map_boundary(mut self, map_boundary: MapBoundary) -> MapBuilder {
        self.map_boundary = Some(map_boundary);
        self
    }

    pub fn music(mut self, music: &str) -> MapBuilder {
        self.music = music.to_owned();
        self
    }
}

pub fn generated_map_to_map(
    gm: GeneratedMap,
    tile: TileIdx,
    wall: WallIdx,
    floor: u32,
    is_deepest_floor: bool,
) -> Map {
    let size = gm.size;
    let mut map = Map::new(size.0 as u32, size.1 as u32);
    let wall_obj = gobj::get_obj(wall);

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
                    let mut piece_pattern_flags = PiecePatternFlags::default();
                    for dir in &Direction::EIGHT_DIRS {
                        piece_pattern_flags.set(*dir, f(p + dir.as_vec()));
                    }
                    let wall_obj = gobj::get_obj(wall);
                    piece_pattern_flags.to_piece_pattern(wall_obj.img.n_pattern)
                };
                map.tile[p].wall = WallIdxPp::with_piece_pattern(wall, piece_pattern);
                map.tile[p].wall_hp = wall_obj.hp;
            }
            _ => (),
        }
    }

    match gm.entrance {
        Entrance::Pos(pos) => {
            map.entrance = pos;
        }
        Entrance::Stairs(e0, e1) => {
            // Set stairs
            let entrance_stairs = StairsKind::UpStairs;
            let exit_stairs = StairsKind::DownStairs;

            let dest_floor = if floor == 0 { FLOOR_OUTSIDE } else { floor - 1 };
            map.entrance.push(e0);
            map.tile[e0].special = SpecialTileKind::Stairs {
                dest_floor,
                kind: entrance_stairs,
            };

            if !is_deepest_floor && e1.is_some() {
                let dest_floor = floor + 1;
                map.tile[e1.unwrap()].special = SpecialTileKind::Stairs {
                    dest_floor,
                    kind: exit_stairs,
                };
            }
        }
    }

    map
}
