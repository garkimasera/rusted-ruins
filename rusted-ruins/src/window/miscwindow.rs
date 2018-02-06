
use super::commonuse::*;
use super::widget::*;
use sdlvalues::FontKind;

pub struct PageWindow {
    rect: Rect,
    label: LabelWidget,
}

impl PageWindow {
    pub fn new(rect: Rect) -> PageWindow {
        PageWindow {
            rect,
            label: LabelWidget::new(rect, "", FontKind::M),
        }
    }

    pub fn set_page(&mut self, current_page: u32, max_page: u32) {
        self.label.set_text(&format!("{} / {}", current_page, max_page));
    }
}

impl Window for PageWindow {
    fn redraw(
        &mut self, canvas: &mut WindowCanvas, _game: &Game, sv: &mut SdlValues,
        _anim: Option<(&Animation, u32)>) {

        draw_rect_border(canvas, self.rect);
        self.label.draw(canvas, sv);
    }
}

