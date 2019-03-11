use super::commonuse::*;
use crate::config::UI_CFG;
use crate::draw::border::draw_rect_border;

pub struct CreationWindow {
    rect: Rect,
}

impl CreationWindow {
    pub fn new() -> CreationWindow {
        let c = &UI_CFG.creation_window;

        CreationWindow {
            rect: c.rect.into(),
        }
    }
}

impl Window for CreationWindow {
    fn draw(&mut self, context: &mut Context, _game: &Game, _anim: Option<(&Animation, u32)>) {
        draw_rect_border(context, self.rect)
    }
}

impl DialogWindow for CreationWindow {
    fn process_command(&mut self, command: &Command, _pa: &mut DoPlayerAction) -> DialogResult {
        match command {
            Command::Cancel => DialogResult::Close,
            _ => DialogResult::Continue,
        }
    }

    fn mode(&self) -> InputMode {
        InputMode::Dialog
    }
}
