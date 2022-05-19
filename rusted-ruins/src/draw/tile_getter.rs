use crate::game::view::ViewMap;
use common::gamedata::*;
use common::gobj;
use common::obj::SpecialTileObject;
use common::objholder::*;
use geom::*;

/// Needed information to draw background parts of an tile
/// "Background" means that they are drawed behind any characters
#[derive(Default)]
pub struct BackgroundDrawInfo<'a> {
    pub tile: Option<&'a TileLayers>,
    pub special: Option<SpecialTileIdx>,
}

impl<'a> BackgroundDrawInfo<'a> {
    pub fn new(map: &Map, pos: Coords) -> BackgroundDrawInfo<'_> {
        let mut di = BackgroundDrawInfo::default();

        let tile = if map.is_inside(pos) {
            let tinfo = &map.observed_tile[pos];
            if tinfo.tile {
                Some(&map.tile[pos].tile)
            } else {
                None
            }
        } else if let Some(ref _outside_tile) = map.outside_tile {
            unimplemented!()
        //Some(outside_tile.tile.clone().into())
        } else {
            let pos = map.nearest_existent_tile(pos);
            // let tinfo = &map.observed_tile[pos];
            Some(&map.tile[pos].tile)
        };

        di.tile = tile;

        if map.is_inside(pos) {
            if let Some(special_tile_id) = map.observed_tile[pos].special.obj_id() {
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

/// Needed information to draw foreground parts of an tile
/// "Foreground" means that they are drawed infront characters
/// whose are on the prev row
#[derive(Default)]
pub struct ForegroundDrawInfo<'a> {
    pub special: Option<SpecialTileIdx>,
    pub wallpp: WallIdxPp,
    pub deco: Option<DecoIdx>,
    pub items: &'a [(ItemIdx, u32)],
    pub chara: Option<CharaId>,
}

impl<'a> ForegroundDrawInfo<'a> {
    pub fn new(map: &'a Map, view_map: &ViewMap, pos: Coords) -> ForegroundDrawInfo<'a> {
        let mut di = ForegroundDrawInfo::default();

        if map.is_inside(pos) {
            if let Some(special_tile_id) = map.observed_tile[pos].special.obj_id() {
                let special_tile_obj: &'static SpecialTileObject = gobj::get_by_id(special_tile_id);
                if !special_tile_obj.always_background {
                    let special_tile_idx: SpecialTileIdx = gobj::id_to_idx(special_tile_id);
                    di.special = Some(special_tile_idx);
                }
            }
        }

        di.wallpp = if map.is_inside(pos) {
            di.deco = map.observed_tile[pos].deco;
            map.observed_tile[pos].wall
        } else if let Some(ref outside_tile) = map.outside_tile {
            if let Some(wall_idx) = outside_tile.wall {
                WallIdxPp::new(wall_idx)
            } else {
                WallIdxPp::default()
            }
        } else {
            let nearest_pos = map.nearest_existent_tile(pos);
            let wallpp = map.observed_tile[nearest_pos].wall;
            if !wallpp.is_empty() {
                adjust_pattern_from_nearest(&mut wallpp.piece_pattern(), pos, nearest_pos);
            }
            wallpp
        };

        if view_map.get_tile_visible(pos) {
            di.chara = map.get_chara(pos);
        }

        // Set items
        if map.is_inside(pos) {
            di.items = map.observed_tile[pos].items.as_slice();
        }

        di
    }
}

/// Adjust piece pattern when getting piece pattern from the nearest tile.
fn adjust_pattern_from_nearest(pp: &mut PiecePattern, _pos: Coords, _nearest_pos: Coords) {
    *pp = PiecePattern::SURROUNDED;
}
