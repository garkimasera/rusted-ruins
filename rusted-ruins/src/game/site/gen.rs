
use game;
use common::gamedata::*;
use common::objholder::*;
use common::gobj;
use common::sitegen::*;

/// Add unique citizens from SiteGenObject
pub fn add_unique_citizens(gd: &mut GameData, sid: SiteId, sg: &SiteGenObject) {
    for uc in &sg.unique_citizens {
        let mut chara = game::chara::creation::create_chara(gobj::id_to_idx(&uc.chara_template_id));
        let mid = MapId::SiteMap { sid: sid, floor: uc.floor };
        chara.rel = ::common::gamedata::chara::Relationship::FRIENDLY;
        
        if let Some(talk_script_id) = uc.talk_script_id.as_ref() { // Talk script setting
            chara.talk = Some(CharaTalk {
                id: talk_script_id.to_owned(),
                section: "start".to_owned(),
            });
        }
        
        let cid = gd.add_chara_to_site(chara, sid, uc.n);
        gd.region.get_map_mut(mid).locate_chara(cid, uc.pos);
    }
}

/// Add items for deepest floor of dungeon
pub fn add_for_deepest_floor(gd: &mut GameData, mid: MapId) {
    let map = gd.region.get_map_mut(mid);

    let p = if let Some(p) = ::game::map::choose_empty_tile(map) { p } else { return; };

    let idx: ItemIdx = gobj::id_to_idx("ancient-box");
    let item_obj: &ItemObject = gobj::get_obj(idx);
    let item = Item {
        idx: idx,
        flags: item_obj.default_flags,
        kind: item_obj.kind,
        rank: ItemRank::default(),
    };

    let mut item_list = ItemList::new(10);
    item_list.append(item, 1);
    map.tile[p].item_list = Some(item_list);
}

