use super::choose_window::{ChooseWindow, DefaultBehavior};
use super::commonuse::*;
use crate::game::DialogOpenRequest;
use crate::text::ui_txt;
use common::gamedata::*;

use super::item_window::ItemWindowMode;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum ItemMenuItem {
    Infomation,
    RegisterAsShortcut(ActionShortcut),
    DropAll,
}

pub struct ItemMenu {
    choose_window: ChooseWindow,
    menu_items: Vec<ItemMenuItem>,
    il: ItemLocation,
}

impl ItemMenu {
    pub fn new(
        gd: &GameData,
        mode: &ItemWindowMode,
        il: ItemLocation,
        pos: Option<(i32, i32)>,
    ) -> ItemMenu {
        let winpos = if let Some((x, y)) = pos {
            WindowPos::from_left_top(x, y)
        } else {
            WindowPos::CENTER
        };

        let mut choices = Vec::new();
        let mut menu_items = Vec::new();

        // Item infomation.
        choices.push(ui_txt("item_menu-infomation"));
        menu_items.push(ItemMenuItem::Infomation);

        // Drop
        if mode.is_main_mode() {
            choices.push(ui_txt("item_menu-drop_all"));
            menu_items.push(ItemMenuItem::DropAll);
        }

        // Register as shortcut
        let item_idx = gd.get_item(il).0.idx;
        let shortcut = match mode {
            ItemWindowMode::Throw => Some(ActionShortcut::Throw(item_idx)),
            ItemWindowMode::Drink => Some(ActionShortcut::Drink(item_idx)),
            ItemWindowMode::Eat => Some(ActionShortcut::Eat(item_idx)),
            ItemWindowMode::Use => Some(ActionShortcut::Use(item_idx)),
            ItemWindowMode::Release => Some(ActionShortcut::Release(item_idx)),
            ItemWindowMode::Read => Some(ActionShortcut::Read(item_idx)),
            _ => None,
        };
        if let Some(shortcut) = shortcut {
            choices.push(ui_txt("item_menu-register-as-shortcut"));
            menu_items.push(ItemMenuItem::RegisterAsShortcut(shortcut));
        }

        let choose_window = ChooseWindow::new(winpos, choices, DefaultBehavior::Close);

        ItemMenu {
            choose_window,
            menu_items,
            il,
        }
    }
}

impl Window for ItemMenu {
    fn draw(
        &mut self,
        context: &mut Context<'_, '_, '_, '_>,
        game: &Game<'_>,
        anim: Option<(&Animation, u32)>,
    ) {
        self.choose_window.draw(context, game, anim);
    }
}

impl DialogWindow for ItemMenu {
    fn process_command(
        &mut self,
        command: &Command,
        pa: &mut DoPlayerAction<'_, '_>,
    ) -> DialogResult {
        match self.choose_window.process_command(command, pa) {
            DialogResult::CloseWithValue(v) => {
                if let DialogCloseValue::Index(n) = v {
                    let item = self.menu_items[n as usize];
                    let il = self.il;
                    match item {
                        ItemMenuItem::Infomation => {
                            pa.request_dialog_open(DialogOpenRequest::ItemInfo { il });
                            DialogResult::Close
                        }
                        ItemMenuItem::DropAll => {
                            let n = pa.gd().get_item(il).1;
                            pa.drop_item(il, n);
                            DialogResult::Special(SpecialDialogResult::ItemListUpdate)
                        }
                        ItemMenuItem::RegisterAsShortcut(shortcut) => {
                            pa.request_dialog_open(DialogOpenRequest::RegisterAsShortcut {
                                shortcut,
                            });
                            DialogResult::Close
                        }
                    }
                } else {
                    unreachable!()
                }
            }
            result => result,
        }
    }

    fn mode(&self) -> InputMode {
        InputMode::Dialog
    }
}
