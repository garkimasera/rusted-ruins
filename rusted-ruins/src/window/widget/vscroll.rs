use super::WidgetTrait;
use crate::context::*;
use sdl2::rect::Rect;

/// Vertical scroll widget
pub struct VScrollWidget {
    rect: Rect,
}

impl VScrollWidget {
    pub fn new(rect: Rect) -> VScrollWidget {
        VScrollWidget { rect }
    }
}

impl WidgetTrait for VScrollWidget {
    type Response = ();

    fn draw(&mut self, context: &mut Context) {}
}
