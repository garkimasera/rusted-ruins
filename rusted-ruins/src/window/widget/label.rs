
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use game::Command;
use sdlvalues::*;
use config::UI_CFG;
use super::WidgetTrait;

/// Simple label widget.
pub struct LabelWidget {
    rect: Rect,
    cache: TextCache,
    font: FontKind,
    wrap_w: Option<u32>,
}

impl LabelWidget {
    pub fn new<R: Into<Rect>>(rect: R, s: &str, font: FontKind) -> LabelWidget {
        let rect = rect.into();
        let cache = TextCache::one(s, font, UI_CFG.color.normal_font.into());
        LabelWidget {
            rect, cache, font,
            wrap_w: None,
        }
    }

    pub fn wrapped<R: Into<Rect>>(rect: R, s: &str, font: FontKind, w: u32) -> LabelWidget {
        let rect = rect.into();
        let cache = TextCache::one_wrapped(s, font, UI_CFG.color.normal_font.into(), w);
        LabelWidget {
            rect, cache, font,
            wrap_w: Some(w),
        }
    }

    pub fn set_text(&mut self, text: &str) {
        let cache = if let Some(w) = self.wrap_w {
            TextCache::one_wrapped(text, self.font, UI_CFG.color.normal_font.into(), w)
        }else{
            TextCache::one(text, self.font, UI_CFG.color.normal_font.into())
        };
        self.cache = cache;
    }
}

impl WidgetTrait for LabelWidget {
    type Response = ();
    fn process_command(&mut self, _command: &Command) -> Option<()> {
        None
    }
    
    fn draw(&mut self, canvas: &mut WindowCanvas, sv: &mut SdlValues) {
        let tex = sv.tt_one(&mut self.cache);

        let w = tex.query().width;
        let h = tex.query().height;
        let dest = Rect::new(self.rect.x + UI_CFG.label_widget.left_margin, self.rect.y, w, h);
        
        check_draw!(canvas.copy(tex, None, dest));
        
    }
}

