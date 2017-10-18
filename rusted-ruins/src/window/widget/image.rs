
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdlvalues::*;
use common::gobj;
use common::objholder::*;
use super::WidgetTrait;

/// Image widget.
pub struct ImageWidget {
    rect: Rect,
    idx: Idx,
}

enum Idx {
    UIImg(UIImgIdx),
}

impl ImageWidget {
    /// Create image widget that show a UIImg
    pub fn ui_img(rect: Rect, id: &str) -> ImageWidget {
        let idx: UIImgIdx = gobj::id_to_idx(id);
        
        ImageWidget {
            rect, idx: Idx::UIImg(idx),
        }
    }
}

impl WidgetTrait for ImageWidget {
    type Response =  ();

    fn draw(&mut self, canvas: &mut WindowCanvas, sv: &mut SdlValues) {
        let t = match self.idx {
            Idx::UIImg(idx) => sv.tex().get(idx)
        };
        
        check_draw!(canvas.copy(t, None, self.rect));
    }
}
