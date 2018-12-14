
use super::commonuse::*;
use super::widget::*;
use crate::context::textrenderer::FontKind;
use crate::config::UI_CFG;

/// Game play information viewer
pub struct GameInfoWindow {
    rect: Rect,
    money_label: LabelWidget,
}

impl GameInfoWindow {
    pub fn new(game: &Game) -> GameInfoWindow {
        let cfg = &UI_CFG.game_info_window;
        let rect: Rect = cfg.rect.into();
        let money_label = LabelWidget::new(
            cfg.money_label_rect, &format!("{} Gold", game.gd.player.money()), FontKind::MonoM);
        
        GameInfoWindow {
            rect,
            money_label,
        }
    }
}

impl Window for GameInfoWindow {
    fn draw(
        &mut self, canvas: &mut WindowCanvas, _game: &Game, sv: &mut SdlValues,
        _anim: Option<(&Animation, u32)>) {

        draw_rect_border(canvas, self.rect);
        self.money_label.draw(canvas, sv);
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

