
use game;
use common::gamedata::*;
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
        
        gd.add_chara_to_map(chara, CharaKind::OnMap, mid, uc.pos);
    }
}

