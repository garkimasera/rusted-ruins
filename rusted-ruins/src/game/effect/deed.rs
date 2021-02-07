use crate::game::Game;
use crate::game::InfoGetter;
use common::gamedata::*;

pub fn use_deed(game: &mut Game) {
    let gd = &mut game.gd;
    let mapid = gd.get_current_mapid();
    if !mapid.is_region_map() {
        game_log_i!("use_item-deed-invalid-map");
        return;
    }

    let pos = gd.player_pos();
    let map = gd.get_current_map();
    if !map.tile[pos].special.is_none() {
        game_log_i!("use_item-deed-occupied");
    }

    let mut site = Site::new(1, None);
    site.content = SiteContent::Player {
        kind: PlayerBaseKind::Normal,
    };
    let rid = mapid.rid();
    let sid = gd.add_site(site, SiteKind::Player, rid, Some(pos)).unwrap();

    let map_random_id = crate::game::saveload::gen_box_id(gd);
    let map = if let Some(map) = crate::game::map::wilderness::generate_wilderness(gd, pos) {
        map
    } else {
        return;
    };
    gd.add_map(map, sid, map_random_id);

    let map = gd.get_current_map_mut();
    map.tile[pos].special = SpecialTileKind::SiteSymbol {
        kind: SiteSymbolKind::from("!rm-h0"),
    };
    game_log_i!("use_item-deed-succeed");
}
