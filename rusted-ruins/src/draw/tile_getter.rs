
use array2d::*;
use common::objholder::*;
use common::gamedata::map::Map;
use common::gamedata::chara::CharaId;
use common::gobj;
use common::obj::SpecialTileObject;
use game::view::ViewMap;

/// Needed infomation to draw background parts of an tile
/// "Background" means that they are drawed behind any characters
#[derive(Default)]
pub struct BackgroundDrawInfo {
    pub tile: Option<TileIdx>,
    pub deco: Option<DecoIdx>,
    pub wall: Option<WallIdx>,
    pub special: Option<SpecialTileIdx>,
}

impl BackgroundDrawInfo {
    pub fn new(map: &Map, pos: Vec2d) -> BackgroundDrawInfo {
        let mut di = BackgroundDrawInfo::default();
        
        let (tile, deco, wall) = if map.is_inside(pos) {
            let tinfo = &map.tile[pos];
            (tinfo.tile, tinfo.deco, tinfo.wall)
        } else {
            if let Some(ref outside_tile) = map.outside_tile {
                (outside_tile.tile, outside_tile.deco, outside_tile.wall)
            } else {
                let pos = map.nearest_existent_tile(pos);
                let tinfo = &map.tile[pos];
                (tinfo.tile, tinfo.deco, tinfo.wall)
            }
        };
        
        if let Some(wall) = wall {
            let wall_obj = gobj::get_obj(wall);
            if wall_obj.base_draw {
                di.tile = Some(tile);
            }
            if wall_obj.always_background {
                di.wall = Some(wall);
            }
        } else {
            di.tile = Some(tile);
        };
        di.deco = deco;

        if map.is_inside(pos) {
            if let Some(special_tile_id) = map.tile[pos].special.obj_id() {
                let special_tile_obj: &'static SpecialTileObject = gobj::get_by_id(special_tile_id);
                if special_tile_obj.always_background {
                    let special_tile_idx: SpecialTileIdx = gobj::id_to_idx(special_tile_id);
                    di.special = Some(special_tile_idx);
                }
            }
        }
        
        di
    }
}

/// If the number of items on one tile is more than this,
/// remaining items will be not drawed.
pub const MAX_ITEM_FOR_DRAW: usize = 5;

/// Needed infomation to draw foreground parts of an tile
/// "Foreground" means that they are drawed infront characters
/// whose are on the prev row
#[derive(Default)]
pub struct ForegroundDrawInfo {
    pub special: Option<SpecialTileIdx>,
    pub wall: Option<WallIdx>,
    pub n_item: usize,
    pub items: [ItemIdx; MAX_ITEM_FOR_DRAW],
    pub chara: Option<CharaId>,
}

impl ForegroundDrawInfo {
    pub fn new(map: &Map, pos: Vec2d) -> ForegroundDrawInfo {
        let mut di = ForegroundDrawInfo::default();

        if map.is_inside(pos) {
            if let Some(special_tile_id) = map.tile[pos].special.obj_id() {
                let special_tile_obj: &'static SpecialTileObject = gobj::get_by_id(special_tile_id);
                if !special_tile_obj.always_background {
                    let special_tile_idx: SpecialTileIdx = gobj::id_to_idx(special_tile_id);
                    di.special = Some(special_tile_idx);
                }
            }
        }

        let wall = if map.is_inside(pos) {
            let tinfo = &map.tile[pos];
            tinfo.wall
        } else {
            if let Some(ref outside_tile) = map.outside_tile {
                outside_tile.wall
            } else {
                let pos = map.nearest_existent_tile(pos);
                let tinfo = &map.tile[pos];
                tinfo.wall
            }
        };

        if let Some(wall) = wall {
            let wall_obj = gobj::get_obj(wall);
            if !wall_obj.always_background {
                di.wall = Some(wall);
            }
        }
        
        di.chara = map.get_chara(pos);

        // Set items
        if map.is_inside(pos) {
            if let Some(ref item_list) = map.tile[pos].item_list {
                for (i, &(ref item, _)) in item_list.iter().take(MAX_ITEM_FOR_DRAW).enumerate() {
                    di.items[i] = item.idx;
                    di.n_item += 1;
                }
            }
        }
        
        di
    }

}

#[derive(Default)]
pub struct EffectDrawInfo {
    pub fog: Option<EffectIdx>
}

impl EffectDrawInfo {
    pub fn new(view_map: &ViewMap, pos: Vec2d) -> EffectDrawInfo {
        let mut di = EffectDrawInfo::default();

        if !view_map.get_tile_visible(pos) {
            di.fog = Some(EffectIdx(0));
        }

        di
    }
}

