use arrayvec::ArrayVec;
use common::basic::N_TILE_IMG_LAYER;
use common::gamedata::{ItemGen, TileLayers};
use common::gobj;
use common::maptemplate::*;
use common::objholder::*;
use common::piece_pattern::*;
use geom::*;

pub struct EditingMap {
    pub property: MapProperty,
    pub width: u32,
    pub height: u32,
    pub tile: Array2d<TileLayers>,
    pub wall: Array2d<WallIdxPp>,
    pub deco: Array2d<Option<DecoIdx>>,
    pub items: Array2d<Vec<ItemGen>>,
}

impl EditingMap {
    pub fn new(id: &str, width: u32, height: u32) -> EditingMap {
        let tile = Array2d::new(width, height, TileLayers::from(TileIdx::default()));
        let wall = Array2d::new(width, height, WallIdxPp::default());
        let deco = Array2d::new(width, height, None);
        let property = MapProperty::new(id);
        let items = Array2d::new(width, height, vec![]);
        EditingMap {
            property,
            width,
            height,
            tile,
            wall,
            deco,
            items,
        }
    }

    pub fn set_tile(&mut self, pos: Coords, idx: TileIdx, layer: usize) {
        self.tile[pos][layer] = TileIdxPp::new(idx);
    }

    pub fn set_wall(&mut self, pos: Coords, wall: Option<WallIdx>) {
        if let Some(idx) = wall {
            self.wall[pos] = WallIdxPp::new(idx);
        } else {
            self.wall[pos] = WallIdxPp::default();
        }

        for p in RectIter::new(
            pos + Direction::NW.as_coords(),
            pos + Direction::SE.as_coords(),
        ) {
            if !self.is_inside(p) || self.wall[p].is_empty() {
                continue;
            }

            let wall_idx = self.wall[p].idx().unwrap();
            let wall_obj = gobj::get_obj(wall_idx);

            if wall_obj.img.n_pattern == 1 {
                continue;
            }

            let ppf = PiecePatternFlags::from_fn(p, |p| {
                if self.is_inside(p) {
                    self.wall[p].idx() == Some(wall_idx)
                } else {
                    false
                }
            });

            let wallpp = WallIdxPp::with_piece_pattern(
                wall_idx,
                ppf.to_piece_pattern(wall_obj.img.n_pattern),
            );
            self.wall[p] = wallpp;
        }
    }

    pub fn set_deco(&mut self, pos: Coords, deco: Option<DecoIdx>) {
        self.deco[pos] = deco;
    }

    pub fn set_item(&mut self, pos: Coords, item_gen: Option<ItemGen>) {
        if let Some(item_gen) = item_gen {
            self.items[pos] = vec![item_gen];
        } else {
            self.items[pos] = vec![];
        }
    }

    pub fn get_item(&self, pos: Coords) -> Option<&ItemGen> {
        self.items[pos].get(0)
    }

    pub fn erase(&mut self, pos: Coords) {
        self.set_wall(pos, None);
        self.deco[pos] = None;
    }

    pub fn erase_layer(&mut self, pos: Coords, layer: usize) {
        self.tile[pos][layer] = TileIdxPp::default();
    }

    pub fn tile_layer_draw(&mut self, pos: Coords, new_tile_idx: TileIdx, layer: usize) {
        let piece_pattern = {
            let f = |pos: Coords| {
                if let Some(t) = self.tile.get(pos) {
                    t[layer].idx() == Some(new_tile_idx)
                } else {
                    true
                }
            };
            let mut piece_pattern_flags = PiecePatternFlags::default();
            for dir in &Direction::EIGHT_DIRS {
                piece_pattern_flags.set(*dir, f(pos + dir.as_coords()));
            }
            let tile_obj = gobj::get_obj(new_tile_idx);
            piece_pattern_flags.to_piece_pattern(tile_obj.img.n_pattern)
        };

        self.tile[pos][layer] = TileIdxPp::with_piece_pattern(new_tile_idx, piece_pattern);
    }

    pub fn resize(&mut self, new_w: u32, new_h: u32, offset_x: i32, offset_y: i32) {
        self.width = new_w;
        self.height = new_h;
        let top_left = Coords(offset_x, offset_y);
        let bottom_right = Coords(new_w as i32 + offset_x, new_h as i32 + offset_y);
        let tile = self
            .tile
            .clip_with_default(top_left, bottom_right, TileLayers::default());
        self.tile = tile;
        let wall = self
            .wall
            .clip_with_default(top_left, bottom_right, WallIdxPp::default());
        self.wall = wall;
        let deco = self.deco.clip_with_default(top_left, bottom_right, None);
        self.deco = deco;
        let items = self
            .items
            .clip_with_default(top_left, bottom_right, Vec::new());
        self.items = items;
    }

