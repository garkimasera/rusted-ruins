
use super::Game;
use common::gamedata::chara::*;
use game::extrait::*;

/// This function will be called before the character's turn
pub fn preturn(game: &mut Game, cid: CharaId) {
    let chara = game.gd.chara.get_mut(cid);

    // Process character status
    for s in chara.status.iter_mut() {
        s.advance_turn(1);
    }
    chara.status.retain(|s| !s.is_expired()); // Remove expired status
}

