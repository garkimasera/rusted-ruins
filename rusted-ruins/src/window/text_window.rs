
use super::commonuse::*;
use super::widget::*;
use sdlvalues::FontKind;

pub struct TextWindow {
    rect: Rect,
    min_w: i32,
    label: LabelWidget,
}

impl TextWindow {
    pub fn new(rect: Rect, s: &str) -> TextWindow {
        TextWindow {
            rect,
            min_w: rect.w,
            label: LabelWidget::wrapped(
                (0, 0, rect.w as u32, 0), s, FontKind::M, rect.w as u32),
        }
    }

    pub fn get_rect(&self) -> Rect {
        self.rect
    }
}

impl Window for TextWindow {
    
    fn draw(
        &mut self, canvas: &mut WindowCanvas, _game: &Game, sv: &mut SdlValues,
        _anim: Option<(&Animation, u32)>) {

        let window_size = self.label.adjust_widget_size(sv);
        self.rect.w = ::std::cmp::max(window_size.0 as i32, self.min_w);
        self.rect.h = window_size.1 as i32;

        draw_rect_border(canvas, self.rect);
        self.label.draw(canvas, sv);
    }
}

pub struct ScrollingTextWindow {
    rect: Rect,
    elapsed_frame: u64,
    labels: Vec<LabelWidget>,
}

impl ScrollingTextWindow {
    pub fn new(rect: Rect, s: &str) -> ScrollingTextWindow {
        let mut labels = Vec::new();

        for line in s.lines() {
            let widget_rect = Rect::new(0, 0, 0, 0);
            labels.push(LabelWidget::bordered(widget_rect, line, FontKind::M));
        }
        
        ScrollingTextWindow {
            rect: rect,
            elapsed_frame: 0,
            labels: labels,
        }
    }   
}

impl Window for ScrollingTextWindow {
    
    fn draw(
        &mut self, canvas: &mut WindowCanvas, _game: &Game, sv: &mut SdlValues,
        _anim: Option<(&Animation, u32)>) {

        /*let window_size = self.label.adjust_widget_size(sv);
        self.rect.w = ::std::cmp::max(window_size.0 as i32, self.min_w);
        self.rect.h = window_size.1 as i32;

        draw_rect_border(canvas, self.rect);*/
        for label in self.labels.iter_mut() {
            label.draw(canvas, sv);
        }
    }
}
