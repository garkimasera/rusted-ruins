
use config::UI_CFG;
use super::commonuse::*;
use super::text_input;

pub struct TextInputDialog {
    rect: Rect,
    text: String,
}

impl TextInputDialog {
    pub fn new() -> TextInputDialog {
        text_input::start();
        TextInputDialog {
            rect: UI_CFG.text_input_dialog.rect.into(),
            text: String::new(),
        }
    }
}

impl Window for TextInputDialog {
    fn redraw(
        &mut self, canvas: &mut WindowCanvas, _game: &Game, sv: &mut SdlValues,
        _anim: Option<(&Animation, u32)>) {
        
        draw_rect_border(canvas, self.rect);
    }
}

impl DialogWindow for TextInputDialog {
    fn process_command(&mut self, command: Command, _pa: DoPlayerAction) -> DialogResult {
        match command {
            Command::TextInput { text } => {
                println!("{}", text);
                self.text.push_str(&text);
                DialogResult::Continue
            },
            Command::Enter => {
                println!("{}", self.text);
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

