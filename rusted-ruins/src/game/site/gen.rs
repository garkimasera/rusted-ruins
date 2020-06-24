use crate::game;
use crate::game::saveload::gen_box_id;
use common::gamedata::*;
use common::gobj;
use common::obj::SiteGenObject;
use geom::*;

/// Add unique citizens from SiteGenObject
pub fn add_unique_citizens(gd: &mut GameData, sid: SiteId, sg: &SiteGenObject) {
    for uc in &sg.unique_citizens {
        let faction_id = sg.default_faction_id;
        let mut chara =
            game::chara::gen::create_chara(gobj::id_to_idx(&uc.chara_template_id), 1, faction_id);
        chara.ai.initial_pos = uc.pos;
        let mid = MapId::SiteMap {
            sid,
            floor: uc.floor,
        };
        chara.rel = common::gamedata::chara::Relationship::FRIENDLY;

        if let Some(talk_script_id) = uc.talk_script_id.as_ref() {
            // Talk script setting
            chara.trigger_talk = Some(talk_script_id.to_owned());
        }

        let cid = gd.add_chara_to_site(chara, sid, uc.n);
        gd.region.get_map_mut(mid).locate_chara(cid, uc.pos);
    }
}

/// Create site from SiteGenObect and add it to region map
pub fn add_site_from_obj(
    gd: &mut GameData,
    rid: RegionId,
    pos: Vec2d,
    site_id: &str,
) -> Option<SiteId> {
    let sg: &SiteGenObject = gobj::get_by_id(site_id);
    let mut site = Site::new(sg.map_template_id.len() as u32, Some(site_id.to_owned()));
    let site_content = SiteContent::Other;
    site.content = site_content;
    let sid = if let Some(sid) = gd.add_site(site, sg.kind, rid, pos) {
        sid
    } else {
        warn!(
            "{:?} is already occupied, and failed to add a new town",
            pos
        );
        return None;
    };

    for map_template_id in &sg.map_template_id {
        let map = crate::game::map::from_template::from_template_id(map_template_id, true)
            .unwrap_or_else(|| panic!("Map template not found: {}", map_template_id));

        let map_random_id = gen_box_id(gd);
        gd.add_map(map, sid, map_random_id);
    }

    add_unique_citizens(gd, sid, sg);

    // Add symbol to region map
    let map = gd.region.get_map_mut(MapId::from(rid));
    map.tile[pos].special = SpecialTileKind::SiteSymbol {
        kind: sg.site_symbol,
    };

    Some(sid)
}
