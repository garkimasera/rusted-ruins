
use common::maptemplate::*;
use common::gamedata::map::*;
use common::gobj;

pub fn from_template(t: &MapTemplateObject) -> Map {
    let mut map = create_terrain(t);
    set_boundary(&mut map, t, 0);
    map
}

pub fn from_template_id(id: &str) -> Option<Map> {
    Some(from_template(gobj::get_by_id_checked(id)?))
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
            let wall_id = &t.wall_table[i as usize];
            map.tile[pos].wall = Some(gobj::id_to_idx(wall_id));
        }
    }

    for (pos, i) in t.deco.iter_with_idx() { // Setting decos
        if let Some(i) = *i {
            let deco_id = &t.deco_table[i as usize];
            map.tile[pos].deco = Some(gobj::id_to_idx(deco_id));
        }
    }

    map
}

/// Setting Boundaries
pub fn set_boundary(map: &mut Map, t: &MapTemplateObject, floor: u32) {
    let next_floor = BoundaryBehavior::Floor(floor + 1);
    let prev_floor = if floor == 0 {
        BoundaryBehavior::RegionMap
    } else {
        BoundaryBehavior::Floor(floor - 1)
    };

    let f = |bb: &mut BoundaryBehavior, mtbb: MapTemplateBoundaryBehavior| {
        *bb = match mtbb {
            MapTemplateBoundaryBehavior::None => BoundaryBehavior::None,
            MapTemplateBoundaryBehavior::NextFloor => next_floor,
            MapTemplateBoundaryBehavior::PrevFloor => prev_floor,
            MapTemplateBoundaryBehavior::RegionMap => BoundaryBehavior::RegionMap,
        };
    };

    f(&mut map.boundary.n, t.boundary.n);
    f(&mut map.boundary.s, t.boundary.s);
    f(&mut map.boundary.e, t.boundary.e);
    f(&mut map.boundary.w, t.boundary.w);
}

