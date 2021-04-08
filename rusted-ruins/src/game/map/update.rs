use crate::game::Game;
use common::gamedata::*;
use rules::RULES;

/// Map update after map switching.
pub fn update_map(game: &mut Game) {
    crate::game::item::time::update_item_time(&mut game.gd);

    let map = game.gd.get_current_map_mut();
    let current_time = crate::game::time::current_time();
    let duration = current_time.duration_from(map.last_visit);
    map.last_visit = current_time;

    if duration > Duration::from_minutes(RULES.npc.map_switch_recover_minutes.into()) {
        recover_npc(&mut game.gd);
    }
}

pub fn recover_npc(gd: &mut GameData) {
    for cid in gd.get_charas_on_map().into_iter() {
        let chara = gd.chara.get_mut(cid);

        chara.ai.state = AiState::Normal;
    }
}
