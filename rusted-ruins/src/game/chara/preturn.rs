use super::Game;
use crate::game::damage::*;
use crate::game::extrait::*;
use crate::text::ToText;
use common::gamedata::*;
use geom::MDistRangeIter;
use rng::{get_rng, roll_dice, Rng};
use rules::RULES;

/// This function will be called before the character's turn
///
pub fn preturn(game: &mut Game, cid: CharaId) -> bool {
    awake_other_npc(game, cid);

    let chara = game.gd.chara.get_mut(cid);
    chara.update();

    // Process character status
    for s in chara.status.iter_mut() {
        s.advance_turn(1);
    }

    // If there is expired status, process expire routines.
    let mut expired_status = Vec::new();
    if chara.status.iter().any(|s| s.is_expired()) {
        for s in std::mem::take(&mut chara.status).into_iter() {
            if s.is_expired() {
                expired_status.push(s);
            } else {
                chara.status.push(s);
            }
        }
    }
    for s in expired_status.into_iter() {
        s.expire(&mut game.gd, cid);
    }

    let chara = game.gd.chara.get_mut(cid);
    let mut is_poisoned = false;
    let mut progress_anim = None;

    for s in chara.status.iter() {
        match *s {
            CharaStatus::Poisoned => {
                is_poisoned = true;
            }
            CharaStatus::Work {
                turn_left,
                needed_turn,
                ..
            } => {
                let ratio = turn_left as f32 / (needed_turn as f32 - 1.0);
                progress_anim = Some(ratio);
            }
            _ => (),
        }
    }

    if is_poisoned {
        let chara = game.gd.chara.get_mut(cid);
        let damage = chara.attr.max_hp / 20;
        game_log_i!("poison-damage"; chara=chara, damage=damage);
        do_damage(game, cid, damage, CharaDamageKind::Poison, None);
    }

    if let Some(ratio) = progress_anim {
        game.anim_queue.push_work(ratio);
    }

    let sp_consumption_factor = if cid == CharaId::Player {
        let factor =
            if game.gd.get_current_mapid().is_region_map() && crate::game::time::player_moved() {
                let regionmap_speed_factor = crate::game::time::regionmap_speed(&game.gd).1;
                RULES.chara.sp_consumption_factor_in_region_map / regionmap_speed_factor
            } else {
                1.0
            };
        crate::game::time::clear_player_moved();
        factor
    } else {
        1.0
    };

    let chara = game.gd.chara.get_mut(cid);
    chara.add_carry_exp();

    if chara.hp < chara.attr.max_hp && chara.sp > RULES.chara.sp_starving {
        // HP regeneration
        let lv = chara.skill_level(SkillKind::Endurance) as f32;
        let p = (RULES.chara.hp_regeneration_probability * chara.attr.vit as f32).clamp(0.0, 1.0);
        if get_rng().gen_bool(p.into()) {
            let a = (lv * RULES.chara.hp_regeneration_factor) as i32;
            let v = roll_dice(1, a);
            chara.heal(v);
        }
        chara.sub_sp(
            RULES.chara.sp_consumption_regen * sp_consumption_factor,
            cid,
        );
        chara.add_regeneration_exp();
    } else {
        let damage = chara.sub_sp(RULES.chara.sp_consumption * sp_consumption_factor, cid);
        if let Some(damage) = damage {
            do_damage(game, cid, damage, CharaDamageKind::Starve, None);
        }
    }

    let chara = game.gd.chara.get_mut(cid);
    if chara.mp < chara.attr.max_mp && chara.sp > RULES.chara.sp_starving {
        // MP regeneration
        let lv = chara.skill_level(SkillKind::Magic) as f32;
        let p = (RULES.chara.mp_regeneration_probability * chara.attr.wil as f32).clamp(0.0, 1.0);
        if get_rng().gen_bool(p.into()) {
            let a = (lv * RULES.chara.hp_regeneration_factor) as i32;
            let v = roll_dice(1, a);
            chara.add_mp(v);
        }
    }

    can_act(game.gd.chara.get_mut(cid))
}

/// Judges this character can act or not
fn can_act(chara: &Chara) -> bool {
    if chara.hp < 0 {
        return false;
    }

    for s in chara.status.iter() {
        match *s {
            CharaStatus::Asleep { .. } => {
                game_log!("asleep"; chara=chara);
                return false;
            }
            CharaStatus::Work { .. } => {
                return false;
            }
            _ => (),
        }
    }
    true
}

/// Awake enemy NPC to combat state.
fn awake_other_npc(game: &mut Game, cid: CharaId) {
    let center = if let Some(center) = game.gd.chara_pos(cid) {
        center
    } else {
        return;
    };
    let detection = game.gd.chara.get(cid).skill_level(SkillKind::Detection);
    let detection_range = RULES.combat.detection_range;
    let detection_factor = RULES.combat.detection_factor;

    for (distance, pos) in MDistRangeIter::new(center, detection_range) {
        let map = game.gd.get_current_map();
        if !map.is_inside(pos) {
            continue;
        }

        let other_cid = if let Some(other_cid) = map.tile[pos].chara {
            other_cid
        } else {
            continue;
        };

        if other_cid == cid
            || other_cid == CharaId::Player
            || game.gd.chara.get_mut(other_cid).ai.state.is_combat()
            || game.gd.chara_relation(cid, other_cid) != Relationship::Hostile
        {
            continue;
        }

        let conceal = game.gd.chara.get(other_cid).skill_level(SkillKind::Conceal);
        let distance_factor = 1.0 - (distance as f32 / detection_range as f32);
        let p = (detection as f32 / conceal as f32) * distance_factor * detection_factor;

        if p >= 1.0 || rng::gen_bool(p) {
            let other_npc = game.gd.chara.get_mut(other_cid);
            trace!("{:?} changed ai state to combat", other_npc.to_text());
            other_npc.ai.state = AiState::Combat { target: cid };
        }

        // Exp for conceal
        let chara = game.gd.chara.get_mut(cid);
        chara.add_skill_exp(SkillKind::Conceal, RULES.exp.conceal, detection);
    }
}
