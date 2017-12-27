
use common::obj::MapTemplateObject;
use common::gamedata::map::Map;
use common::gobj;

pub fn from_template(t: &MapTemplateObject) -> Map {
    let map = create_terrain(t);
    map
}

/// Create map its terrains (tile, wall) are loaded from template
fn create_terrain(t: &MapTemplateObject) -> Map {
    let mut map = Map::new(t.w, t.h);

    for (pos, &i) in t.tile.iter_with_idx() { // Setting tiles
        let tile_id = &t.tile_table[i as usize];
        map.tile[pos].tile = gobj::id_to_idx(tile_id);
    }

    for (pos, i) in t.wall.iter_with_idx() { // Setting walls
        if let Some(i) = *i {
            let wall_id = &t.tile_table[i as usize];
            map.tile[pos].wall = Some(gobj::id_to_idx(wall_id));
        }
    }

    map
}

