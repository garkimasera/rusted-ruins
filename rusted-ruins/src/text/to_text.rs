use crate::game::Command;
use crate::text::{self, ability_txt, misc_txt, obj_txt, ui_txt, ToText, ToTextId};
use common::gamedata::*;
use common::gobj;
use common::objholder::*;
use std::borrow::Cow;

use super::{obj_txt_checked, quest_txt};

impl<T: ToTextId> ToText for T {
    fn to_text(&self) -> Cow<'_, str> {
        text::to_txt(self).into()
    }
}

impl ToText for AbilityId {
    fn to_text(&self) -> Cow<'_, str> {
        ability_txt(&self.0).into()
    }
}

impl ToText for FactionId {
    fn to_text(&self) -> Cow<'_, str> {
        let s = self.as_str();
        let s = if let Some(s) = s.strip_prefix('!') {
            s
        } else {
            s
        };

        misc_txt(&format!("faction-{}", s)).into()
    }
}

impl ToText for Site {
    fn to_text(&self) -> Cow<'_, str> {
        if let Some(name) = self.name.as_ref() {
            return name.into();
        }

        if let Some(id) = self.id.as_ref() {
            if let Some(text) = obj_txt_checked(id) {
                return text.into();
            }
        }

        match self.content {
            SiteContent::AutoGenDungeon { dungeon_kind } => {
                misc_txt(&format!("dungeon_kind-{}", dungeon_kind.as_str())).into()
            }
            SiteContent::Town { ref town } => text::obj_txt(town.id()).into(),
            SiteContent::Player { .. } => "home".into(),
            SiteContent::Temp { .. } => "temp".into(),
            SiteContent::Other => {
                warn!("Unnamed other kind site");
                "".into()
            }
        }
    }
}

impl ToText for Region {
    fn to_text(&self) -> Cow<'_, str> {
        obj_txt(&self.name).into()
    }
}

impl ToText for Item {
    fn to_text(&self) -> Cow<'_, str> {
        use crate::game::item::ItemExt;
        let mut text: String = obj_txt(gobj::idx_to_id(self.idx));

        if let Some(n) = self.charge() {
            text.push_str(&format!(" ({} : {})", ui_txt("item-charges"), n));
        }

        if let Some(title) = self.title() {
            if let Some(title) = super::readable::readable_title_txt(title) {
                text.push_str(&format!(" <{}>", title));
            }
        }

        if let Some((material_name, _)) = self.material() {
            text.push_str(&format!(" ({})", super::prefix::material(material_name)))
        }

        if let Some(remaining) = self.remaining() {
            let days = remaining.days();
            let hours = remaining.hours();
            let s = if days > 0 {
                format!("{} {}", days, misc_txt("duration-days"))
            } else if hours > 0 {
                format!("{} {}", hours, misc_txt("duration-hours"))
            } else {
                let minutes = (remaining.minutes() / 10 + 1) * 10;
                format!("{} {}", minutes, misc_txt("duration-minutes"))
            };

            text.push_str(&misc_txt_format!("item_info_text-remaining"; duration=s));
        }

        if let Some(ItemAttr::BuildObj(build_obj)) = find_attr!(self, ItemAttr::BuildObj) {
            let obj_name = match build_obj {
                BuildObj::Tile(id) => obj_txt(id),
                BuildObj::Wall(id) => obj_txt(id),
            };
            text.push_str(&format!(" ({})", obj_name));
        }

        let quality = self.quality.as_int();

        if quality > 0 {
            text.push_str(&format!(" +{}", quality));
        } else if quality < 0 {
            text.push_str(&format!(" -{}", -quality));
        }

        for attr in &self.attrs {
            if let ItemAttr::SkillLearning(kind) = attr {
                text.push_str(&format!(" <{}>", kind.to_text()));
            }
        }

        text.into()
    }
}

impl ToText for CharaTemplateIdx {
    fn to_text(&self) -> Cow<'_, str> {
        obj_txt(gobj::idx_to_id(*self)).into()
    }
}

impl ToText for Chara {
    fn to_text(&self) -> Cow<'_, str> {
        use crate::game::chara::CharaExt;
        if self.is_main_character() {
            return misc_txt("you").into();
        }
        if let Some(ref name) = self.name {
            name.into()
        } else {
            obj_txt(gobj::idx_to_id(self.idx)).into()
        }
    }
}

