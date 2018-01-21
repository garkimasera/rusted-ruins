
use super::Game;
use super::action;
use common::gamedata::{self, GameData};
use common::gamedata::chara::{CharaId, Relationship};
use common::gamedata::map::{MapId, SpecialTileKind};
use common::gamedata::item::*;
use game::{InfoGetter, DialogOpenRequest};
use array2d::*;

/// Player actions are processed through this.
pub struct DoPlayerAction<'a>(pub(in super) &'a mut Game);

impl<'a> DoPlayerAction<'a> {
    pub fn new(game: &'a mut Game) -> DoPlayerAction<'a> {
        DoPlayerAction(game)
    }

    pub fn gd(&self) -> &GameData {
        &self.0.gd
    }

    fn gd_mut(&mut self) -> &mut GameData {
        &mut self.0.gd
    }

    pub fn try_move(&mut self, dir: Direction) {
        let dest_tile = self.gd().get_current_map().chara_pos(CharaId::Player).unwrap() + dir.as_vec();
        // If there is friendy chara on target tile, and have a trigger to start talk
        let will_talk = {
            if dir.as_vec() != (0, 0) {
                let gd = self.gd();
                let player_chara = gd.chara.get(CharaId::Player);
                if let Some(other_chara) = gd.get_current_map().get_chara(dest_tile) {
                    let other_chara = gd.chara.get(other_chara);
                    match player_chara.rel.relative(other_chara.rel) {
                        Relationship::ALLY | Relationship::FRIENDLY => {
                            if !other_chara.talk.is_none() {
                                true
                            } else {
                                false
                            }
                        }
                        _ => false,
                    }
                } else {
                    false
                }
            } else {
                false
            }
        };
        if will_talk {
            self.try_talk(dir);
            return;
        }
        // If destination is out of boundary
        if !self.gd().get_current_map().is_inside(dest_tile) {
            self.goto_next_floor(dir);
            return;
        }
        // Move to the next tile
        if action::try_move(self.0, CharaId::Player, dir) {
            self.0.finish_player_turn();
        }
    }

    /// Try to go to next floor
    /// This function will be called when players use stairs or try to exit from map boundaries.
    /// In the latter case, dir is not None and represents player's move direction.
    pub fn goto_next_floor(&mut self, dir: Direction) {
        // Use stairs
        if dir.is_none() {
            let next_mid = {
                let gd = self.gd_mut();
                let mid = gd.get_current_mapid();
                let next_mid = {
                    let special_tile_kind
                        = &gd.get_current_map().tile[gd.player_pos()].special;
                    let region_map = MapId::from(mid.sid.rid);
                    match special_tile_kind {
                        &SpecialTileKind::DownStairs => {
                            if gd.region.get_site(mid.sid).is_underground() {
                                Some(mid.inc_floor())
                            } else {
                                if mid.floor == 0 { Some(region_map) } else { mid.dec_floor() }
                            }
                        }
                        &SpecialTileKind::UpStairs => {
                            if gd.region.get_site(mid.sid).is_underground() {
                                if mid.floor == 0 { Some(region_map) } else { mid.dec_floor() }
                            } else {
                                Some(mid.inc_floor())
                            }
                        }
                        _ => { panic!("Try to use not exist stairs") }
                    }
                };
                if next_mid.is_none() { return; }
                next_mid.unwrap()
            };

            let cb = Box::new(move |pa: &mut DoPlayerAction, result: bool| {
                if !result { return; }
                let gd = pa.gd_mut();
                if gd.region.get_map_checked(next_mid).is_none() { // If next_mid floor doesn't exist
                    super::site::extend_site_floor(gd, next_mid.sid);
                }
                super::map::switch_map(gd, next_mid);
            });
            self.0.request_dialog_open(DialogOpenRequest::YesNo {
                callback: cb, msg_text_id: "dialog.move_floor"
            });
            
            return;
        } else {
            let cb = Box::new(|pa: &mut DoPlayerAction, result: bool| {
                println!("{}", result);
            });
            self.0.request_dialog_open(DialogOpenRequest::YesNo {
                callback: cb, msg_text_id: ""
            });
        }
    }

    /// Pick up an item on tile
    pub fn pick_up_item(&mut self, il: gamedata::item::ItemLocation, n: u32) -> bool {
        let gd = self.gd_mut();
        let player_item_list_location = gamedata::item::ItemListLocation::Chara { cid: CharaId::Player };
        let item_name = super::item::get_item_name(gd.get_item(il).0);
        gd.move_item(il, player_item_list_location, n);
        game_log_i!("item-pickup"; chara=gd.chara.get(CharaId::Player).name, item=item_name);
        true
    }

    /// Change specified character's equipment by given item
    pub fn change_equipment(&mut self, cid: CharaId, slot: (ItemKind, u8), il: ItemLocation) -> bool {
        super::item::change_equipment(self.gd_mut(), cid, slot, il)
    }

    /// Try talk to next chara
    /// If success, returns id of the talk script
    pub fn try_talk(&mut self, dir: Direction) {
        if dir.as_vec() == (0, 0) { return; }

        let mut chara_talk = None;
        let mut cid = None;
        {
            let gd = self.gd();
            let player_chara = gd.chara.get(CharaId::Player);
            let dest_tile = gd.get_current_map().chara_pos(CharaId::Player).unwrap() + dir.as_vec();
            if let Some(other_chara) = gd.get_current_map().get_chara(dest_tile) {
                cid = Some(other_chara);
                let other_chara = gd.chara.get(other_chara);
                match player_chara.rel.relative(other_chara.rel) {
                    Relationship::ALLY | Relationship::FRIENDLY => {
                        if let Some(ref t) = other_chara.talk {
                            chara_talk = Some(t.clone())
                        }
                    }
                    _ => (),
                }
            }
        }
        if let Some(chara_talk) = chara_talk {
            self.0.request_dialog_open(DialogOpenRequest::Talk {
                chara_talk, cid: cid.unwrap(),
            });
        }
    }
}

