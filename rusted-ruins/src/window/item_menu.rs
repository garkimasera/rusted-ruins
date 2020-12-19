use super::choose_window::{ChooseWindow, DefaultBehavior};
use super::commonuse::*;
use crate::game::DialogOpenRequest;
use crate::text::ui_txt;
use common::gamedata::*;

use super::item_window::ItemWindowMode;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum ItemMenuItem {
    Infomation,
    DropAll,
}

pub struct ItemMenu {
    choose_window: ChooseWindow,
    menu_items: Vec<ItemMenuItem>,
    il: ItemLocation,
}

impl ItemMenu {
    pub fn new(mode: &ItemWindowMode, il: ItemLocation, pos: Option<(i32, i32)>) -> ItemMenu {
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

        let choose_window = ChooseWindow::new(winpos, choices, DefaultBehavior::Close);

        ItemMenu {
            choose_window,
            menu_items,
            il,
        }
    }
}

impl Window for ItemMenu {
    fn draw(&mut self, context: &mut Context, game: &Game, anim: Option<(&Animation, u32)>) {
        self.choose_window.draw(context, game, anim);
    }
}

impl DialogWindow for ItemMenu {
    fn process_command(&mut self, command: &Command, pa: &mut DoPlayerAction) -> DialogResult {
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
