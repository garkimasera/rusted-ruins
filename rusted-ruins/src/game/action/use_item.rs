use crate::game::InfoGetter;
use common::gamedata::*;
use common::gobj;

pub fn use_item(gd: &mut GameData, il: ItemLocation, cid: CharaId) {
    let item = gd.get_item(il);
    let item_obj = gobj::get_obj(item.0.idx);

    match item_obj.use_effect {
        UseEffect::None => panic!("use invalid item"),
        UseEffect::Deed => {
            assert_eq!(cid, CharaId::Player);

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
            site.content = SiteContent::Base {
                kind: BaseKind::Home,
            };
            let rid = mapid.rid();
            let sid = gd.add_site(site, SiteKind::Base, rid, pos).unwrap();

            let map_random_id = crate::game::saveload::gen_box_id(gd);
            let map =
                crate::game::map::from_template::from_template_id("home-default", false).unwrap();
            gd.add_map(map, sid, map_random_id);

            let map = gd.get_current_map_mut();
            map.tile[pos].special = SpecialTileKind::SiteSymbol {
                kind: SiteSymbolKind::from("!rm-h0"),
            };
            game_log_i!("use_item-deed-succeed");
            gd.remove_item(il, 1);
        }
    }
}
