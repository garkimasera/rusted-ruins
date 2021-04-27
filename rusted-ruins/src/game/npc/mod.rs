//! Functions for NPC's AI and actions

pub mod map_search;

use super::action;
use super::active_skill::use_active_skill;
use super::extrait::*;
use super::{Game, InfoGetter};
use common::gamedata::*;
use common::gobj;
use geom::*;
use rng::*;
use rules::{npc_ai::*, RULES};

pub fn process_npc_turn(game: &mut Game, cid: CharaId) {
    match game.gd.chara.get(cid).ai.state {
        AiState::Normal => process_npc_turn_normal(game, cid),
        AiState::Combat { .. } => process_npc_turn_combat(game, cid),
    }
}

fn process_npc_turn_normal(game: &mut Game, cid: CharaId) {
    let chara = game.gd.chara.get(cid);
    let ai = &chara.ai;
    let ai_rule = RULES.npc_ai.get(ai.kind);

    match ai_rule.move_kind {
        MoveKind::NoMove => (),
        MoveKind::Wander => {
            if gen_bool(ai_rule.walk_prob) {
                random_walk(game, cid);
            }
        }
        MoveKind::Return => {
            let initial_pos = chara.ai.initial_pos;
            let pos = game.gd.chara_pos(cid).unwrap();
            if initial_pos != pos {
                if gen_range(0..1) == 0 {
                    let dir = geom::dir_by_2pos(pos, initial_pos);
                    action::try_move(game, cid, dir);
                }
            } else if gen_bool(ai_rule.walk_prob) {
                random_walk(game, cid);
            }
        }
    }
}

fn process_npc_turn_combat(game: &mut Game, cid: CharaId) {
    let chara = game.gd.chara.get(cid);
    let ai = &chara.ai;
    let ai_rule = RULES.npc_ai.get(ai.kind);
    let ct: &CharaTemplateObject = gobj::get_obj(chara.template);

    let target = match chara.ai.state {
        AiState::Combat { target } => target,
        _ => unreachable!(),
    };
    if rng::gen_bool(ai_rule.approach_enemy_prob) {
        move_to_target_enemy(game, cid, ai_rule, target);
    } else if rng::gen_bool(ai_rule.active_skill_prob) {
        if let Some(active_skill_id) = ct.active_skills.choose(&mut get_rng()) {
            use_active_skill(game, active_skill_id, cid, target);
        } else {
            move_to_target_enemy(game, cid, ai_rule, target);
        }
    }
}

/// Move npc at random
fn random_walk(game: &mut Game, cid: CharaId) {
    let dir = Direction::new(
        *[HDirection::Left, HDirection::None, HDirection::Right]
            .choose(&mut get_rng())
            .unwrap(),
        *[VDirection::Up, VDirection::None, VDirection::Down]
            .choose(&mut get_rng())
            .unwrap(),
    );
    action::try_move(game, cid, dir);
}

/// Move npc to nearest enemy
fn move_to_target_enemy(game: &mut Game, cid: CharaId, ai_rule: &NpcAi, target: CharaId) {
    let dir =
        dir_to_chara(&game.gd, cid, target, ai_rule.pathfinding_step).unwrap_or(Direction::NONE);
    action::try_move(game, cid, dir);
}

/// Returns direction to target chara
fn dir_to_chara(
    gd: &GameData,
    cid: CharaId,
    target: CharaId,
    pathfinding_step: u32,
) -> Option<Direction> {
    let start_pos = gd.chara_pos(cid)?;
    let target_pos = gd.chara_pos(target)?;
    let map = gd.get_current_map();
    let chara = gd.chara.get(cid);

    let route = geom::PathFinding::new(map.w, map.h, pathfinding_step, |pos| {
        map.is_passable(chara, pos)
    })
    .route(start_pos, target_pos);

    let next_pos = route.and_then(|route| route.get(1).copied())?;
    Some(geom::dir_by_2pos(start_pos, next_pos))
}
