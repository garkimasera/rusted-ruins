
use super::commonuse::*;
use super::widget::*;
use sdlvalues::FontKind;
use config::UI_CFG;

pub struct PageWindow {
    rect: Rect,
    label: LabelWidget,
}

impl PageWindow {
    pub fn new(x: Option<i32>, y: Option<i32>) -> PageWindow {
        let mut rect: Rect = UI_CFG.page_window.rect.into();

        if let Some(x) = x { rect.set_x(x) }
        if let Some(y) = y { rect.set_y(y) }
        
        PageWindow {
            rect,
            label: LabelWidget::new(Rect::new(0, UI_CFG.page_window.label_y, rect.width(), rect.height()),
                                    "1 / 1", FontKind::M),
        }
    }

    pub fn set_page(&mut self, current_page: u32, max_page: u32) {
        self.label.set_text(&format!("{} / {}", current_page + 1, max_page + 1));
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

