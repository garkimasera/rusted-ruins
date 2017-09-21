
use super::commonuse::*;

pub struct TextWindow {
    rect: Rect,
}

impl TextWindow {
    pub fn new(rect: Rect) -> TextWindow {
        TextWindow {
            rect,
        }
    }
}

impl Window for TextWindow {
    
    fn redraw(
        &mut self, canvas: &mut WindowCanvas, _game: &Game, sv: &mut SdlValues,
        _anim: Option<(&Animation, u32)>) {
        
    }
}

