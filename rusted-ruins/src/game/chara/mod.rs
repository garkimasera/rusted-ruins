
pub mod creation;

use super::Game;
use super::combat::DamageKind;
use common::gamedata::chara::CharaId;

pub fn damage(game: &mut Game, target: CharaId, damage: i32, damage_kind: DamageKind) {
    let t = game.gd.chara.get_mut(target);
    
    t.hp -= damage;

    if t.hp < 0 {
        game.dying_charas.push(target);
        // Logging
        match damage_kind {
            DamageKind::CloseRangeAttack => {
                game_log!("killed-by-close-attack"; target=t.name);
            },
        }
    }
}

