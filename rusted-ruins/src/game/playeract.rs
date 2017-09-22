
use super::Game;
use super::action;
use game::CharaId;
use array2d::*;

/// Player actions are processed through this.
pub struct DoPlayerAction<'a>(&'a mut Game);

impl<'a> DoPlayerAction<'a> {
    pub fn new(game: &'a mut Game) -> DoPlayerAction<'a> {
        DoPlayerAction(game)
    }

    pub fn try_move(&mut self, dir: Direction) {
        if action::try_move(self.0, CharaId::Player, dir) {
            self.0.finish_player_turn();
        }
    }
}

