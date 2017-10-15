
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdlvalues::*;
use game::Command;
use super::WidgetTrait;

/// Icon widget.
pub struct IconWidget {
    rect: Rect,
    idx: IconIdx,
}

impl IconWidget {
    /// Create icon widget that has normal size (24x24)
    pub fn normal<I: Into<IconIdx>>(rect: Rect, i: I) -> IconWidget {
        IconWidget {
            rect: rect,
            idx: i.into(),
        }
    }
}

impl WidgetTrait for IconWidget {
    type Response =  ();
    fn process_command(&mut self, _command: &Command) -> Option<()> {
        None
    }

    fn draw(&mut self, canvas: &mut WindowCanvas, sv: &mut SdlValues) {
        let orig: Rect;

        let (t, r) = sv.tex().get_icon(self.idx);
        
        check_draw!(canvas.copy(t, r, self.rect));
    }
}

