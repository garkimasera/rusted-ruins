use super::choose_window::ChooseWindow;
use super::commonuse::*;
use super::text_window::TextWindow;
use super::widget::ButtonWidget;
use super::winpos::{WindowHPos, WindowPos, WindowVPos};
use crate::config::UI_CFG;
use crate::text::ui_txt;
use sdl2::rect::Rect;

pub struct ExitWindow {
    rect: Rect,
    closer: DialogCloser,
    save_button: ButtonWidget,
    title_screen_button: ButtonWidget,
    exit_button: ButtonWidget,
    cancel_button: ButtonWidget,
}

impl ExitWindow {
    pub fn new() -> ExitWindow {
        let cfg = &UI_CFG.exit_window;
        let rect: Rect = cfg.rect.into();

        let save_button = ButtonWidget::new(
            cfg.save_button_rect,
            ui_txt("dialog-button-save"),
            FontKind::M,
        );
        let title_screen_button = ButtonWidget::new(
            cfg.title_screen_button_rect,
            ui_txt("dialog-button-title_screen"),
            FontKind::M,
        );
        let exit_button = ButtonWidget::new(
            cfg.exit_button_rect,
            ui_txt("dialog-button-exit"),
            FontKind::M,
        );
        let cancel_button = ButtonWidget::new(
            cfg.cancel_button_rect,
            ui_txt("dialog-button-cancel"),
            FontKind::M,
        );

        ExitWindow {
            rect,
            closer: DialogCloser::default(),
            save_button,
            title_screen_button,
            exit_button,
            cancel_button,
        }
    }
}

impl Window for ExitWindow {
    fn draw(
        &mut self,
        context: &mut Context<'_, '_, '_, '_>,
        _game: &Game,
        _anim: Option<(&Animation, u32)>,
    ) {
        draw_window_border(context, self.rect);
        self.save_button.draw(context);
        self.title_screen_button.draw(context);
        self.exit_button.draw(context);
        self.cancel_button.draw(context);
    }
}

impl DialogWindow for ExitWindow {
    fn process_command(&mut self, command: &Command, pa: &mut DoPlayerAction<'_>) -> DialogResult {
        closer!(self, command);
        let command = command.relative_to(self.rect);

        if command == Command::Cancel {
            return DialogResult::Close;
        }

        if self.save_button.process_command(&command).is_some() {
            pa.game().save_file();
            DialogResult::Close
        } else if self.title_screen_button.process_command(&command).is_some() {
            DialogResult::Special(SpecialDialogResult::ReturnToStartScreen)
        } else if self.exit_button.process_command(&command).is_some() {
            DialogResult::Quit
        } else if self.cancel_button.process_command(&command).is_some() {
            DialogResult::Close
        } else {
            DialogResult::Continue
        }
    }
}

/// Ask to return start screen or quit
pub struct GameOverWindow {
    text_win: TextWindow,
    choose_win: ChooseWindow,
}

impl GameOverWindow {
    pub fn new() -> GameOverWindow {
        let rect: Rect = UI_CFG.exit_window.rect.into();
        let text_win = TextWindow::new(rect, &ui_txt("dialog-gameover"));
        let winpos = WindowPos::new(
            WindowHPos::RightX(rect.right()),
            WindowVPos::TopMargin(rect.bottom() + UI_CFG.gap_len_between_dialogs),
        );
        let choices = vec![
            ui_txt("dialog-choice-restart"),
            ui_txt("dialog-choice-title_screen"),
            ui_txt("dialog-choice-exit_game"),
        ];
        GameOverWindow {
            text_win,
            choose_win: ChooseWindow::new(
                winpos,
                choices,
                super::choose_window::DefaultBehavior::Ignore,
            ),
        }
    }
}

impl Window for GameOverWindow {
    fn draw(
        &mut self,
        context: &mut Context<'_, '_, '_, '_>,
        game: &Game,
        anim: Option<(&Animation, u32)>,
    ) {
        self.text_win.draw(context, game, anim);
        let rect = self.text_win.get_rect();
        let winpos = WindowPos::new(
            WindowHPos::RightX(rect.right()),
            WindowVPos::TopMargin(rect.bottom() + UI_CFG.gap_len_between_dialogs),
        );
        self.choose_win.set_winpos(winpos);
        self.choose_win.draw(context, game, anim);
    }
}

impl DialogWindow for GameOverWindow {
    fn process_command(&mut self, command: &Command, pa: &mut DoPlayerAction<'_>) -> DialogResult {
        if *command == Command::Cancel {
            return DialogResult::Continue;
        }

        use super::SpecialDialogResult::ReturnToStartScreen;
        if let DialogResult::CloseWithValue(DialogCloseValue::Index(n)) =
            self.choose_win.process_command(command, pa)
        {
            // An choice is choosed
            match n {
                0 => {
                    pa.restart();
                    return DialogResult::Close;
                }
                1 => return DialogResult::Special(ReturnToStartScreen),
                2 => return DialogResult::Quit,
                _ => panic!(),
            }
        }
        DialogResult::Continue
    }
}
