use super::commonuse::*;
use super::widget::*;
use super::SpecialDialogResult;
use crate::config::{SCREEN_CFG, UI_CFG};
use crate::text;
use common::gamedata::GameData;
use std::ffi::OsStr;
use std::path::PathBuf;

pub struct StartWindow {
    title_screen: ImageWidget,
}

impl StartWindow {
    pub fn new() -> StartWindow {
        let rect = Rect::new(0, 0, SCREEN_CFG.screen_w, SCREEN_CFG.screen_h);

        StartWindow {
            title_screen: ImageWidget::ui_img(rect, "!title-screen"),
        }
    }

    pub fn draw(&mut self, context: &mut Context, _game: &Game, _anim: Option<(&Animation, u32)>) {
        self.title_screen.draw(context);
    }
}

pub struct StartDialog {
    rect: Rect,
    answer_list: TextListWidget,
}

impl StartDialog {
    pub fn new() -> StartDialog {
        let choices = vec![
            text::ui_txt("dialog-choice-newgame"),
            text::ui_txt("dialog-choice-loadgame"),
            text::ui_txt("dialog-choice-exit"),
        ];
        let rect = UI_CFG.start_dialog.rect.into();
        StartDialog {
            rect,
            answer_list: ListWidget::text_choices((0, 0, rect.w as u32, 0), choices),
        }
    }
}

impl Window for StartDialog {
    fn draw(&mut self, context: &mut Context, _game: &Game, _anim: Option<(&Animation, u32)>) {
        draw_window_border(context, self.rect);
        self.answer_list.draw(context);
    }
}

impl DialogWindow for StartDialog {
    fn process_command(&mut self, command: &Command, _pa: &mut DoPlayerAction) -> DialogResult {
        let command = command.relative_to(self.rect);
        if let Some(response) = self.answer_list.process_command(&command) {
            match response {
                ListWidgetResponse::Select(0) => {
                    // New Game
                    return DialogResult::Special(SpecialDialogResult::StartDialogNewGame);
                }
                ListWidgetResponse::Select(1) => {
                    // Load Game
                    return DialogResult::Special(SpecialDialogResult::StartDialogLoadGame);
                }
                ListWidgetResponse::Select(2) => {
                    // Exit
                    return DialogResult::Quit;
                }
                _ => (),
            }
            return DialogResult::Continue;
        }

        DialogResult::Continue
    }

    fn mode(&self) -> InputMode {
        InputMode::Dialog
    }

    fn sound(&self, _: bool) {}
}

pub struct ChooseSaveFileDialog {
    rect: Rect,
    list: TextListWidget,
    save_files: Vec<PathBuf>,
}

impl ChooseSaveFileDialog {
    pub fn new() -> ChooseSaveFileDialog {
        let save_files =
            crate::game::saveload::save_file_list().expect("Error at reading save file directory");

        let file_name_list: Vec<String> = save_files
            .iter()
            .map(|path| {
                path.file_stem()
                    .unwrap_or_else(|| OsStr::new(""))
                    .to_string_lossy()
                    .into_owned()
            })
            .collect();
        let rect = UI_CFG.choose_save_file_dialog.rect.into();

        ChooseSaveFileDialog {
            rect,
            list: TextListWidget::text_choices((0, 0, rect.width(), rect.height()), file_name_list),
            save_files,
        }
    }
}

impl Window for ChooseSaveFileDialog {
    fn draw(&mut self, context: &mut Context, _game: &Game, _anim: Option<(&Animation, u32)>) {
        draw_window_border(context, self.rect);
        self.list.draw(context);
    }
}

impl DialogWindow for ChooseSaveFileDialog {
    fn process_command(&mut self, command: &Command, _pa: &mut DoPlayerAction) -> DialogResult {
        let command = command.relative_to(self.rect);
        if let Some(response) = self.list.process_command(&command) {
            if let ListWidgetResponse::Select(i) = response {
                // Any item is selected
                match GameData::load(&self.save_files[i as usize]) {
                    Ok(gd) => {
                        return DialogResult::Special(SpecialDialogResult::NewGameStart(Box::new(
                            gd,
                        )));
                    }
                    Err(e) => {
                        warn!("Failed to load a save file: {}", e);
                        return DialogResult::Continue;
                    }
                }
            }
            return DialogResult::Continue;
        }
        match command {
            Command::Cancel => DialogResult::Close,
            _ => DialogResult::Continue,
        }
    }

    fn mode(&self) -> InputMode {
        InputMode::Dialog
    }

    fn sound(&self, _: bool) {}
}
