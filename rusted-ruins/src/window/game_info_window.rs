use super::commonuse::*;
use super::widget::*;
use crate::config::UI_CFG;
use crate::context::textrenderer::FontKind;

/// Game play information viewer
pub struct GameInfoWindow {
    rect: Rect,
    money_label: LabelWidget,
}

impl GameInfoWindow {
    pub fn new(game: &Game) -> GameInfoWindow {
        let cfg = &UI_CFG.game_info_window;
        let rect: Rect = UI_CFG.info_window.rect.into();
        let money_label = LabelWidget::new(
            cfg.money_label_rect,
            &format!("{} Gold", game.gd.player.money()),
            FontKind::MonoM,
        );

        GameInfoWindow { rect, money_label }
    }
}

impl Window for GameInfoWindow {
    fn draw(&mut self, context: &mut Context, _game: &Game, _anim: Option<(&Animation, u32)>) {
        draw_window_border(context, self.rect);
        self.money_label.draw(context);
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
