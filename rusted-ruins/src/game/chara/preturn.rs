use super::Game;
use crate::game::combat::DamageKind;
use crate::game::extrait::*;
use common::gamedata::*;
use rules::RULES;

/// This function will be called before the character's turn
///
pub fn preturn(game: &mut Game, cid: CharaId) -> bool {
    let mut is_poisoned = false;

    {
        let chara = game.gd.chara.get_mut(cid);

        // Process character status
        for s in chara.status.iter_mut() {
            s.advance_turn(1);
        }

        chara.status.retain(|s| !s.is_expired()); // Remove expired status

        for s in chara.status.iter() {
            match *s {
                CharaStatus::Poisoned => {
                    is_poisoned = true;
                }
                _ => (),
            }
        }
    }

    if is_poisoned {
        let chara = game.gd.chara.get_mut(cid);
        let damage = chara.attr.max_hp / 20;
        game_log!("poison-damage"; chara=chara, damage=damage);
        super::damage(game, cid, damage, DamageKind::Poison);
    }

    let chara = game.gd.chara.get_mut(cid);
    chara.add_sp(-RULES.chara.sp_consumption, cid);
    can_act(chara)
}

/// Judges this character can act or not
fn can_act(chara: &Chara) -> bool {
    for s in chara.status.iter() {
        match *s {
            CharaStatus::Asleep { .. } => {
                game_log_i!("asleep"; chara=chara);
                return false;
            }
            _ => (),
        }
    }
    true
}
