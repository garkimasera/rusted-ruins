use super::choose_window::ChooseWindow;
use super::commonuse::*;
use super::misc_window::ImageWindow;
use super::widget::*;
use super::winpos::*;
use crate::config::UI_CFG;
use crate::context::textrenderer::FontKind;
use crate::game::{AdvanceScriptResult, TalkText};
use crate::text;
use common::basic::TILE_SIZE;
use common::objholder::CharaTemplateIdx;
use std::any::Any;
use std::borrow::Cow;

pub struct TalkWindow {
    rect: Rect,
    talk_text: TalkText,
    label: LabelWidget,
    image_window: Option<ImageWindow>,
    msg_text: MsgText,
    choose_win: Option<ChooseWindow>,
}

impl TalkWindow {
    pub fn new(talk_text: TalkText, chara_template_idx: Option<CharaTemplateIdx>) -> TalkWindow {
        let c = &UI_CFG.talk_window;
        let rect: Rect = c.rect.into();
        let label = LabelWidget::wrapped(
            Rect::new(0, 0, rect.width(), rect.height()),
            "",
            FontKind::M,
            c.text_wrap_width,
        );
        let rect_image_window = Rect::new(
            rect.x + c.image_window_pos_x,
            rect.y + c.image_window_pos_y,
            TILE_SIZE,
            TILE_SIZE * 2,
        );
        let image_window = if let Some(chara_template_idx) = chara_template_idx {
            Some(ImageWindow::chara(rect_image_window, chara_template_idx))
        } else {
            None
        };
        let mut talk_window = TalkWindow {
            rect,
            talk_text,
            label,
            image_window,
            msg_text: MsgText::default(),
            choose_win: None,
        };
        talk_window.update_page(Some(talk_text));
        talk_window
    }

    fn update_page(&mut self, talk_text: Option<TalkText>) {
        if let Some(talk_text) = talk_text {
            self.talk_text = talk_text;
            self.msg_text = MsgText::new(&*talk_text.text_id);
            self.choose_win = None;
        }

        // Create answers
        if self.msg_text.is_final_page() {
            if let Some(choices) = self.talk_text.choices {
                let winpos = WindowPos::new(
                    WindowHPos::RightX(self.rect.right()),
                    WindowVPos::TopMargin(self.rect.bottom() + UI_CFG.gap_len_between_dialogs),
                );
                let choices: Vec<String> = choices
                    .iter()
                    .map(|a| text::talk_txt(&*a.0).to_owned())
                    .collect();
                self.choose_win = Some(ChooseWindow::new(winpos, choices, None));
            }
        }

        self.label.set_text(self.msg_text.text());
    }
}

impl Window for TalkWindow {
    fn draw(&mut self, context: &mut Context, game: &Game, anim: Option<(&Animation, u32)>) {
        if let Some(image_window) = self.image_window.as_mut() {
            image_window.draw(context, game, anim);
        }
        draw_rect_border(context, self.rect);
        self.label.draw(context);
        if let Some(ref mut choose_win) = self.choose_win {
            choose_win.draw(context, game, anim);
        }
    }
}

impl DialogWindow for TalkWindow {
    fn process_command(&mut self, command: &Command, pa: &mut DoPlayerAction) -> DialogResult {
        if let Some(ref mut choose_win) = self.choose_win {
            match choose_win.process_command(command, pa) {
                // When one answer is choosed
                DialogResult::CloseWithValue(choosed_answer) => {
                    if let Ok(choosed_answer) = choosed_answer.downcast::<u32>() {
                        match pa.advance_talk(Some(*choosed_answer)) {
                            AdvanceScriptResult::UpdateTalkText(talk_text) => {
                                self.update_page(Some(talk_text));
                                return DialogResult::Continue;
                            }
                            AdvanceScriptResult::Continue => {
                                return DialogResult::Continue;
                            }
                            AdvanceScriptResult::Quit => {
                                return DialogResult::Close;
                            }
                        }
                    }
                }
                _ => (),
            }
        }

        match *command {
            Command::Enter => {
                // If all text of the section has been displayed,
                // proceeds the talk to next section
                if self.msg_text.is_final_page() {
                    match pa.advance_talk(None) {
                        AdvanceScriptResult::UpdateTalkText(talk_text) => {
                            self.update_page(Some(talk_text));
                            DialogResult::Continue
                        }
                        AdvanceScriptResult::Continue => DialogResult::Continue,
                        AdvanceScriptResult::Quit => DialogResult::Close,
                    }
                } else {
                    self.msg_text.next_page();
                    self.update_page(None);
                    DialogResult::Continue
                }
            }
            _ => DialogResult::Continue,
        }
    }

    fn mode(&self) -> InputMode {
        InputMode::Dialog
    }

    /// When child window is closed, call advance_script(), and update text.
    fn callback_child_closed(
        &mut self,
        _result: Option<Box<dyn Any>>,
        pa: &mut DoPlayerAction,
    ) -> DialogResult {
        match pa.advance_script() {
            AdvanceScriptResult::UpdateTalkText(talk_text) => {
                self.update_page(Some(talk_text));
                DialogResult::Continue
            }
            AdvanceScriptResult::Continue => DialogResult::Continue,
            AdvanceScriptResult::Quit => DialogResult::Close,
        }
    }
}

/// Manage lines of text message
#[derive(Default)]
struct MsgText {
    text: Vec<String>,
    current_page: usize,
}

impl MsgText {
    fn new(text_id: &str) -> MsgText {
        let text: Vec<String> = if let Some(s) = text::talk_txt_checked(text_id) {
            s.split("\n\n").map(|s| s.to_owned()).collect()
        } else {
            vec![text_id.to_owned()]
        };

        MsgText {
            text,
            current_page: 0,
        }
    }

    fn text(&self) -> &str {
        &self.text[self.current_page]
    }

    fn next_page(&mut self) {
        self.current_page += 1;
    }

    fn is_final_page(&self) -> bool {
        self.current_page >= self.text.len() - 1
    }
}
