
pub mod creation;

use super::Game;
use common::gamedata::chara::CharaId;

pub fn damage(game: &mut Game, target: CharaId, damage: i32) {
    let target = game.gd.chara.get_mut(target);
    
    target.hp -= damage;
}

