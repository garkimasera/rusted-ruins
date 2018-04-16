
use array2d::*;
use common::maptemplate::*;
use common::objholder::*;
use common::gamedata::OverlappedTile;
use common::gobj;

pub struct EditingMap {
    pub property: MapProperty,
    pub width: u32,
    pub height: u32,
    pub tile: Array2d<OverlappedTile>,
    pub wall: Array2d<Option<WallIdx>>,
    pub deco: Array2d<Option<DecoIdx>>,
}

impl EditingMap {
    pub fn new(id: &str, width: u32, height: u32) -> EditingMap {
        let tile = Array2d::new(width, height, OverlappedTile::default());
        let wall = Array2d::new(width, height, None);
        let deco = Array2d::new(width, height, None);
        let property = MapProperty::new(id);
        EditingMap { property, width, height, tile, wall, deco }
    }

    pub fn set_tile(&mut self, pos: Vec2d, tile: TileIdx) {
        self.tile[pos] = tile.into();
    }

    pub fn set_wall(&mut self, pos: Vec2d, wall: Option<WallIdx>) {
        self.wall[pos] = wall;
    }

    pub fn set_deco(&mut self, pos: Vec2d, deco: Option<DecoIdx>) {
        self.deco[pos] = deco;
    }

    pub fn erase(&mut self, pos: Vec2d) {
        self.wall[pos] = None;
        self.deco[pos] = None;
    }

    pub fn resize(&mut self, new_w: u32, new_h: u32) {
        self.width = new_w;
        self.height = new_h;
        let tile = self.tile.clip_with_default((0, 0), (new_w, new_h), OverlappedTile::default());
        self.tile = tile;
        let wall = self.wall.clip_with_default((0, 0), (new_w, new_h), None);
        self.wall = wall;
        let deco = self.deco.clip_with_default((0, 0), (new_w, new_h), None);
        self.deco = deco;
    }

    pub fn create_mapobj(&self) -> MapTemplateObject {
        let mut tile_table: Vec<String> = Vec::new();
        
        for &tile in self.tile.iter() {
            for i in 0..tile.len as usize {
                let tile_idx = tile.idx[i];
                let tile_id = gobj::idx_to_id(tile_idx);
                if tile_table.iter().all(|a| *a != tile_id) {
                    tile_table.push(tile_id.to_owned());
                }
            }
        }
        
        let mut tile_map = Array2d::new(self.width, self.height, OverlappedTileConverted::default());
        for (pos, tile) in self.tile.iter_with_idx() {
            let mut c = OverlappedTileConverted::default();
            
            for i in 0..tile.len as usize {
                let tile_id = gobj::idx_to_id(tile.idx[i]);
                let converted_idx =
                    tile_table.iter().enumerate().find(|&(_, a)| a == tile_id).unwrap().0 as u32;
                c.i_pattern[i] = tile.i_pattern[i];
                c.idx[i] = converted_idx;
            }

            c.len = tile.len;
            tile_map[pos] = c;
        }

        let mut wall_table: Vec<String> = Vec::new();
        for wall_idx in self.wall.iter() {
            if let Some(wall_idx) = *wall_idx {
                let wall_id = gobj::idx_to_id(wall_idx);
                if wall_table.iter().all(|a| *a != wall_id) {
                    wall_table.push(wall_id.to_owned());
                }
            }
        }
        let mut wall_map = Array2d::new(self.width, self.height, None);
        for (pos, wall_idx) in self.wall.iter_with_idx() {
            if let Some(wall_idx) = *wall_idx {
                let wall_id = gobj::idx_to_id(wall_idx);
                let converted_idx =
                    wall_table.iter().enumerate().find(|&(_, a)| a == wall_id).unwrap().0 as u32;
                wall_map[pos] = Some(converted_idx);
            }
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
                let converted_idx =
                    deco_table.iter().enumerate().find(|&(_, a)| a == deco_id).unwrap().0 as u32;
                deco_map[pos] = Some(converted_idx);
            }
        }

        MapTemplateObject {
            id: self.property.id.to_owned(),
            w: self.width,
            h: self.height,
            tile_table: tile_table,
            tile: tile_map,
            wall_table: wall_table,
            wall: wall_map,
            deco_table: deco_table,
            deco: deco_map,
            boundary: self.property.boundary,
        }
    }
}

#[derive(Debug)]
pub struct MapProperty {
    pub id: String,
    pub is_region_map: bool,
    pub boundary: MapTemplateBoundary,
}

impl MapProperty {
    fn new(id: &str) -> MapProperty {
        MapProperty {
            id: id.to_owned(),
            is_region_map: false,
            boundary: MapTemplateBoundary::default(),
        }
    }
}

impl From<MapTemplateObject> for EditingMap {
    fn from(obj: MapTemplateObject) -> EditingMap {
        let mut map = EditingMap::new(&obj.id, obj.w, obj.h);

        for (pos, c) in obj.tile.iter_with_idx() {
            let mut tile = OverlappedTile::default();
            tile.len = c.len;
            tile.i_pattern = c.i_pattern;

            for i in 0..(tile.len as usize) {
                let tile_id = &obj.tile_table[c.idx[i] as usize];
                let tile_idx: TileIdx = gobj::id_to_idx(tile_id);
                tile.idx[i] = tile_idx;
            }
            
            map.tile[pos] = tile;
        }

        for (pos, i) in obj.wall.iter_with_idx() {
            if let Some(i) = *i {
                let wall_id = &obj.wall_table[i as usize];
                map.wall[pos] = Some(gobj::id_to_idx(wall_id));
            }
        }

        for (pos, i) in obj.deco.iter_with_idx() {
            if let Some(i) = *i {
                let deco_id = &obj.deco_table[i as usize];
                map.deco[pos] = Some(gobj::id_to_idx(deco_id));
            }
        }

        map.property.boundary = obj.boundary;
        
        map
    }
}

