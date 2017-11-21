
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

    pub fn gd_mut(&mut self) -> &mut GameData {
        &mut self.0.gd
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
        let gd = self.gd_mut();
        
        // Use stairs
        if dir.is_none() {

            let mid = gd.get_current_mapid();
            let next_mid = {
                let special_tile_kind
                    = &gd.get_current_map().tile[gd.player_pos()].special;
                match special_tile_kind {
                    &SpecialTileKind::DownStairs => {
                        if gd.site.get(mid.sid).get_dungeon_kind().is_underground() {
                            Some(mid.inc_floor())
                        } else {
                            mid.dec_floor()
                        }
                    }
                    &SpecialTileKind::UpStairs => {
                        if gd.site.get(mid.sid).get_dungeon_kind().is_underground() {
                            mid.dec_floor()
                        } else {
                            Some(mid.inc_floor())
                        }
                    }
                    _ => { panic!("Try to use not exist stairs") }
                }
            };

            if next_mid.is_none() {
                return;
            }
            let next_mid = next_mid.unwrap();

            if gd.site.get_map_checked(next_mid).is_none() { // If next_mid floor doesn't exist
                super::site::extend_site_floor(gd, next_mid.sid);
            }

            super::map::switch_map(gd, next_mid);
            return;
        }
    }

    /// Pick up an item on tile
    pub fn pick_up_item(&mut self, item_idx: usize, n: u32) -> bool {
        let gd = self.gd_mut();
        let mid = gd.get_current_mapid();
        let player_pos = gd.player_pos();
        let item_list_src = gd.site.get_map_mut(mid).tile[player_pos].item_list.as_mut().unwrap();
        let item_list_dest = &mut gd.chara.get_mut(CharaId::Player).item_list;
        item_list_src.move_to(item_list_dest, item_idx, n);
        
        true
    }
}



