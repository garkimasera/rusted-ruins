use crate::game::extrait::CharaExt;
use crate::game::{Game, InfoGetter};
use common::gamedata::*;
use rules::RULES;

pub fn restore_hp(game: &mut Game, cid: CharaId, power: f32) {
    let value = (RULES.effect.restore_hp_factor * power) as i32;
    let pos = game.gd.chara_pos(cid).unwrap();
    let chara = game.gd.chara.get_mut(cid);
    chara.heal(value);
    crate::damage_popup::push(cid, pos, crate::damage_popup::PopupKind::Heal(value));
}
