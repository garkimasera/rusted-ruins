use super::WidgetTrait;
use crate::context::*;
use common::gamedata::Item;
use common::gamedata::*;
use common::gobj;
use common::objholder::*;
use sdl2::rect::Rect;

/// Image widget.
pub struct ImageWidget {
    rect: Rect,
    idx: Idx,
}

enum Idx {
    UiImg(UiImgIdx),
    Chara(CharaTemplateIdx),
    Item((ItemIdx, u32)),
}

impl ImageWidget {
    /// Create image widget that show a UIImg
    pub fn ui_img<R: Into<Rect>>(rect: R, id: &str) -> ImageWidget {
        let rect = rect.into();
        let idx: UiImgIdx = gobj::id_to_idx(id);

        ImageWidget {
            rect,
            idx: Idx::UiImg(idx),
        }
    }

    pub fn chara<R: Into<Rect>>(rect: R, chara_idx: CharaTemplateIdx) -> ImageWidget {
        let rect = rect.into();
        ImageWidget {
            rect,
            idx: Idx::Chara(chara_idx),
        }
    }

    pub fn item<R: Into<Rect>>(rect: R, item: &Item) -> ImageWidget {
        let mut variation = 0;
        for attr in &item.attrs {
            match attr {
                ItemAttr::ImageVariation(n) => {
                    variation = *n;
                }
                _ => (),
            }
        }

        Self::item_idx(rect, item.idx, variation)
    }

    pub fn item_idx<R: Into<Rect>>(rect: R, item_idx: ItemIdx, n: u32) -> ImageWidget {
        let rect = rect.into();
        ImageWidget {
            rect,
            idx: Idx::Item((item_idx, n)),
        }
    }

    pub fn set_rect<R: Into<Rect>>(&mut self, rect: R) {
        self.rect = rect.into();
    }
}

impl WidgetTrait for ImageWidget {
    type Response = ();

    fn draw(&mut self, context: &mut Context) {
        match self.idx {
            Idx::UiImg(idx) => {
                context.render_tex(idx, self.rect);
            }
            Idx::Chara(idx) => {
                // Centering to given rect
                context.render_tex_n_center(idx, self.rect, 0);
            }
            Idx::Item(idx) => {
                // Centering to given rect
                context.render_tex_n_center(idx.0, self.rect, idx.1);
            }
        }
    }
}
