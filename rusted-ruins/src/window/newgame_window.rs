
use config::SCREEN_CFG;
use super::commonuse::*;
use super::widget::*;
use text;
use game::newgame::NewGameBuilder;
use super::textinputdialog::TextInputDialog;

pub const NEW_GAME_WINDOW_RESULT_START: u32 = 0;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum NewGameBuildStage {
    PlayerNameInput,
}

pub struct NewGameWindow {
    back_image: ImageWidget,
}

impl NewGameWindow {
    pub fn new() -> NewGameWindow {
        let rect = Rect::new(0, 0, SCREEN_CFG.screen_w, SCREEN_CFG.screen_h);
        
        NewGameWindow {
            back_image: ImageWidget::ui_img(rect, "!title-screen"),
        }
    }
    
    pub fn redraw(&mut self, canvas: &mut WindowCanvas, _game: &Game, sv: &mut SdlValues,
                  _anim: Option<(&Animation, u32)>) {

        self.back_image.draw(canvas, sv);
    }
}

pub struct DummyNewGameDialog {
    builder: NewGameBuilder,
    stage: NewGameBuildStage,
    name_input_dialog: Option<TextInputDialog>,
}

impl DummyNewGameDialog {
    pub fn new() -> DummyNewGameDialog {
        DummyNewGameDialog {
            builder: NewGameBuilder::new(),
            stage: NewGameBuildStage::PlayerNameInput,
            name_input_dialog: Some(TextInputDialog::new()),
        }
    }
}

impl Window for DummyNewGameDialog {
    fn redraw(&mut self, canvas: &mut WindowCanvas, game: &Game, sv: &mut SdlValues,
              anim: Option<(&Animation, u32)>) {

        match self.stage {
            NewGameBuildStage::PlayerNameInput => {
                self.name_input_dialog.as_mut().unwrap().redraw(canvas, game, sv, anim);
            }
        }
    }
}

impl DialogWindow for DummyNewGameDialog {
    fn process_command(&mut self, command: &Command, pa: &mut DoPlayerAction) -> DialogResult {
        match self.stage {
            NewGameBuildStage::PlayerNameInput => {
                let name_input_dialog = self.name_input_dialog.as_mut().unwrap();
                match name_input_dialog.process_command(command, pa) {
                    DialogResult::Close => {
                        let player_name = name_input_dialog.get_text();
                        if player_name != "" { // If input text is invalid for character name
                            self.builder.set_player_name(player_name);
                            return DialogResult::User(NEW_GAME_WINDOW_RESULT_START);
                        }
                        name_input_dialog.restart();
                    }
                    _ => (),
                }
                return DialogResult::Continue;
            }
        }
    }

    fn mode(&self) -> InputMode {
        match self.stage {
            NewGameBuildStage::PlayerNameInput => InputMode::TextInput,
            _ => InputMode::Dialog,
        }
    }
}

