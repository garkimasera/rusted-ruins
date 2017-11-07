
use super::Game;
use super::action;
use common::gamedata::GameData;
use common::gamedata::chara::CharaId;
use common::gamedata::map::SpecialTileKind;
use game::InfoGetter;
use array2d::*;

/// Player actions are processed through this.
pub struct DoPlayerAction<'a>(&'a mut Game);

impl<'a> DoPlayerAction<'a> {
    pub fn new(game: &'a mut Game) -> DoPlayerAction<'a> {
        DoPlayerAction(game)
    }

    pub fn gd(&self) -> &GameData {
        &self.0.gd
    }

    pub fn try_move(&mut self, dir: Direction) {
        if action::try_move(self.0, CharaId::Player, dir) {
            self.0.finish_player_turn();
        }
    }

    /// Try to go to next floor
    /// This function will be called when players use stairs or try to exit from map borders.
    /// In the latter case, dir is not None and represents player's move direction.
    pub fn goto_next_floor(&mut self, dir: Direction) {
        // Use stairs
        if dir.is_none() {
            let special_tile_kind
                = &self.gd().get_current_map().tile[self.gd().player_pos()].special;

            match special_tile_kind {
                &SpecialTileKind::DownStairs => {

                }
                &SpecialTileKind::UpStairs => {

                }
                _ => { panic!("Try to use not exist stairs") }
            }
            return;
        }
    }
}



