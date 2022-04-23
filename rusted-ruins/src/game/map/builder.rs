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
    entrance_method: EntranceMethod,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EntranceMethod {
    Original,
    South,
}

impl Default for EntranceMethod {
    fn default() -> EntranceMethod {
        EntranceMethod::Original
    }
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
            self.entrance_method,
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

    pub fn entrance_method(mut self, entrance_method: EntranceMethod) -> MapBuilder {
        self.entrance_method = entrance_method;
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
    entrance_method: EntranceMethod,
) -> Map {
    let size = gm.size;
    let mut map = Map::new(
        size.0 as u32,
        size.1 as u32,
        crate::game::time::current_time(),
    );
    let wall_obj = gobj::get_obj(wall);

    trace!("New map creating");

    for p in size.iter_from_zero() {
        map.tile[p].tile = tile.into();
        if gm.tile[p] == TileKind::Wall {
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
    }

    match (entrance_method, gm.entrance.clone()) {
        (EntranceMethod::Original, Entrance::Pos(pos)) => {
            map.entrance = pos;
        }
        (EntranceMethod::Original, Entrance::Stairs(e0, e1)) => {
            // Set stairs
            let entrance_stairs = StairsKind::UpStairs;
            let exit_stairs = StairsKind::DownStairs;

            let dest_floor = if floor == 0 { FLOOR_OUTSIDE } else { floor - 1 };
            map.entrance.push(e0);
            map.tile[e0].special = SpecialTileKind::Stairs {
                dest_floor,
                kind: entrance_stairs,
            };

            if let (false, Some(e1)) = (is_deepest_floor, e1) {
                let dest_floor = floor + 1;
                map.tile[e1].special = SpecialTileKind::Stairs {
                    dest_floor,
                    kind: exit_stairs,
                };
            }
        }
        (EntranceMethod::South, _) => {
            map.entrance.push(search_empty_tile_on_boundary(&gm));
        }
    }

    map
}

fn search_empty_tile_on_boundary(gm: &GeneratedMap) -> Vec2d {
    let d0 = Vec2d(1, 0);
    let d1 = Vec2d(0, -1);

    let start = Vec2d(gm.size.0 / 2, gm.size.1 - 1);
    let mut a = 0i32;
    let mut b = 0i32;

    loop {
        let p = start + d0 * a + d1 * b;
        if gm.tile[p] == TileKind::Floor {
            return p;
        }
        if a <= 0 {
            a = -a + 1;
        } else {
            a = -a;
        }
        let x = start.0 + a;
        if x < 0 || x >= gm.size.0 {
            a = 0;
            b += 1;
        }
    }
}