    pub fn is_inside(&self, p: Coords) -> bool {
        p.0 >= 0 && p.0 < self.width as i32 && p.1 >= 0 && p.1 < self.height as i32
    }

    pub fn create_mapobj(&self) -> MapTemplateObject {
        let mut tile_table: Vec<String> = Vec::new();

        // Create table for TileIdx
        for &tile in self.tile.iter() {
            for i in 0..N_TILE_IMG_LAYER {
                let tile_idx = if let Some(idx) = tile[i].idx() {
                    idx
                } else {
                    continue;
                };
                let tile_id = gobj::idx_to_id(tile_idx);
                if tile_table.iter().all(|a| *a != tile_id) {
                    tile_table.push(tile_id.to_owned());
                }
            }
        }

        // Create converted tile map
        let mut tile_map = Array2d::new(self.width, self.height, TileLayersConverted::default());
        for (pos, tile) in self.tile.iter_with_idx() {
            tile_map[pos] = tile.conv_into(&tile_table);
        }

        // Create table for WallIdx
        let mut wall_table: Vec<String> = Vec::new();
        for wall in self.wall.iter() {
            if !wall.is_empty() {
                let wall_id = gobj::idx_to_id(wall.idx().unwrap());
                if wall_table.iter().all(|a| *a != wall_id) {
                    wall_table.push(wall_id.to_owned());
                }
            }
        }
        // Create converted wall map
        let mut wall_map = Array2d::new(self.width, self.height, ConvertedIdxPp::default());
        for (pos, wall) in self.wall.iter_with_idx() {
            wall_map[pos] = wall.conv_into(&wall_table);
        }

        let mut deco_table: Vec<String> = Vec::new();
        for deco_idx in self.deco.iter() {
            if let Some(deco_idx) = *deco_idx {
                let deco_id = gobj::idx_to_id(deco_idx);
                if deco_table.iter().all(|a| *a != deco_id) {
                    deco_table.push(deco_id.to_owned());
                }
            }
        }
        let mut deco_map = Array2d::new(self.width, self.height, None);
        for (pos, deco_idx) in self.deco.iter_with_idx() {
            if let Some(deco_idx) = *deco_idx {
                let deco_id = gobj::idx_to_id(deco_idx);
                let converted_idx = deco_table
                    .iter()
                    .enumerate()
                    .find(|&(_, a)| a == deco_id)
                    .unwrap()
                    .0 as u32;
                deco_map[pos] = Some(converted_idx);
            }
        }

        // Create items
        let mut items = Vec::new();
        for (pos, item_gens) in self.items.iter_with_idx() {
            for item_gen in item_gens {
                items.push((pos, item_gen.clone()));
            }
        }
        items.sort();

        MapTemplateObject {
            id: self.property.id.to_owned(),
            w: self.width,
            h: self.height,
            tile_table,
            tile: tile_map,
            wall_table,
            wall: wall_map,
            deco_table,
            deco: deco_map,
            boundary: self.property.boundary,
            entrance: self.property.entrance.clone(),
            items,
            music: self.property.music.clone(),
        }
    }
}

#[derive(Debug)]
pub struct MapProperty {
    pub id: String,
    pub is_region_map: bool,
    pub boundary: MapTemplateBoundary,
    pub entrance: ArrayVec<Coords, 4>,
    pub music: String,
}

impl MapProperty {
    fn new(id: &str) -> MapProperty {
        MapProperty {
            id: id.to_owned(),
            is_region_map: false,
            boundary: MapTemplateBoundary::default(),
            entrance: ArrayVec::new(),
            music: String::default(),
        }
    }
}

impl From<MapTemplateObject> for EditingMap {
    fn from(obj: MapTemplateObject) -> EditingMap {
        let mut map = EditingMap::new(&obj.id, obj.w, obj.h);

        for (pos, c) in obj.tile.iter_with_idx() {
            map.tile[pos] = TileLayers::conv_from(*c, &obj.tile_table);
        }

        for (pos, c) in obj.wall.iter_with_idx() {
            map.wall[pos] = WallIdxPp::conv_from(*c, &obj.wall_table);
        }

        for (pos, i) in obj.deco.iter_with_idx() {
            if let Some(i) = *i {
                let deco_id = &obj.deco_table[i as usize];
                map.deco[pos] = Some(gobj::id_to_idx(deco_id));
            }
        }

        for (pos, item_gen) in &obj.items {
            map.set_item(*pos, Some(item_gen.clone()));
        }

        map.property.boundary = obj.boundary;
        map.property.entrance = obj.entrance;
        map.property.music = obj.music;

        map
    }
}
