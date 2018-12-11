
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
        self.rect.w = std::cmp::max(window_size.0 as i32, self.min_w);
        self.rect.h = window_size.1 as i32;

        draw_rect_border(canvas, self.rect);
        self.label.draw(canvas, sv);
    }
}

pub struct ScrollingTextWindow {
    rect: Rect,
    elapsed_frame: i32,
    labels: Vec<LabelWidget>,
    is_adjusted: bool,
    is_finished: bool,
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
            is_adjusted: false,
            is_finished: false,
        }
    }

    pub fn is_finished(&self) -> bool {
        self.is_finished
    }
}

impl Window for ScrollingTextWindow {
    
    fn draw(
        &mut self, canvas: &mut WindowCanvas, _game: &Game, sv: &mut SdlValues,
        _anim: Option<(&Animation, u32)>) {

        if self.is_finished { return; }

        if !self.is_adjusted {
            for label in self.labels.iter_mut() {
                label.adjust_widget_size(sv);
            }
            self.is_adjusted = true;
        }

        let line_space = UI_CFG.scrolling_text_window.line_space;
        let speed = UI_CFG.scrolling_text_window.speed;
        let start_y = self.rect.h;
        let mut endline_bottom = 0;
        
        // Update label widgets positions
        for (i, label) in self.labels.iter_mut().enumerate() {
            let rect = label.rect();
            let rect = Rect::new(
                self.rect.w / 2 - rect.w / 2,
                (-speed * self.elapsed_frame as f64) as i32 + line_space * i as i32 + start_y,
                rect.width(),
                rect.height());
            label.set_rect(rect);
            endline_bottom = rect.bottom();
        }

        canvas.set_viewport(self.rect);
            
        for label in self.labels.iter_mut() {
            label.draw(canvas, sv);
        }

        if !self.is_finished {
            if endline_bottom < 0 {
                self.is_finished = true;
            }
            self.elapsed_frame += 1;
        }
    }
}
