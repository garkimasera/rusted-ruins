
use config::UI_CFG;
use sdlvalues::FontKind;
use super::commonuse::*;
use super::text_input;
use super::widget::*;

pub struct TextInputDialog {
    label: LabelWidget,
    rect: Rect,
    text: String,
}

impl TextInputDialog {
    pub fn new() -> TextInputDialog {
        text_input::start();

        let rect: Rect = UI_CFG.text_input_dialog.rect.into();
        let label_rect = Rect::new(0, 0, rect.width(), rect.height());
        
        TextInputDialog {
            label: LabelWidget::new(label_rect, "", FontKind::M),
            rect: rect,
            text: String::new(),
        }
    }
}

impl Window for TextInputDialog {
    fn redraw(
        &mut self, canvas: &mut WindowCanvas, _game: &Game, sv: &mut SdlValues,
        _anim: Option<(&Animation, u32)>) {
        
        draw_rect_border(canvas, self.rect);
        self.label.draw(canvas, sv);
    }
}

impl DialogWindow for TextInputDialog {
    fn process_command(&mut self, command: Command, _pa: DoPlayerAction) -> DialogResult {
        match command {
            Command::TextInput { text } => {
                self.text.push_str(&text);
                self.label.set_text(&self.text);
                DialogResult::Continue
            },
            Command::TextDelete => {
                self.text.pop();
                self.label.set_text(&self.text);
                DialogResult::Continue
            },
            Command::Enter => {
                text_input::end();
                DialogResult::Close
            },
            Command::Cancel => {
                text_input::end();
                DialogResult::Close
            },
            _ => DialogResult::Continue,
        }
    }

    fn mode(&self) -> InputMode {
        InputMode::TextInput
    }
}

