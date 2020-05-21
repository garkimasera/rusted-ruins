use super::Game;
use crate::game::combat::DamageKind;
use crate::game::extrait::*;
use common::gamedata::*;
use rng::{dice, get_rng, Rng};
use rules::RULES;

/// This function will be called before the character's turn
///
pub fn preturn(game: &mut Game, cid: CharaId) -> bool {
    let mut is_poisoned = false;

    let chara = game.gd.chara.get_mut(cid);
    chara.update();

    // Process character status
    for s in chara.status.iter_mut() {
        s.advance_turn(1);
    }

    // If there is expired status, process expire routines.
    let mut expired_status = Vec::new();
    if chara.status.iter().any(|s| s.is_expired()) {
        for s in std::mem::replace(&mut chara.status, Vec::new()).into_iter() {
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
    for s in chara.status.iter() {
        match *s {
            CharaStatus::Poisoned => {
                is_poisoned = true;
            }
            _ => (),
        }
    }

    if is_poisoned {
        let chara = game.gd.chara.get_mut(cid);
        let damage = chara.attr.max_hp / 20;
        game_log!("poison-damage"; chara=chara, damage=damage);
        chara.damage(damage, DamageKind::Poison);
    }

    let chara = game.gd.chara.get_mut(cid);

    if chara.hp < chara.attr.max_hp && chara.sp > RULES.chara.sp_starving {
        // HP regeneration
        let lv = chara.skills.get(SkillKind::Healing) as f32;
        if get_rng().gen_bool(RULES.chara.hp_regeneration_probability.into()) {
            let a = (lv * RULES.chara.hp_regeneration_factor) as i32;
            let v = dice(1, a);
            chara.heal(v);
        }
        chara.sub_sp(RULES.chara.sp_consumption_regen, cid);
        chara.add_healing_exp();
    } else {
        chara.sub_sp(RULES.chara.sp_consumption, cid);
    }

    can_act(chara)
}

/// Judges this character can act or not
fn can_act(chara: &Chara) -> bool {
    if chara.hp < 0 {
        return false;
    }

    for s in chara.status.iter() {
        match *s {
            CharaStatus::Asleep { .. } => {
                game_log_i!("asleep"; chara=chara);
                return false;
            }
            CharaStatus::Creation { .. } => {
                return false;
            }
            _ => (),
        }
    }
    true
}
