use crate::game;
use crate::game::extrait::CharaExt;
use crate::game::saveload::gen_box_id;
use common::gamedata::*;
use common::gobj;
use common::obj::SiteGenObject;
use common::sitegen::NpcGenId;
use geom::*;

/// Add npcs from SiteGenObject
pub fn add_npcs(gd: &mut GameData, sid: SiteId, sg: &SiteGenObject) {
    for npc_gen in &sg.npcs {
        let cid = match npc_gen.id {
            NpcGenId::Site(id) => CharaId::OnSite { sid, id },
            NpcGenId::Unique(id) => CharaId::Unique { id },
        };

        if gd.chara.exist(cid) {
            let chara = gd.chara.get_mut(cid);
            chara.resurrect();
        } else {
            let idx = gobj::id_to_idx(&npc_gen.chara_template_id);
            let ct: &CharaTemplateObject = gobj::get_obj(idx);
            let faction = if !matches!(cid, CharaId::OnSite { .. }) && ct.faction.is_unknown() {
                None
            } else {
                Some(sg.default_faction_id)
            };

            let mut chara = game::chara::gen::create_chara(idx, 1, faction, None);
            chara.ai.initial_pos = npc_gen.pos;

            if !npc_gen.talk_script_id.is_empty() {
                // Talk script setting
                chara.trigger_talk = Some(npc_gen.talk_script_id.to_owned());
            }

            gd.add_chara(cid, chara);
        }

        let mid = MapId::SiteMap {
            sid,
            floor: npc_gen.floor,
        };
        gd.region.get_map_mut(mid).locate_chara(cid, npc_gen.pos);
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
    let sid = if let Some(sid) = gd.add_site(site, sg.kind, rid, Some(pos)) {
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

    add_npcs(gd, sid, sg);

    // Add symbol to region map
    let map = gd.region.get_map_mut(MapId::from(rid));
    map.tile[pos].special = SpecialTileKind::SiteSymbol {
        kind: sg.site_symbol,
    };

    crate::game::shop::update_shops(gd, sid, sg);

    Some(sid)
}
