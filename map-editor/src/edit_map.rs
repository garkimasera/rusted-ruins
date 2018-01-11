
use array2d::*;
use common::obj::MapTemplateObject;
use common::objholder::*;
use common::gobj;

pub struct EditingMap {
    pub property: MapProperty,
    pub width: u32,
    pub height: u32,
    pub tile: Array2d<TileIdx>,
    pub wall: Array2d<Option<WallIdx>>,
}

impl EditingMap {
    pub fn new(id: &str, width: u32, height: u32) -> EditingMap {
        let tile = Array2d::new(width, height, TileIdx(0));
        let wall = Array2d::new(width, height, None);
        let property = MapProperty::new(id);
        EditingMap { property, width, height, tile, wall }
    }

    pub fn set_tile(&mut self, pos: Vec2d, tile: TileIdx) {
        self.tile[pos] = tile;
    }

    pub fn set_wall(&mut self, pos: Vec2d, wall: Option<WallIdx>) {
        self.wall[pos] = wall;
    }

    pub fn create_mapobj(&self) -> MapTemplateObject {
        let mut tile_table: Vec<String> = Vec::new();
        
        for &tile_idx in self.tile.iter() {
            let tile_id = gobj::idx_to_id(tile_idx);
            if tile_table.iter().all(|a| *a != tile_id) {
                tile_table.push(tile_id.to_owned());
            }
        }
        let mut tile_map = Array2d::new(self.width, self.height, 0);
        for (pos, &tile_idx) in self.tile.iter_with_idx() {
            let tile_id = gobj::idx_to_id(tile_idx);
            let converted_idx =
                tile_table.iter().enumerate().find(|&(_, a)| a == tile_id).unwrap().0 as u32;
            tile_map[pos] = converted_idx;
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

        MapTemplateObject {
            id: self.property.id.to_owned(),
            w: self.width,
            h: self.height,
            tile_table: tile_table,
            tile: tile_map,
            wall_table: wall_table,
            wall: wall_map,
        }
    }
}

#[derive(Debug)]
pub struct MapProperty {
    pub id: String,
    pub is_region_map: bool,
}

impl MapProperty {
    fn new(id: &str) -> MapProperty {
        MapProperty {
            id: id.to_owned(),
            is_region_map: false,
        }
    }
}

impl From<MapTemplateObject> for EditingMap {
    fn from(obj: MapTemplateObject) -> EditingMap {
        let mut map = EditingMap::new(&obj.id, obj.w, obj.h);

        for (pos, &i) in obj.tile.iter_with_idx() {
            let tile_id = &obj.tile_table[i as usize];
            map.tile[pos] = gobj::id_to_idx(tile_id);
        }

        for (pos, i) in obj.wall.iter_with_idx() {
            if let Some(i) = *i {
                let wall_id = &obj.tile_table[i as usize];
                map.wall[pos] = Some(gobj::id_to_idx(wall_id));
            }
        }
        
        map
    }
}

