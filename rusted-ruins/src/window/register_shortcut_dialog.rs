use super::choose_window::{ChooseWindow, DefaultBehavior};
use super::commonuse::*;
use crate::config::UI_CFG;
use common::gamedata::ActionShortcut;

pub struct RegisterShortcutDialog {
    shortcut: ActionShortcut,
    choose_win: ChooseWindow,
}

impl RegisterShortcutDialog {
    pub fn new(shortcut: ActionShortcut) -> Self {
        let n_shortcut = UI_CFG.toolbar.n_shortcut;
        let choices = (0..n_shortcut)
            .map(|i| {
                let i = if i != 9 { i + 1 } else { 0 };
                ui_txt_format!("register_shortcut"; i=i)
            })
            .collect();
        let choose_win = ChooseWindow::new(WindowPos::CENTER, choices, DefaultBehavior::Close);
        Self {
            shortcut,
            choose_win,
        }
    }
}

impl Window for RegisterShortcutDialog {
    fn draw(
        &mut self,
        context: &mut Context<'_, '_, '_, '_>,
        game: &Game,
        anim: Option<(&Animation, u32)>,
    ) {
        self.choose_win.draw(context, game, anim);
    }
}

impl DialogWindow for RegisterShortcutDialog {
    fn process_command(&mut self, command: &Command, pa: &mut DoPlayerAction<'_>) -> DialogResult {
        if *command == Command::Cancel {
            return DialogResult::Close;
        }

        match self.choose_win.process_command(command, pa) {
            DialogResult::CloseWithValue(v) => {
                if let DialogCloseValue::Index(choosed_answer) = v {
                    pa.register_shortcut(self.shortcut, choosed_answer);
                }
                return DialogResult::Close;
            }
            DialogResult::Close => {
                return DialogResult::Close;
            }
            _ => (),
        }
        DialogResult::Continue
    }
}
