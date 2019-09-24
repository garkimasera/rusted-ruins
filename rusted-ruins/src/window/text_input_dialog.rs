use super::commonuse::*;
use super::text_input;
use super::widget::*;
use crate::config::UI_CFG;
use crate::context::textrenderer::FontKind;

pub struct TextInputDialog {
    label: LabelWidget,
    rect: Rect,
    text: String,
    callback: Option<Box<dyn Fn(&mut DoPlayerAction, &str)>>,
}

impl TextInputDialog {
    pub fn new() -> TextInputDialog {
        text_input::start();

        let rect: Rect = UI_CFG.text_input_dialog.rect.into();
        let label_rect = Rect::new(0, 0, rect.width(), rect.height());

        TextInputDialog {
            label: LabelWidget::new(label_rect, "", FontKind::M),
            rect,
            text: String::new(),
            callback: None,
        }
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }

    pub fn set_callback<F: Fn(&mut DoPlayerAction, &str) + 'static>(&mut self, callback: F) {
        self.callback = Some(Box::new(callback));
    }

    /// This function is used when the result string is invalid,
    /// and text input is needed again.
    pub fn restart(&self) {
        text_input::start();
    }
}

impl Window for TextInputDialog {
    fn draw(&mut self, context: &mut Context, _game: &Game, _anim: Option<(&Animation, u32)>) {
        draw_rect_border(context, self.rect);
        self.label.draw(context);
    }
}

impl DialogWindow for TextInputDialog {
    fn process_command(&mut self, command: &Command, pa: &mut DoPlayerAction) -> DialogResult {
        match command {
            Command::TextInput { ref text } => {
                self.text.push_str(&text);
                self.label.set_text(&self.text);
                DialogResult::Continue
            }
            Command::TextDelete => {
                self.text.pop();
                self.label.set_text(&self.text);
                DialogResult::Continue
            }
            Command::Enter => {
                text_input::end();
                if let Some(callback) = &self.callback {
                    callback(pa, &self.text);
                }
                DialogResult::Close
            }
            Command::Cancel => {
                text_input::end();
                DialogResult::Close
            }
            _ => DialogResult::Continue,
        }
    }

    fn mode(&self) -> InputMode {
        InputMode::TextInput
    }
}
