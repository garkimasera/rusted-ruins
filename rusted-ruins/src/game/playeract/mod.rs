mod moving;
mod restart;
mod shortcut;
mod use_tool;

use super::{Game, UiRequest};
use crate::game::extrait::*;
use crate::game::target::{auto_target_for_player, Target};
use crate::game::{AdvanceScriptResult, DialogOpenRequest, InfoGetter};
use common::gamedata::*;
use common::objholder::ItemIdx;
use geom::*;

/// Player actions are processed through this.
/// Mutable access to Game or GameData is limited by this wrapper.
pub struct DoPlayerAction<'a, 's>(pub(super) &'a mut Game<'s>);

impl<'a, 's> DoPlayerAction<'a, 's> {
    pub fn new(game: &'a mut Game<'s>) -> DoPlayerAction<'a, 's> {
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
            if super::action::shoot_target(&mut self.0, CharaId::Player, target_id) {
                self.0.finish_player_turn();
            }
        }
    }

    pub fn set_target(&mut self, pos: Vec2d) -> bool {
        self.0.set_target(pos)
    }

    /// Pick up an item on tile
    pub fn pick_up_item<T: Into<ItemMoveNum>>(&mut self, il: ItemLocation, n: T) -> bool {
        let gd = self.gd_mut();
        let item = gd.get_item(il).0;

        if item.flags.contains(ItemFlags::FIXED) {
            game_log_i!("item-pick-up-fixed"; item=item);
            return false;
        }
        if item.flags.contains(ItemFlags::OWNED) {
            game_log_i!("item-owned-by-others"; item=item);
            return false;
        }
        if item.flags.contains(ItemFlags::PLANT) {
            game_log_i!("item-pick-up-plant"; item=item);
            return false;
        }

        game_log_i!("item-pickup"; chara=gd.chara.get(CharaId::Player), item=item);
        super::action::get_item::get_item(gd, il, CharaId::Player, n);
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
        gd.chara.get_mut(CharaId::Player).update();
        true
    }

    /// Throw one item
    pub fn throw_item(&mut self, il: ItemLocation) {
        let effect = crate::game::item::throw::item_to_throw_effect(self.gd(), il, CharaId::Player);
        let target = if let Ok(Some(target)) = auto_target_for_player(self.0, &effect) {
            target
        } else {
            self.0.ui_request.push_back(UiRequest::StartTargeting {
                effect: effect.clone(),
                callback: Box::new(move |pa, target| {
                    super::action::throw_item(pa.0, il, CharaId::Player, target);
                    pa.0.finish_player_turn();
                }),
            });
            return;
        };
        super::action::throw_item(self.0, il, CharaId::Player, target);
        self.0.finish_player_turn();
    }

    /// Drink one item
    pub fn drink_item(&mut self, il: ItemLocation) {
        super::action::drink_item(self.0, il, CharaId::Player);
        self.0.finish_player_turn();
    }

    /// Eat one item
    pub fn eat_item(&mut self, il: ItemLocation) {
        super::action::eat_item(self.0, il, CharaId::Player);
        self.0.finish_player_turn();
    }

    /// Use one item
    pub fn use_item(&mut self, il: ItemLocation) {
        let item_obj = self.gd().get_item(il).0.obj();
        let target = if let Some(UseEffect::Effect(effect)) = item_obj.use_effect.as_ref() {
            if let Ok(Some(target)) = auto_target_for_player(self.0, effect) {
                target
            } else {
                self.0.ui_request.push_back(UiRequest::StartTargeting {
                    effect: effect.clone(),
                    callback: Box::new(move |pa, target| {
                        super::action::use_item::use_item(pa.0, il, CharaId::Player, target);
                        pa.0.finish_player_turn();
                    }),
                });
                return;
            }
        } else {
            Target::None
        };
        super::action::use_item::use_item(self.0, il, CharaId::Player, target);
        self.0.finish_player_turn();
    }

    /// Read item, returns continue dialog or not.
    pub fn read_item(&mut self, il: ItemLocation) -> bool {
        use crate::game::creation::LearnRecipeResult;

        let title = self.gd().get_item(il).0.title().unwrap().to_owned();
        match crate::game::creation::learn_recipe(self.gd_mut(), il) {
            LearnRecipeResult::Success => {
                self.0.finish_player_turn();
                return false;
            }
            LearnRecipeResult::NoAvailableRecipe => {
                return true;
            }
            _ => (),
        }
        self.request_dialog_open(DialogOpenRequest::Read { title });
        true
    }

    /// Release one magic device item
    pub fn release_item(&mut self, il: ItemLocation) {
        let item_obj = self.gd().get_item(il).0.obj();
        let effect = if let Some(effect) = item_obj.magical_effect.as_ref() {
            effect
        } else {
            error!("release item that doesn't have effect");
            return;
        };
        let target = if let Ok(Some(target)) = auto_target_for_player(self.0, effect) {
            target
        } else {
            self.0.ui_request.push_back(UiRequest::StartTargeting {
                effect: effect.clone(),
                callback: Box::new(move |pa, target| {
                    super::action::release_item(pa.0, il, CharaId::Player, target);
                    pa.0.finish_player_turn();
                }),
            });
            return;
        };
        super::action::release_item(self.0, il, CharaId::Player, target);
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
                    Relationship::Ally | Relationship::Friendly | Relationship::Neutral => {
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

    pub fn harvest_item(&mut self, il: ItemLocation) {
        if crate::game::action::harvest::harvest_item(self.gd_mut(), il) {
            self.0.finish_player_turn();
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

    pub fn start_creation(
        &mut self,
        kind: CreationKind,
        recipe: &Recipe,
        ill: ItemListLocation,
        prior_high_quality: bool,
        material_to_use: Option<ItemIdx>,
    ) {
        super::creation::start_creation(
            self.0,
            kind,
            recipe,
            ill,
            prior_high_quality,
            material_to_use,
        );
        self.0.finish_player_turn();
    }

    pub fn exec_debug_command(&mut self, command: &str) {
        super::debug_command::exec_debug_command(self.0, command);
    }

    /// Print infomation of specified tile
    pub fn print_tile_info(&mut self, tile: Vec2d) {
        // Open StatusWindow for selected character
        let cid = self.gd().get_current_map().get_chara(tile);
        if cid.is_some() && cid.unwrap() != CharaId::Player {
            let cid = cid.unwrap();
            let scanned = self
                .gd()
                .chara
                .get(cid)
                .status
                .iter()
                .any(|status| *status == CharaStatus::Scanned);

            if scanned {
                self.0
                    .request_dialog_open(DialogOpenRequest::CharaStatus { cid });
            } else {
                let chara = self.gd().chara.get(cid);
                game_log_i!("not-scanned"; chara=chara);
                return;
            }
        }

        super::map::tile_info::print_tile_info(self.0, tile);
    }

    // pub fn harvest_item(&mut self, il: ItemLocation) {
    //     super::action::harvest::harvest_item(self.gd_mut(), il);
    // }
}
