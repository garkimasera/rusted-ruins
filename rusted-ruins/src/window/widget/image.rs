
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use crate::context::*;
use common::obj::ImgObject;
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
    Chara(CharaTemplateIdx),
}

impl ImageWidget {
    /// Create image widget that show a UIImg
    pub fn ui_img<R: Into<Rect>>(rect: R, id: &str) -> ImageWidget {
        let rect = rect.into();
        let idx: UIImgIdx = gobj::id_to_idx(id);
        
        ImageWidget {
            rect, idx: Idx::UIImg(idx),
        }
    }

    pub fn chara<R: Into<Rect>>(rect: R, chara_idx: CharaTemplateIdx) -> ImageWidget {
        let rect = rect.into();
        ImageWidget {
            rect, idx: Idx::Chara(chara_idx),
        }
    }
}

impl WidgetTrait for ImageWidget {
    type Response =  ();

    fn draw(&mut self, canvas: &mut WindowCanvas, sv: &mut SdlValues) {
        match self.idx {
            Idx::UIImg(idx) => {
                let t = sv.tex().get(idx);
                check_draw!(canvas.copy(t, None, self.rect));
            }
            Idx::Chara(idx) => { // Centering to given rect
                let t = sv.tex().get(idx);
                let chara_obj = gobj::get_obj(idx);
                let orig_rect: Rect = chara_obj.img_rect().into();
                let dest_rect = Rect::new(
                    self.rect.x + (self.rect.w - orig_rect.w) / 2,
                    self.rect.y + (self.rect.h - orig_rect.h) / 2,
                    orig_rect.w as u32, orig_rect.h as u32);
                check_draw!(canvas.copy(t, orig_rect, dest_rect));
            }
        }
    }
}
