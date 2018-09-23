
use super::Game;
use super::action;
use common::gamedata::*;
use game::{InfoGetter, DialogOpenRequest};
use array2d::*;

/// Player actions are processed through this.
/// Mutable access to Game or GameData is limited by this wrapper.
pub struct DoPlayerAction<'a>(pub(in super) &'a mut Game);

impl<'a> DoPlayerAction<'a> {
    pub fn new(game: &'a mut Game) -> DoPlayerAction<'a> {
        DoPlayerAction(game)
    }

    pub fn game(&self) -> &Game {
        &self.0
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
            let (next_mid, msg) = {
                let gd = self.gd_mut();
                let mid = gd.get_current_mapid();
                let special_tile_kind
                    = &gd.get_current_map().tile[gd.player_pos()].special;
                match *special_tile_kind {
                    SpecialTileKind::Stairs { dest_floor, .. } => { // Use stairs on map
                        let mid = if dest_floor == FLOOR_OUTSIDE {
                            MapId::from(mid.rid())
                        } else {
                            mid.set_floor(dest_floor)
                        };
                        (mid, msg_switch_map(mid))
                    }
                    SpecialTileKind::SiteSymbol { .. } => { // Enter other site from region map
                        let pos = gd.player_pos();
                        let region = gd.region.get(mid.rid());
                        if let Some(sid) = region.get_id_by_pos(pos) {
                            let mid = MapId::site_first_floor(sid);
                            let site = gd.region.get_site(sid);
                            let msg = replace_str!(
                                ::text::ui_txt("dialog.enter_site");
                                site_name=site);
                            (mid, msg.into())
                        } else {
                            warn!("No site existed at {:?}", pos);
                            return;
                        }
                    }
                    _ => {
                        warn!("Tried to use stairs that don't exist");
                        return;
                    }
                }
            };

            let cb = Box::new(move |pa: &mut DoPlayerAction, result: bool| {
                if !result { return; }
                super::map::switch_map(pa.0, next_mid);
            });
            self.0.request_dialog_open(DialogOpenRequest::YesNo {
                callback: cb, msg: msg,
            });
            
            return;
        } else { // Crossing boundary
            use common::gamedata::map::BoundaryBehavior;
            let boundary = {
                let player_pos = self.gd().player_pos();
                let map = self.gd().get_current_map();
                let b = map.get_boundary_by_tile_and_dir(player_pos, dir);
                if let Some(b) = b { b } else { return; }
            };
            let next_mid = match boundary {
                BoundaryBehavior::None => { return; },
                BoundaryBehavior::RegionMap => {
                    let mid = MapId::from(self.gd().get_current_mapid().rid());
                    mid
                }
                BoundaryBehavior::Floor(floor) => {
                    let mid = self.gd().get_current_mapid().set_floor(floor);
                    mid
                }
                BoundaryBehavior::MapId(_, _) => { unimplemented!() }
            };
            let cb = Box::new(move |pa: &mut DoPlayerAction, result: bool| {
                if !result { return; }
                super::map::switch_map(pa.0, next_mid);
            });
            self.0.request_dialog_open(DialogOpenRequest::YesNo {
                callback: cb, msg: msg_switch_map(next_mid),
            });
        }
    }

    /// Shot to target using long range weapon
    pub fn shot(&mut self) {
        if let Some(target) = self.0.target_chara {
            if super::action::shot_target(&mut self.0, CharaId::Player, target) {
                self.0.finish_player_turn();
            }
        }
    }

    /// Pick up an item on tile
    pub fn pick_up_item(&mut self, il: ItemLocation, n: u32) -> bool {
        let gd = self.gd_mut();
        let player_item_list_location = ItemListLocation::Chara { cid: CharaId::Player };
        game_log_i!("item-pickup"; chara=gd.chara.get(CharaId::Player), item=gd.get_item(il).0);
        gd.move_item(il, player_item_list_location, n);
        true
    }

    /// Drop items on tile
    pub fn drop_item(&mut self, il: ItemLocation, n: u32) -> bool {
        let gd = self.gd_mut();
        let tile_list_location = ItemListLocation::OnMap {
            mid: gd.get_current_mapid(),
            pos: gd.player_pos(),
        };
        game_log_i!("item-drop"; chara=gd.chara.get(CharaId::Player), item=gd.get_item(il).0);
        gd.move_item(il, tile_list_location, n);
        true
    }

    /// Drink one item
    pub fn drink_item(&mut self, il: ItemLocation) {
        super::action::drink_item(self.gd_mut(), il, CharaId::Player);
        self.0.finish_player_turn();
    }

    /// Eat one item
    pub fn eat_item(&mut self, il: ItemLocation) {
        super::action::eat_item(self.gd_mut(), il, CharaId::Player);
        self.0.finish_player_turn();
    }

    /// Buy item
    pub fn buy_item(&mut self, il: ItemLocation) {
        super::shop::buy_item(self.gd_mut(), il);
    }

    /// Sell item
    pub fn sell_item(&mut self, il: ItemLocation) {
        super::shop::sell_item(self.gd_mut(), il);
    }

    /// Change specified character's equipment by given item
    pub fn change_equipment(&mut self, cid: CharaId, slot: (EquipSlotKind, u8), il: ItemLocation) -> bool {
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

pub fn msg_switch_map(next_mid: MapId) -> ::std::borrow::Cow<'static, str> {
    if next_mid.is_region_map() {
        ::text::ui_txt("dialog.exit_to_regionmap").into()
    } else {
        ::text::ui_txt("dialog.move_floor").into()
    }
}

