mod moving;
mod use_tool;

use super::Game;
use crate::game::{AdvanceScriptResult, DialogOpenRequest, InfoGetter};
use common::gamedata::*;
use geom::*;

/// Player actions are processed through this.
/// Mutable access to Game or GameData is limited by this wrapper.
pub struct DoPlayerAction<'a>(pub(super) &'a mut Game);

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

    pub fn shoot(&mut self, target: Vec2d) {
        let map = self.gd().get_current_map();
        if let Some(target_id) = map.get_chara(target) {
            if super::action::shot_target(&mut self.0, CharaId::Player, target_id) {
                self.0.finish_player_turn();
            }
        }
    }

    /// Pick up an item on tile
    pub fn pick_up_item(&mut self, il: ItemLocation, n: u32) -> bool {
        let gd = self.gd_mut();
        let player_item_list_location = ItemListLocation::Chara {
            cid: CharaId::Player,
        };
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

    /// Use one item
    pub fn use_item(&mut self, il: ItemLocation) {
        super::action::use_item::use_item(self.gd_mut(), il, CharaId::Player);
        self.0.finish_player_turn();
    }

    /// Release one magic device item
    pub fn release_item(&mut self, il: ItemLocation) {
        super::action::release_item(self.0, il, CharaId::Player);
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
    pub fn change_equipment(&mut self, cid: CharaId, slot: (EquipSlotKind, u8), il: ItemLocation) {
        super::item::change_equipment(self.gd_mut(), cid, slot, il)
    }

    /// Try talk to next chara
    /// If success, returns id of the talk script
    pub fn try_talk(&mut self, dir: Direction) {
        if dir.as_vec() == (0, 0) {
            return;
        }

        let mut trigger_talk = None;
        let mut cid = None;
        {
            let gd = self.gd();
            let dest_tile = gd.get_current_map().chara_pos(CharaId::Player).unwrap() + dir.as_vec();
            if let Some(other_chara) = gd.get_current_map().get_chara(dest_tile) {
                cid = Some(other_chara);
                let relation = gd.chara_relation(CharaId::Player, other_chara);
                let other_chara = gd.chara.get(other_chara);
                match relation {
                    Relationship::ALLY | Relationship::FRIENDLY | Relationship::NEUTRAL => {
                        if let Some(ref t) = other_chara.trigger_talk {
                            trigger_talk = Some(t.clone())
                        }
                    }
                    _ => (),
                }
            }
        }
        if let Some(trigger_talk) = trigger_talk {
            self.0.start_script(&trigger_talk, cid);
        }
    }

    /// Advance current talk. Give player's choice if the talk has choices.
    /// If returns new text, continue talk dialog.
    pub fn advance_talk(&mut self, choice: Option<u32>) -> AdvanceScriptResult {
        self.0.advance_script(Some(choice))
    }

    /// Shotcut to Game::advance_talk
    pub fn advance_script(&mut self) -> AdvanceScriptResult {
        self.0.advance_script(None)
    }

    /// Undertake quest
    pub fn undertake_quest(&mut self, i: u32) {
        crate::game::quest::undertake_quest(self.0, i);
    }

    pub fn request_dialog_open(&mut self, req: DialogOpenRequest) {
        self.0.request_dialog_open(req);
    }

    pub fn start_creation(&mut self, recipe: &Recipe, il: Vec<ItemLocation>) {
        super::creation::start_creation(self.0, recipe, il);
        self.0.finish_player_turn();
    }

    pub fn exec_debug_command(&mut self, command: &str) {
        super::debug_command::exec_debug_command(self.0, command);
    }

    /// Print infomation of specified tile
    pub fn print_tile_info(&mut self, tile: Vec2d) {
        super::map::tile_info::print_tile_info(self.0, tile);
    }

    // pub fn harvest_item(&mut self, il: ItemLocation) {
    //     super::action::harvest::harvest_item(self.gd_mut(), il);
    // }
}