impl ToText for CharaTrait {
    fn to_text(&self) -> Cow<'_, str> {
        misc_txt(&format!("trait-{}", self.text_id())).into()
    }
}

impl ToText for CharaModifier {
    fn to_text(&self) -> Cow<'_, str> {
        match self {
            CharaModifier::Str(value) => format!("STR {:+}", value),
            CharaModifier::Vit(value) => format!("VIT {:+}", value),
            CharaModifier::Dex(value) => format!("DEX {:+}", value),
            CharaModifier::Int(value) => format!("INT {:+}", value),
            CharaModifier::Wil(value) => format!("WIL {:+}", value),
            CharaModifier::Cha(value) => format!("CHA {:+}", value),
            CharaModifier::Spd(value) => format!("SPD {:+}", value),
            CharaModifier::Defence { element, value } => {
                format!("Defence {} {:+}", element.to_text(), value)
            }
            CharaModifier::DefenceMultiplier { element, value } => {
                format!("Defence {} {:+.0}%", element.to_text(), value * 100.0)
            }
        }
        .into()
    }
}

impl ToText for ModuleEffect {
    fn to_text(&self) -> Cow<'_, str> {
        match self {
            ModuleEffect::Ability { group } => misc_txt(&format!("ability_group-{}", group)).into(),
            ModuleEffect::Extend(effect) => match effect {
                ExtendModuleEffect::Chara(chara_modifier) => chara_modifier.to_text(),
                ExtendModuleEffect::Weapon(_weapon_modifier) => todo!(),
            },
            ModuleEffect::Core => "Core".into(),
        }
    }
}

#[extend::ext(pub, name = CharaTraitTextId)]
impl CharaTrait {
    fn text_id(&self) -> &str {
        match self {
            CharaTrait::Player => "player",
            CharaTrait::Id(id) => id,
        }
    }
}

impl ToText for Command {
    fn to_text(&self) -> Cow<'_, str> {
        use Command::*;
        let id = match self {
            Move { .. } => "command-move",
            MoveTo { .. } => "command-move_to",
            Shoot { .. } => "command-shoot",
            UseTool { .. } => "command-use-tool",
            Enter => "command-enter",
            Cancel => "command-cancel",
            RotateWindowRight => "command-rotate_window_right",
            RotateWindowLeft => "command-rotate_window_left",
            ItemInformation => "command-item_information",
            OpenAbilityWin => "command-open_ability_win",
            OpenCreationWin => "command-open_creation_win",
            OpenDebugCommandWin => "command-open_debug_command_win",
            OpenEquipWin => "command-open_equip_win",
            OpenExitWin => "command-open_exit_win",
            OpenGameInfoWin => "command-open_game_info_win",
            OpenHelpWin => "command-open_help_win",
            OpenStatusWin => "command-open_status_win",
            OpenItemWin => "command-open_item_menu",
            PickUpItem => "command-pick_up_item",
            DropItem => "command-drop_item",
            DrinkItem => "command-drink_item",
            EatItem => "command-eat_item",
            ReleaseItem => "command-release_item",
            ActionShortcut(_) => "command-action_shortcut",
            ChangeEquip { .. } => "command-change_equip",
            TextInput { .. } => "command-text_input",
            TextDelete => "command-text_delete",
            MouseButtonDown { .. } => "command-mouse_button_down",
            MouseButtonUp { .. } => "command-mouse_button_up",
            MouseWheel { .. } => "command-mouse_wheel",
            MouseState { .. } => "command-mouse_state",
        };
        ui_txt(id).into()
    }
}

impl ToText for CustomQuest {
    fn to_text(&self) -> Cow<'_, str> {
        quest_txt(&self.id).into()
    }
}

impl ToText for TownQuest {
    fn to_text(&self) -> Cow<'_, str> {
        quest_txt(&self.text_id).into()
    }
}

/// Implement ToText for primitive types
macro_rules! impl_to_text {
    ( $($t:ty),* ) => {
        $(
            impl ToText for $t {
                fn to_text(&self) -> Cow<'_, str> {
                    self.to_string().into()
                }
            }
        )*
    }
}

impl_to_text!(i8, u8, i16, u16, i32, u32, i64, u64, f32, f64, String);

impl<'a> ToText for &'a str {
    fn to_text(&self) -> Cow<'static, str> {
        self.to_string().into()
    }
}

impl<'a> ToText for Cow<'a, str> {
    fn to_text(&self) -> Cow<'static, str> {
        self.to_string().into()
    }
}
