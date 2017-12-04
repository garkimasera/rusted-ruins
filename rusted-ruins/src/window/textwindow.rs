
use super::commonuse::*;
use super::widget::*;
use sdlvalues::FontKind;

pub struct TextWindow {
    rect: Rect,
    label: LabelWidget,
}

impl TextWindow {
    pub fn new(rect: Rect) -> TextWindow {
        TextWindow {
            rect,
            label: LabelWidget::wrapped(
                (0, 0, rect.w as u32, 0), "", FontKind::M, rect.w as u32),
        }
    }
}

impl Window for TextWindow {
    
    fn redraw(
        &mut self, canvas: &mut WindowCanvas, _game: &Game, sv: &mut SdlValues,
        _anim: Option<(&Animation, u32)>) {

        draw_rect_border(canvas, self.rect);
        self.label.draw(canvas, sv);
    }
}

