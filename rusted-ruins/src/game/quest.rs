//! Quest handlings

use common::gamedata::*;
use common::objholder::CharaTemplateIdx;
use rules::RULES;
use super::Game;
use super::chara::gen::choose_npc_chara_template;

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

/// Undertake quest in the current town
pub fn undertake_quest(game: &mut Game, i: u32) {
    let mid = game.gd.get_current_mapid();
    let town = match game.gd.region.get_site_mut(mid.sid()).content {
        SiteContent::Town { ref mut town } => town,
        _ => unreachable!(),
    };

    let quest = town.quests.remove(i as usize);
    game.gd.quest.start_new_quest(quest);
}

/// Generate an quest
fn gen_quest() -> Quest {
    Quest::SlayMonsters {
        idx: choose_npc_chara_template(
            &RULES.quest.slay_race_probability,
            1),
        goal: 10,
        killed: 0,
    }
}

pub fn count_slayed_monster(gd: &mut GameData, t: CharaTemplateIdx) {

    for (state, quest) in &mut gd.quest.iter_mut() {
        match quest {
            Quest::SlayMonsters { idx, goal, killed } => {
                if *state == QuestState::Active && *idx == t {
                    *killed += 1;
                    if *killed == *goal {
                        *state = QuestState::Completed;
                        // Log
                        game_log_i!("quest-complete-slay_monsters"; monster=idx, n=goal);
                    }
                }
            }
        }
    }
}

