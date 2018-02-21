
use super::commonuse::*;
use super::widget::*;
use sdlvalues::FontKind;
use config::UI_CFG;
use text;

/// Character status viewer
pub struct StatusWindow {
    rect: Rect,
}

impl StatusWindow {
    pub fn new() -> StatusWindow {
        let rect: Rect = UI_CFG.status_window.rect.into();
        let mut status_window = StatusWindow {
            rect: rect,
        };
        status_window
    }
}

impl Window for StatusWindow {
    fn redraw(
        &mut self, canvas: &mut WindowCanvas, _game: &Game, sv: &mut SdlValues,
        _anim: Option<(&Animation, u32)>) {

        draw_rect_border(canvas, self.rect);
    }
}

impl DialogWindow for StatusWindow {
    fn process_command(&mut self, command: &Command, pa: &mut DoPlayerAction) -> DialogResult {
        match *command {
            Command::Cancel => DialogResult::Close,
            _ => DialogResult::Continue,
        }
    }

    fn mode(&self) -> InputMode {
        InputMode::Dialog
    }
}

