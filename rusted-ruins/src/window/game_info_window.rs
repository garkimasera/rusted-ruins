
use super::commonuse::*;
use super::widget::*;
use sdlvalues::FontKind;
use config::UI_CFG;
use common::gamedata::*;

/// Game play information viewer
pub struct GameInfoWindow {
    rect: Rect,
}

impl GameInfoWindow {
    pub fn new(game: &Game) -> GameInfoWindow {
        let cfg = &UI_CFG.game_info_window;
        let rect: Rect = cfg.rect.into();
        
        GameInfoWindow {
            rect,
        }
    }
}

impl Window for GameInfoWindow {
    fn draw(
        &mut self, canvas: &mut WindowCanvas, _game: &Game, sv: &mut SdlValues,
        _anim: Option<(&Animation, u32)>) {

        draw_rect_border(canvas, self.rect);
    }
}

impl DialogWindow for GameInfoWindow {
    fn process_command(&mut self, command: &Command, _pa: &mut DoPlayerAction) -> DialogResult {
        match *command {
            Command::Cancel => DialogResult::Close,
            _ => DialogResult::Continue,
        }
    }

    fn mode(&self) -> InputMode {
        InputMode::Dialog
    }
}

