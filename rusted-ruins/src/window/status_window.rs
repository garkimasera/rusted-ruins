
use super::commonuse::*;
use super::widget::*;
use sdlvalues::FontKind;
use config::UI_CFG;
use common::gamedata::GameData;
use common::gamedata::chara::*;
use text;

/// Character status viewer
pub struct StatusWindow {
    rect: Rect,
    image: ImageWidget,
    name_label: LabelWidget,
}

impl StatusWindow {
    pub fn new(gd: &GameData) -> StatusWindow {
        let cfg = &UI_CFG.status_window;
        let rect: Rect = cfg.rect.into();
        let chara = gd.chara.get(CharaId::Player);
        let image = ImageWidget::chara(cfg.image_rect, chara.template);
        let name_label = LabelWidget::new(cfg.name_label_rect, &chara.name, FontKind::M);
        let mut status_window = StatusWindow {
            rect,
            image, name_label
        };
        status_window
    }
}

impl Window for StatusWindow {
    fn redraw(
        &mut self, canvas: &mut WindowCanvas, _game: &Game, sv: &mut SdlValues,
        _anim: Option<(&Animation, u32)>) {

        draw_rect_border(canvas, self.rect);
        self.image.draw(canvas, sv);
        self.name_label.draw(canvas, sv);
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

