
use common::gamedata::chara::CharaTalk;
use super::commonuse::*;
use super::widget::*;
use sdlvalues::FontKind;
use config::UI_CFG;

pub struct TalkWindow {
    rect: Rect,
    label: LabelWidget,
}

impl TalkWindow {
    pub fn new(chara_talk: CharaTalk) -> TalkWindow {
        let rect = UI_CFG.talk_window.rect.into();
        TalkWindow {
            rect: rect,
            label: LabelWidget::wrapped(
                rect, "", FontKind::M, rect.w as u32),
        }
    }
}

impl Window for TalkWindow {
    
    fn redraw(
        &mut self, canvas: &mut WindowCanvas, _game: &Game, sv: &mut SdlValues,
        _anim: Option<(&Animation, u32)>) {

        draw_rect_border(canvas, self.rect);
        self.label.draw(canvas, sv);
    }
}

impl DialogWindow for TalkWindow {
    fn process_command(&mut self, command: Command, pa: DoPlayerAction) -> DialogResult {
        match command {
            Command::Cancel => {
                DialogResult::Close
            },
            _ => DialogResult::Continue,
        }
    }

    fn mode(&self) -> InputMode {
        InputMode::Dialog
    }
}
