
use super::Map;

pub struct MapBuilder {
    map: Map,
}

impl MapBuilder {
    pub fn new(w: u32, h: u32) -> MapBuilder {
        MapBuilder {
            map: Map::new(w, h),
        }
    }

    pub fn build(self) -> Map {
        let map = self.map;
        
        map
    }
}

