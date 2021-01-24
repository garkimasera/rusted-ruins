use crate::game::extrait::CharaEx;
use crate::game::Game;
use common::gamedata::*;
use rules::RULES;

// Melee attack to a chara.
pub fn recover_hp(game: &mut Game, cid: CharaId, power: f32) {
    let value = RULES.effect.recover_hp_factor * power;
    let chara = game.gd.chara.get_mut(cid);
    chara.heal(value as i32);
}
