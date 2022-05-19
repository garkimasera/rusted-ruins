mod moving;
mod restart;
mod shortcut;
mod use_tool;

use super::{Game, UiRequest};
use crate::game::extrait::*;
use crate::game::script_exec::AdvanceScriptResult;
use crate::game::target::{auto_target_for_player, Target};
use crate::game::{DialogOpenRequest, InfoGetter};
use common::gamedata::*;
use common::objholder::ItemIdx;
use geom::*;
use rules::RULES;

/// Player actions are processed through this.
/// Mutable access to Game or GameData is limited by this wrapper.
pub struct DoPlayerAction<'a>(pub(super) &'a mut Game);

impl<'a> DoPlayerAction<'a> {
    pub fn new(game: &'a mut Game) -> DoPlayerAction<'a> {
        DoPlayerAction(game)
    }

    pub fn game(&self) -> &Game {
        self.0
    }

    pub fn gd(&self) -> &GameData {
        &self.0.gd
    }

    fn gd_mut(&mut self) -> &mut GameData {
        &mut self.0.gd
    }

    pub fn shoot(&mut self, target: Coords) {
        let map = self.gd().get_current_map();
        if let Some(target_id) = map.get_chara(target) {
            if super::action::shoot_target(self.0, CharaId::Player, target_id) {
                self.0.finish_player_turn();
            }
        }
    }

    pub fn set_target(&mut self, pos: Coords) -> bool {
        self.0.set_target(pos)
    }

    /// Pick up an item on tile
    pub fn pick_up_item<T: Into<ItemMoveNum>>(&mut self, il: ItemLocation, n: T) -> bool {
        let gd = self.gd_mut();
        let item = gd.get_item(il).0;

        if item.flags.contains(ItemFlags::FIXED) {
            game_log!("item-pick-up-fixed"; item=item);
            return false;
        }
        if item.flags.contains(ItemFlags::OWNED) {
            game_log!("item-owned-by-others"; item=item);
            return false;
        }
        if item.flags.contains(ItemFlags::PLANT) {
            game_log!("item-pick-up-plant"; item=item);
            return false;
        }

        game_log!("item-pickup"; chara=gd.chara.get(CharaId::Player), item=item);
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
        game_log!("item-drop"; chara=gd.chara.get(CharaId::Player), item=gd.get_item(il).0);
        gd.move_item(il, tile_list_location, n);
        gd.chara.get_mut(CharaId::Player).update();
        true
    }

    /// Throw one item
    pub fn throw_item(&mut self, il: ItemLocation) {
        let effect = crate::game::item::throw::item_to_throw_effect(self.gd(), il, CharaId::Player);
        let target = if let Some(target) = auto_target_for_player(self.0, &effect) {
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
        let target = if let Some(ItemObjAttr::Use(UseEffect::Effect(effect))) =
            find_attr!(item_obj, ItemObjAttr::Use)
        {
            if let Some(target) = auto_target_for_player(self.0, effect) {
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
        let effect = if let Some(effect) =
            find_attr!(item_obj, ItemObjAttr::Release { effect, .. } => effect)
        {
            effect
        } else {
            error!("release item that doesn't have effect");
            return;
        };
        let target = if let Some(target) = auto_target_for_player(self.0, effect) {
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

    pub fn move_item<T: Into<ItemMoveNum>>(
        &mut self,
        il: ItemLocation,
        ill_in_container: ItemListLocation,
        n: T,
    ) {
        self.0.gd.move_item(il, ill_in_container, n);
    }

    pub fn append_item(&mut self, item: Item, n: u32) {
        self.0
            .gd
            .get_item_list_mut(ItemListLocation::PLAYER)
            .append(item, n);
    }

    pub fn remove_item<T: Into<ItemMoveNum>>(&mut self, il: ItemLocation, n: T) -> Item {
        self.0.gd.remove_item_and_get(il, n)
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

    /// Remove specified character's equipment
    pub fn remove_equipment(&mut self, cid: CharaId, slot: (EquipSlotKind, u8)) {
        super::item::remove_equipment(self.gd_mut(), cid, slot);
    }

    /// Install slot to specified item
    pub fn install_slot(&mut self, il: ItemLocation, slot_kind: ModuleSlotKind, cost: i64) {
        super::item::slot::install_slot(self.gd_mut(), il, slot_kind, cost);
    }

    /// Use active skill. Returns false if the skill cost is not enough.
    pub fn use_ability(&mut self, ability_id: &AbilityId) -> bool {
        if !super::action::ability::usable(self.gd(), CharaId::Player, ability_id, true) {
            return false;
        }
        let ability = if let Some(ability) = RULES.abilities.get(ability_id) {
            ability
        } else {
            return false;
        };

        let target = if let Some(target) = auto_target_for_player(self.0, &ability.effect) {
            target
        } else {
            let ability_id = ability_id.clone();
            self.0.ui_request.push_back(UiRequest::StartTargeting {
                effect: ability.effect.clone(),
                callback: Box::new(move |pa, target| {
                    super::action::ability::use_ability(pa.0, &ability_id, CharaId::Player, target);
                    pa.0.finish_player_turn();
                }),
            });
            return true;
        };
        super::action::ability::use_ability(self.0, ability_id, CharaId::Player, target);
        true
    }

    /// Try talk to next chara
    /// If success, returns id of the talk script
    pub fn try_talk(&mut self, dir: Direction) {
        if dir.as_coords() == (0, 0) {
            return;
        }

        let mut trigger_talk = None;
        let mut cid = None;
        {
            let gd = self.gd();
            let dest_tile =
                gd.get_current_map().chara_pos(CharaId::Player).unwrap() + dir.as_coords();
            if let Some(other_chara) = gd.get_current_map().get_chara(dest_tile) {
                cid = Some(other_chara);
                let relation = gd.chara_relation(CharaId::Player, other_chara);
                let other_chara = gd.chara.get(other_chara);
                match relation {
                    Relationship::Ally | Relationship::Friendly | Relationship::Neutral => {
                        if let Some(ref t) = other_chara.talk_script {
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
        let response = if let Some(choice) = choice {
            Value::Int(choice.into())
        } else {
            Value::None
        };
        self.0.advance_script(Some(response))
    }

    /// Shotcut to Game::advance_talk
    pub fn advance_script(&mut self) -> AdvanceScriptResult {
        self.0.advance_script(None)
    }

    /// Undertake quests
    pub fn undertake_quests(&mut self, targets: Vec<usize>) {
        crate::game::quest::undertake_quests(self.gd_mut(), targets);
    }

    /// Report quests
    pub fn report_quests(&mut self, targets: Vec<usize>) {
        crate::game::quest::report_quests(self.gd_mut(), targets);
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

    /// Print information of specified tile
    pub fn print_tile_info(&mut self, tile: Coords) {
        // Open StatusWindow for selected character
        let cid = self.gd().get_current_map().get_chara(tile);

        match cid {
            Some(cid) if cid != CharaId::Player => {
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
                    game_log!("not-scanned"; chara=chara);
                    return;
                }
            }
            _ => (),
        }

        super::map::tile_info::print_tile_info(self.0, tile);
    }

    pub fn select_build_obj(&mut self, il: ItemLocation, new_build_obj: BuildObj) {
        let item = &mut self.gd_mut().get_item_mut(il).0;

        if let Some(ItemAttr::BuildObj(ref mut build_obj)) =
            find_attr_mut!(item, ItemAttr::BuildObj)
        {
            *build_obj = new_build_obj;
        }
    }

    pub fn update_quest_status(&mut self) {
        super::quest::update_quest_status(self.gd_mut());
    }
}
