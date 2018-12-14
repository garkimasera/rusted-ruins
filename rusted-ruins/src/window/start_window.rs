
use std::ffi::OsStr;
use std::path::PathBuf;
use common::gamedata::GameData;
use crate::config::{SCREEN_CFG, UI_CFG};
use super::commonuse::*;
use super::widget::*;
use super::SpecialDialogResult;
use super::choose_window::PagedChooseWindow;
use crate::text;

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
    answer_list: ListWidget,
}

impl StartDialog {
    pub fn new() -> StartDialog {
        let choices = vec![
            text::ui_txt("dialog.choice.newgame").to_owned(),
            text::ui_txt("dialog.choice.loadgame").to_owned(),
            text::ui_txt("dialog.choice.exit").to_owned()];
        let rect = UI_CFG.start_dialog.rect.into();
        StartDialog {
            rect: rect,
            answer_list: ListWidget::texts_choices((0, 0, rect.w as u32, 0), choices),
        }
    }
}

impl Window for StartDialog {
    fn draw(&mut self, context: &mut Context, _game: &Game, _anim: Option<(&Animation, u32)>) {

        draw_rect_border(context.canvas, self.rect);
        
        self.answer_list.draw(context);
    }
}

impl DialogWindow for StartDialog {
    fn process_command(&mut self, command: &Command, _pa: &mut DoPlayerAction) -> DialogResult {
        if let Some(response) = self.answer_list.process_command(&command) {
            match response {
                ListWidgetResponse::Select(0) => { // New Game
                    return DialogResult::Special(SpecialDialogResult::StartDialogNewGame);
                }
                ListWidgetResponse::Select(1) => { // Load Game
                    return DialogResult::Special(SpecialDialogResult::StartDialogLoadGame);
                }
                ListWidgetResponse::Select(2) => { // Exit
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
}

pub struct ChooseSaveFileDialog {
    choose_window: PagedChooseWindow,
    save_files: Vec<PathBuf>,
}

impl ChooseSaveFileDialog {
    pub fn new() -> ChooseSaveFileDialog {
        let save_files = crate::game::saveload::save_file_list().expect("Error at reading save file directory");
        
        let file_name_list: Vec<ListRow> = save_files
            .iter()
            .map(|path| path.file_stem().unwrap_or(OsStr::new("")).to_string_lossy().into_owned())
            .map(|filename| ListRow::Str(filename))
            .collect();
        
        ChooseSaveFileDialog {
            choose_window: PagedChooseWindow::new(
                UI_CFG.choose_save_file_dialog.rect.into(),
                file_name_list,
                UI_CFG.choose_save_file_dialog.list_size,
                None
            ),
            save_files,
        }
    }
}

impl Window for ChooseSaveFileDialog {
    fn draw(&mut self, context: &mut Context, game: &Game, anim: Option<(&Animation, u32)>) {
        self.choose_window.draw(context, game, anim);
    }
}

impl DialogWindow for ChooseSaveFileDialog {
    fn process_command(&mut self, command: &Command, pa: &mut DoPlayerAction) -> DialogResult {
        match self.choose_window.process_command(&command, pa) {
            DialogResult::Close => { return DialogResult::Close; }
            DialogResult::CloseWithValue(v) => {
                let i = *v.downcast::<u32>().unwrap() as usize;

                match GameData::load(&self.save_files[i]) {
                    Ok(o) => {
                        return DialogResult::Special(SpecialDialogResult::NewGameStart(o));
                    }
                    Err(e) => {
                        warn!("Failed to load a save file: {}", e);
                        return DialogResult::Continue;
                    }
                }
            }
            _ => (),
        }

        DialogResult::Continue
    }

    fn mode(&self) -> InputMode {
        InputMode::Dialog
    }
}

