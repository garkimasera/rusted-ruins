use super::commonuse::*;
use super::text_input_dialog::TextInputDialog;
use super::text_window::{ScrollingTextWindow, TextWindow};
use super::widget::*;
use super::SpecialDialogResult;
use crate::config::SCREEN_CFG;
use crate::game::newgame::NewGameBuilder;
use crate::text;
use rules::RULES;

/// Newgame processes with next order
/// PlayerNameinput -> ChooseClass
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum NewGameBuildStage {
    PlayerNameInput,
    ChooseClass,
    OpeningText,
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

    pub fn draw(
        &mut self,
        context: &mut Context<'_, '_, '_, '_>,
        _game: &Game<'_>,
        _anim: Option<(&Animation, u32)>,
    ) {
        self.back_image.draw(context);
    }
}

pub struct DummyNewGameDialog {
    builder: Option<NewGameBuilder>,
    stage: NewGameBuildStage,
    explanation_text: TextWindow,
    name_input_dialog: Option<TextInputDialog>,
    choose_class_dialog: ChooseClassDialog,
    opening_text: ScrollingTextWindow,
}

impl DummyNewGameDialog {
    pub fn new() -> DummyNewGameDialog {
        DummyNewGameDialog {
            builder: Some(NewGameBuilder::new()),
            explanation_text: explanation_text_window("newgame-inputplayername"),
            stage: NewGameBuildStage::PlayerNameInput,
            name_input_dialog: Some(TextInputDialog::new()),
            choose_class_dialog: ChooseClassDialog::new(),
            opening_text: opening_text_window(),
        }
    }
}

impl Window for DummyNewGameDialog {
    fn draw(
        &mut self,
        context: &mut Context<'_, '_, '_, '_>,
        game: &Game<'_>,
        anim: Option<(&Animation, u32)>,
    ) {
        match self.stage {
            NewGameBuildStage::PlayerNameInput => {
                self.explanation_text.draw(context, game, anim);
                self.name_input_dialog
                    .as_mut()
                    .unwrap()
                    .draw(context, game, anim);
            }
            NewGameBuildStage::ChooseClass => {
                self.explanation_text.draw(context, game, anim);
                self.choose_class_dialog.draw(context, game, anim);
            }
            NewGameBuildStage::OpeningText => {
                self.opening_text.draw(context, game, anim);
            }
        }
    }
}

impl DialogWindow for DummyNewGameDialog {
    fn process_command(
        &mut self,
        command: &Command,
        pa: &mut DoPlayerAction<'_, '_>,
    ) -> DialogResult {
        match self.stage {
            NewGameBuildStage::PlayerNameInput => {
                let name_input_dialog = self.name_input_dialog.as_mut().unwrap();
                if let DialogResult::Close = name_input_dialog.process_command(command, pa) {
                    let player_name = name_input_dialog.get_text();
                    if !player_name.is_empty() {
                        // If input text is invalid for character name
                        self.builder.as_mut().unwrap().set_player_name(player_name);
                        self.explanation_text = explanation_text_window("newgame-chooseclass");
                        self.stage = NewGameBuildStage::ChooseClass;
                    }
                    name_input_dialog.restart();
                }
                DialogResult::Continue
            }
            NewGameBuildStage::ChooseClass => {
                if let DialogResult::CloseWithValue(DialogCloseValue::CharaClass(chara_class)) =
                    self.choose_class_dialog.process_command(command, pa)
                {
                    self.builder.as_mut().unwrap().set_chara_class(chara_class);
                    self.stage = NewGameBuildStage::OpeningText;
                    {
                        // Skip OP text
                        let builder = self.builder.take().unwrap();
                        let gd = builder.build();
                        return DialogResult::Special(SpecialDialogResult::NewGameStart(Box::new(
                            gd,
                        )));
                    }
                }
                DialogResult::Continue
            }
            NewGameBuildStage::OpeningText => {
                match command {
                    Command::Enter => {
                        if !self.opening_text.is_finished() {
                            return DialogResult::Continue;
                        }
                    }
                    Command::Cancel => (),
                    _ => {
                        return DialogResult::Continue;
                    }
                }
                let builder = self.builder.take().unwrap();
                let gd = builder.build();
                DialogResult::Special(SpecialDialogResult::NewGameStart(Box::new(gd)))
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
    rect: Rect,
    list: TextListWidget,
}

impl ChooseClassDialog {
    pub fn new() -> ChooseClassDialog {
        let rect: Rect = UI_CFG.choose_class_dialog.rect.into();
        let choices: Vec<String> = RULES
            .newgame
            .class_choices
            .iter()
            .map(|c| text::misc_txt(c.as_str()))
            .collect();

        ChooseClassDialog {
            rect,
            list: TextListWidget::text_choices((0, 0, rect.width(), rect.height()), choices),
        }
    }
}

impl Window for ChooseClassDialog {
    fn draw(
        &mut self,
        context: &mut Context<'_, '_, '_, '_>,
        _game: &Game<'_>,
        _anim: Option<(&Animation, u32)>,
    ) {
        draw_window_border(context, self.rect);
        self.list.draw(context);
    }
}

impl DialogWindow for ChooseClassDialog {
    fn process_command(
        &mut self,
        command: &Command,
        _pa: &mut DoPlayerAction<'_, '_>,
    ) -> DialogResult {
        let command = command.relative_to(self.rect);
        if let Some(response) = self.list.process_command(&command) {
            if let ListWidgetResponse::Select(i) = response {
                // Any item is selected
                let chara_class = RULES.newgame.class_choices[i as usize];
                return DialogResult::CloseWithValue(DialogCloseValue::CharaClass(chara_class));
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
}

fn explanation_text_window(s: &str) -> TextWindow {
    TextWindow::new(
        UI_CFG.newgame_dialog.explanation_text_rect.into(),
        &text::ui_txt(s),
    )
}

/// Create scrolling text window that displays opening text
fn opening_text_window() -> ScrollingTextWindow {
    ScrollingTextWindow::new(SCREEN_CFG.main_window.into(), &text::misc_txt("!op-scroll"))
}
