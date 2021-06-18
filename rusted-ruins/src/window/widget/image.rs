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
    idx: ImageIdx,
}

pub enum ImageIdx {
    UiImg(UiImgIdx),
    Chara(CharaTemplateIdx),
    Item((ItemIdx, u32)),
}

impl ImageIdx {
    pub fn item(item: &Item) -> Self {
        let mut variation = 0;
        for attr in &item.attrs {
            if let ItemAttr::ImageVariation(n) = attr {
                variation = *n;
            }
        }

        ImageIdx::Item((item.idx, variation))
    }
}

impl ImageWidget {
    pub fn new<R: Into<Rect>>(rect: R, idx: ImageIdx) -> Self {
        let rect = rect.into();
        ImageWidget { rect, idx }
    }

    /// Create image widget that show a UIImg
    pub fn ui_img<R: Into<Rect>>(rect: R, id: &str) -> Self {
        let idx: UiImgIdx = gobj::id_to_idx(id);

        Self::new(rect, ImageIdx::UiImg(idx))
    }

    pub fn chara<R: Into<Rect>>(rect: R, chara_idx: CharaTemplateIdx) -> Self {
        Self::new(rect, ImageIdx::Chara(chara_idx))
    }

    pub fn item<R: Into<Rect>>(rect: R, item: &Item) -> Self {
        Self::new(rect, ImageIdx::item(item))
    }

    pub fn set_rect<R: Into<Rect>>(&mut self, rect: R) {
        self.rect = rect.into();
    }
}

impl WidgetTrait for ImageWidget {
    type Response = ();

    fn draw(&mut self, context: &mut Context) {
        match self.idx {
            ImageIdx::UiImg(idx) => {
                context.render_tex(idx, self.rect);
            }
            ImageIdx::Chara(idx) => {
                // Centering to given rect
                context.render_tex_n_center(idx, self.rect, 0);
            }
            ImageIdx::Item(idx) => {
                // Centering to given rect
                context.render_tex_n_center(idx.0, self.rect, idx.1);
            }
        }
    }
}
