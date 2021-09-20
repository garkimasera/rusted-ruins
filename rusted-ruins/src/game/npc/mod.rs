//! Functions for NPC's AI and actions

pub mod map_search;

use crate::game::action::shoot_target;

use super::action;
use super::active_skill::use_active_skill;
use super::extrait::*;
use super::{Game, InfoGetter};
use common::gamedata::*;
use common::gobj;
use geom::*;
use rng::*;
use rules::{npc_ai::*, RULES};

pub fn process_npc_turn(game: &mut Game<'_>, cid: CharaId) {
    match game.gd.chara.get(cid).ai.state {
        AiState::Normal => process_npc_turn_normal(game, cid),
        AiState::Combat { .. } => process_npc_turn_combat(game, cid),
        AiState::Search { .. } => process_npc_turn_search(game, cid),
    }
}

fn process_npc_turn_normal(game: &mut Game<'_>, cid: CharaId) {
    move_normal(game, cid);
}

fn process_npc_turn_combat(game: &mut Game<'_>, cid: CharaId) {
    let chara = game.gd.chara.get(cid);
    let ai = &chara.ai;
    let ai_rule = RULES.npc_ai.get(ai.kind);
    let ct: &CharaTemplateObject = gobj::get_obj(chara.idx);

    let target = match chara.ai.state {
        AiState::Combat { target, .. } => target,
        _ => unreachable!(),
    };

    let mut enable_active_skill = true;

    for _ in 0..4 {
        let action_kind = rng::choose(CombatActionKind::ALL, |kind| {
            if *kind == CombatActionKind::ActiveSkill && !enable_active_skill {
                0.0
            } else {
                ai_rule.combat_prob.get(kind).copied().unwrap_or(0.0)
            }
        })
        .map(|a| *a.1)
        .unwrap_or(CombatActionKind::Skip);

        match action_kind {
            CombatActionKind::Skip => (),
            CombatActionKind::ApproachEnemy => {
                move_to_target_enemy(game, cid, ai_rule, target);
            }
            CombatActionKind::RangedWeapon => {
                shoot_target(game, cid, target);
            }
            CombatActionKind::ActiveSkill => {
                if let Some(active_skill_id) = ct.active_skills.choose(&mut get_rng()) {
                    use_active_skill(game, active_skill_id, cid, target);
                } else {
                    enable_active_skill = false;
                    continue;
                }
            }
        }

        break;
    }
}

fn process_npc_turn_search(game: &mut Game<'_>, cid: CharaId) {
    let view_range = game.gd.chara.get(cid).attr.view_range;
    if let Some(target) = crate::game::map::search::search_nearest_target(
        &game.gd,
        cid,
        Relationship::Hostile,
        view_range,
    ) {
        game.gd.chara.get_mut(cid).ai.state = AiState::Combat { target };
        process_npc_turn_combat(game, cid);
        return;
    }

    let chara = game.gd.chara.get_mut(cid);
    let ai_rule = RULES.npc_ai.get(chara.ai.kind);

    let turn_count = match &mut chara.ai.state {
        AiState::Search { ref mut turn_count } => turn_count,
        _ => unreachable!(),
    };
    *turn_count += 1;

    if ai_rule.search_turn < *turn_count {
        chara.ai.state = AiState::Normal;
    }
    move_normal(game, cid);
}

/// Move when normal state or missing target
fn move_normal(game: &mut Game<'_>, cid: CharaId) {
    if game.gd.player.party.contains(&cid) {
        follow_other(game, cid, CharaId::Player);
    }

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

/// Move npc at random
fn random_walk(game: &mut Game<'_>, cid: CharaId) {
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
fn move_to_target_enemy(game: &mut Game<'_>, cid: CharaId, ai_rule: &NpcAi, target: CharaId) {
    if let Some(dir) = dir_to_chara(&game.gd, cid, target, ai_rule.pathfinding_step) {
        action::try_move(game, cid, dir);
    } else {
        game.gd.chara.get_mut(cid).ai.state = AiState::default_search();
        move_normal(game, cid);
    }
}

/// Follow other chara
fn follow_other(game: &mut Game<'_>, cid: CharaId, target: CharaId) {
    let (pos, target_pos) = if let (Some(pos), Some(target_pos)) =
        (game.gd.chara_pos(cid), game.gd.chara_pos(target))
    {
        (pos, target_pos)
    } else {
        return;
    };

    if pos.is_adjacent(target_pos) {
        return;
    }

    let dir = dir_to_chara(&game.gd, cid, target, RULES.npc.party_pathfinding_step)
        .unwrap_or(Direction::NONE);
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
