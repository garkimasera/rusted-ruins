use super::WidgetTrait;
use crate::context::*;
use common::gobj;
use common::objholder::*;
use sdl2::rect::Rect;

/// Image widget.
pub struct ImageWidget {
    rect: Rect,
    idx: Idx,
}

enum Idx {
    UIImg(UIImgIdx),
    Chara(CharaTemplateIdx),
    Item(ItemIdx),
}

impl ImageWidget {
    /// Create image widget that show a UIImg
    pub fn ui_img<R: Into<Rect>>(rect: R, id: &str) -> ImageWidget {
        let rect = rect.into();
        let idx: UIImgIdx = gobj::id_to_idx(id);

        ImageWidget {
            rect,
            idx: Idx::UIImg(idx),
        }
    }

    pub fn chara<R: Into<Rect>>(rect: R, chara_idx: CharaTemplateIdx) -> ImageWidget {
        let rect = rect.into();
        ImageWidget {
            rect,
            idx: Idx::Chara(chara_idx),
        }
    }

    pub fn item<R: Into<Rect>>(rect: R, item_idx: ItemIdx) -> ImageWidget {
        let rect = rect.into();
        ImageWidget {
            rect,
            idx: Idx::Item(item_idx),
        }
    }
}

impl WidgetTrait for ImageWidget {
    type Response = ();

    fn draw(&mut self, context: &mut Context) {
        match self.idx {
            Idx::UIImg(idx) => {
                context.render_tex(idx, self.rect);
            }
            Idx::Chara(idx) => {
                // Centering to given rect
                context.render_tex_n_center(idx, self.rect, 0);
            }
            Idx::Item(idx) => {
                // Centering to given rect
                context.render_tex_n_center(idx, self.rect, 0);
            }
        }
    }
}
