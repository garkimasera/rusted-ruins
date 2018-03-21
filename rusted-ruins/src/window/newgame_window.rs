
use config::SCREEN_CFG;
use super::commonuse::*;
use super::widget::*;
use text;
use common::gamedata::chara::CharaClass;
use game::newgame::NewGameBuilder;
use super::text_window::TextWindow;
use super::text_input_dialog::TextInputDialog;
use super::choose_window::PagedChooseWindow;
use super::widget::ListRow;
use super::SpecialDialogResult;
use rules::RULES;

/// Newgame processes with next order
/// PlayerNameinput -> ChooseClass
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum NewGameBuildStage {
    PlayerNameInput,
    ChooseClass,
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
    
    pub fn draw(&mut self, canvas: &mut WindowCanvas, _game: &Game, sv: &mut SdlValues,
                  _anim: Option<(&Animation, u32)>) {

        self.back_image.draw(canvas, sv);
    }
}

pub struct DummyNewGameDialog {
    builder: Option<NewGameBuilder>,
    stage: NewGameBuildStage,
    explanation_text: TextWindow,
    name_input_dialog: Option<TextInputDialog>,
    choose_class_dialog: ChooseClassDialog,
}

impl DummyNewGameDialog {
    pub fn new() -> DummyNewGameDialog {
        DummyNewGameDialog {
            builder: Some(NewGameBuilder::new()),
            explanation_text: explanation_text_window("newgame.inputplayername"),
            stage: NewGameBuildStage::PlayerNameInput,
            name_input_dialog: Some(TextInputDialog::new()),
            choose_class_dialog: ChooseClassDialog::new(),
        }
    }
}

impl Window for DummyNewGameDialog {
    fn draw(&mut self, canvas: &mut WindowCanvas, game: &Game, sv: &mut SdlValues,
              anim: Option<(&Animation, u32)>) {

        self.explanation_text.draw(canvas, game, sv, anim);
        
        match self.stage {
            NewGameBuildStage::PlayerNameInput => {
                self.name_input_dialog.as_mut().unwrap().draw(canvas, game, sv, anim);
            }
            NewGameBuildStage::ChooseClass => {
                self.choose_class_dialog.draw(canvas, game, sv, anim);
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
                            self.builder.as_mut().unwrap().set_player_name(player_name);
                            self.explanation_text = explanation_text_window("newgame.chooseclass");
                            self.stage = NewGameBuildStage::ChooseClass;
                        }
                        name_input_dialog.restart();
                    }
                    _ => (),
                }
                return DialogResult::Continue;
            }
            NewGameBuildStage::ChooseClass => {
                match self.choose_class_dialog.process_command(command, pa) {
                    DialogResult::CloseWithValue(chara_class) => {
                        let chara_class = chara_class.downcast::<CharaClass>().unwrap();
                        self.builder.as_mut().unwrap().set_chara_class(*chara_class);
                        let builder = self.builder.take().unwrap();
                        let gd = builder.build();
                        return DialogResult::Special(SpecialDialogResult::NewGameStart(gd));
                    }
                    _ => DialogResult::Continue
                }
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

pub struct ChooseClassDialog {
    choose_window: PagedChooseWindow,
}

impl ChooseClassDialog {
    pub fn new() -> ChooseClassDialog {
        let rect: Rect = UI_CFG.choose_class_dialog.rect.into();
        let choices: Vec<ListRow> = RULES
            .newgame
            .class_choices
            .iter()
            .map(|c| ListRow::Str(format!("{:?}", c)))
            .collect();
        let choose_window = PagedChooseWindow::new(
            rect, choices, 7, None);
        
        ChooseClassDialog {
            choose_window
        }
    }
}

impl Window for ChooseClassDialog {
    
    fn draw(
        &mut self, canvas: &mut WindowCanvas, game: &Game, sv: &mut SdlValues,
        anim: Option<(&Animation, u32)>) {
       
        self.choose_window.draw(canvas, game, sv, anim);
    }
}

impl DialogWindow for ChooseClassDialog {
    fn process_command(&mut self, command: &Command, pa: &mut DoPlayerAction) -> DialogResult {
        match self.choose_window.process_command(&command, pa) {
            DialogResult::CloseWithValue(_) => {
                let chara_class =
                    RULES.newgame.class_choices[self.choose_window.get_current_choice() as usize];
                DialogResult::CloseWithValue(Box::new(chara_class))
            }
            _ => DialogResult::Continue
        }
    }

    fn mode(&self) -> InputMode {
        InputMode::Dialog
    }
}

fn explanation_text_window(s: &str) -> TextWindow {
    TextWindow::new(
        UI_CFG.newgame_dialog.explanation_text_rect.into(),
        text::ui_txt(s))
}
