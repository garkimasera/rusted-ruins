//! Quest handlings

use super::chara::gen::choose_npc_chara_template;
use super::Game;
use common::gamedata::*;
use common::objholder::CharaTemplateIdx;
use rules::RULES;

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
    let reward = Reward {
        money: 1000,
        item: Vec::new(),
    };

    Quest::SlayMonsters {
        reward,
        idx: choose_npc_chara_template(&RULES.quest.slay_race_probability, 1),
        goal: 10,
        killed: 0,
    }
}

pub fn count_slayed_monster(gd: &mut GameData, t: CharaTemplateIdx) {
    for (state, quest) in gd.quest.iter_mut() {
        match quest {
            Quest::SlayMonsters {
                idx, goal, killed, ..
            } => {
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

pub fn receive_rewards(gd: &mut GameData) -> bool {
    let mut money = 0;
    let mut exist_completed_quest = false;

    for (state, quest) in gd.quest.iter_mut() {
        if *state == QuestState::Completed {
            exist_completed_quest = true;
            let reward = quest.reward();
            money += reward.money;
            *state = QuestState::RewardReceived;
        }
    }

    if exist_completed_quest {
        gd.quest.remove_reward_received();
        gd.player.add_money(money);
        game_log_i!("quest-reward-receive-money"; money=money);
    }
    exist_completed_quest
}
