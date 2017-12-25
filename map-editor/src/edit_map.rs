
use array2d::*;
use common::obj::MapTemplateObject;
use common::objholder::*;
use common::gobj;

pub struct EditingMap {
    pub property: MapProperty,
    pub width: u32,
    pub height: u32,
    pub tile: Array2d<TileIdx>,
}

impl EditingMap {
    pub fn new(id: &str, width: u32, height: u32) -> EditingMap {
        let tile = Array2d::new(width, height, TileIdx(0));
        let property = MapProperty::new(id);
        EditingMap { property, tile, width, height }
    }

    pub fn set_tile(&mut self, pos: Vec2d, tile: TileIdx) {
        self.tile[pos] = tile;
    }

    pub fn create_mapobj(&self) -> MapTemplateObject {
        let mut tile_table: Vec<String> = Vec::new();
        
        for &tile_idx in self.tile.iter() {
            let tile_id = gobj::idx_to_id(tile_idx);
            if !tile_table.iter().any(|a| *a == tile_id) {
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

        MapTemplateObject {
            id: self.property.id.to_owned(),
            w: self.width,
            h: self.height,
            tile_table: tile_table,
            tile: tile_map,
        }
    }
}

#[derive(Debug)]
pub struct MapProperty {
    id: String,
}

impl MapProperty {
    fn new(id: &str) -> MapProperty {
        MapProperty {
            id: id.to_owned(),
        }
    }
}

