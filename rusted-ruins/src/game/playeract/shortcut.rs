use super::DoPlayerAction;
use crate::game::InfoGetter;
use common::gamedata::*;

impl<'a> DoPlayerAction<'a> {
    pub fn register_shortcut(&mut self, shortcut: ActionShortcut, n: u32) {
        self.0.gd.settings.action_shortcuts[n as usize] = Some(shortcut);
    }

    pub fn exec_shortcut(&mut self, n: usize) {
        let shortcut = if let Some(shortcut) = self.gd().settings.action_shortcuts[n] {
            shortcut
        } else {
            return;
        };

        match shortcut {
            ActionShortcut::Throw(idx) => {
                if let Some(il) = self.gd().search_item(idx).get(0) {
                    self.throw_item(*il);
                }
            }
            ActionShortcut::Drink(idx) => {
                if let Some(il) = self.gd().search_item(idx).get(0) {
                    self.drink_item(*il);
                }
            }
            ActionShortcut::Eat(idx) => {
                if let Some(il) = self.gd().search_item(idx).get(0) {
                    self.eat_item(*il);
                }
            }
            ActionShortcut::Use(idx) => {
                if let Some(il) = self.gd().search_item(idx).get(0) {
                    self.use_item(*il);
                }
            }
            ActionShortcut::Release(idx) => {
                if let Some(il) = self.gd().search_item(idx).get(0) {
                    self.release_item(*il);
                }
            }
            ActionShortcut::Read(idx) => {
                if let Some(il) = self.gd().search_item(idx).get(0) {
                    self.read_item(*il);
                }
            }
        }
    }
}
