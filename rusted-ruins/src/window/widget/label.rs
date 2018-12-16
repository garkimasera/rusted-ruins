
use sdl2::rect::Rect;
use crate::context::*;
use crate::config::UI_CFG;
use super::WidgetTrait;

/// Simple label widget.
pub struct LabelWidget {
    rect: Rect,
    cache: TextCache,
    font: FontKind,
    wrap_w: Option<u32>,
    is_bordered: bool,
    centering: bool,
}

impl LabelWidget {
    pub fn new<R: Into<Rect>>(rect: R, s: &str, font: FontKind) -> LabelWidget {
        let rect = rect.into();
        let cache = TextCache::one(s, font, UI_CFG.color.normal_font.into());
        LabelWidget {
            rect, cache, font,
            wrap_w: None, is_bordered: false,
            centering: false,
        }
    }

    pub fn bordered<R: Into<Rect>>(rect: R, s: &str, font: FontKind) -> LabelWidget {
        let rect = rect.into();
        let cache = TextCache::one_bordered(s, font, UI_CFG.color.normal_font.into());
        LabelWidget {
            rect, cache, font,
            wrap_w: None, is_bordered: true,
            centering: false,
        }
    }

    pub fn wrapped<R: Into<Rect>>(rect: R, s: &str, font: FontKind, w: u32) -> LabelWidget {
        let rect = rect.into();
        let cache = TextCache::one_wrapped(s, font, UI_CFG.color.normal_font.into(), w);
        LabelWidget {
            rect, cache, font,
            wrap_w: Some(w), is_bordered: false,
            centering: false,
        }
    }

    pub fn set_text(&mut self, text: &str) {
        let cache = if let Some(w) = self.wrap_w {
            TextCache::one_wrapped(text, self.font, UI_CFG.color.normal_font.into(), w)
        }else{
            if self.is_bordered {
                TextCache::one_bordered(text, self.font, UI_CFG.color.normal_font.into())
            } else {
                TextCache::one(text, self.font, UI_CFG.color.normal_font.into())
            }
        };
        self.cache = cache;
    }

    /// Adjust widget size to fit inner contents if given rect doesn't have value
    /// Returns adjusted size
    pub fn adjust_widget_size(&mut self, sv: &mut SdlValues) -> (u32, u32) {
        let tex = sv.tt_one(&mut self.cache);
        let w = tex.query().width;
        let h = tex.query().height;
        let rect = Rect::new(self.rect.x, self.rect.y, w, h);
        self.rect = rect;
        (w, h)
    }

    pub fn rect(&self) -> Rect {
        self.rect
    }

    pub fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }

    pub fn centering(mut self) -> LabelWidget {
        self.centering = true;
        self
    }
}

impl WidgetTrait for LabelWidget {
    type Response = ();
    
    fn draw(&mut self, context: &mut Context) {
        let canvas = &mut context.canvas;
        let sv = &mut context.sv;
        let tex = sv.tt_one(&mut self.cache);

        let w = tex.query().width;
        let h = tex.query().height;
        let dest = if self.centering {
            Rect::new(
                self.rect.x + (self.rect.w - w as i32) / 2,
                self.rect.y + (self.rect.h - h as i32) / 2,
                w, h)
        } else {
            Rect::new(self.rect.x + UI_CFG.label_widget.left_margin, self.rect.y, w, h)
        };
        
        check_draw!(canvas.copy(tex, None, dest));
        
    }
}

/// Label widget whose lines are specified by user
pub struct LineSpecifiedLabelWidget {
    rect: Rect,
    cache: TextCache,
    font: FontKind,
    max_line: usize,
}

impl LineSpecifiedLabelWidget {
    pub fn new<R: Into<Rect>, S: AsRef<str>>(
        rect: R, s: &[S], font: FontKind, max_line: usize) -> LineSpecifiedLabelWidget {
        
        let rect = rect.into();
        let slice_len = std::cmp::min(s.len(), max_line);
        let cache = TextCache::new(&s[0..slice_len], font, UI_CFG.color.normal_font.into());
        LineSpecifiedLabelWidget {
            rect, cache, font, max_line,
        }
    }

    pub fn set_text<S: AsRef<str>>(&mut self, s: &[S]) {
        let slice_len = std::cmp::min(s.len(), self.max_line);
        let cache = TextCache::new(&s[0..slice_len], self.font, UI_CFG.color.normal_font.into());
        self.cache = cache;
    }
}

impl WidgetTrait for LineSpecifiedLabelWidget {
    type Response = ();
    
    fn draw(&mut self, context: &mut Context) {
        let canvas = &mut context.canvas;
        let sv = &mut context.sv;
        let tex_group = sv.tt_group(&mut self.cache);

        let mut y = 0;

        for tex in tex_group {
            let w = tex.query().width;
            let h = tex.query().height;
            let dest = Rect::new(self.rect.x + UI_CFG.label_widget.left_margin, self.rect.y + y, w, h);
            y += h as i32;
            check_draw!(canvas.copy(tex, None, dest));
        }
    }
}
