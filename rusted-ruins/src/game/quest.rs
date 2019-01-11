//! Quest handlings

use common::gamedata::*;
use common::objholder::CharaTemplateIdx;

/// Update quest list of current town
pub fn update_town_quest(gd: &mut GameData) {
    let mid = gd.get_current_mapid();
    let town = match gd.region.get_site_mut(mid.sid()).content {
        SiteContent::Town { ref mut town } => town,
        _ => unreachable!(),
    };

    town.quests.clear();

    town.quests.push(gen_quest());
}

/// Returns available quest in the current town
pub fn available_quests(gd: &GameData) -> &[Quest] {
    let mid = gd.get_current_mapid();
    let town = match gd.region.get_site(mid.sid()).content {
        SiteContent::Town { ref town } => town,
        _ => unreachable!(),
    };

    town.quests.as_ref()
}

/// Generate an quest
fn gen_quest() -> Quest {
    Quest::SlayMonsters {
        idx: CharaTemplateIdx::default(),
        goal: 10,
        killed: 0,
    }
}

