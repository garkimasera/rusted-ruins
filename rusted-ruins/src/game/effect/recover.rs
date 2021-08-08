use crate::game::extrait::CharaExt;
use crate::game::{Game, InfoGetter};
use common::gamedata::*;
use rules::RULES;

// Melee attack to a chara.
pub fn recover_hp(game: &mut Game<'_>, cid: CharaId, power: f32) {
    let value = (RULES.effect.recover_hp_factor * power) as i32;
    let pos = game.gd.chara_pos(cid).unwrap();
    let chara = game.gd.chara.get_mut(cid);
    chara.heal(value);
    crate::chara_log::get_log_mut().push_damage(cid, pos, -value);
}
