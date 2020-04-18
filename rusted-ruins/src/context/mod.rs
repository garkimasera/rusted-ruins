use sdl2::render::WindowCanvas;

pub mod sdlvalues;
pub mod textcachepool;
pub mod textrenderer;
pub mod texture;

pub use self::sdlvalues::SdlValues;
pub use self::textcachepool::TextCache;
pub use self::textrenderer::FontKind;
pub use self::texture::IconIdx;

use common::objholder::Holder;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Texture;

/// Wrapper for SDL drawing functions
pub struct Context<'a, 'b, 't, 'sdl> {
    pub canvas: &'a mut WindowCanvas,
    pub sv: &'b mut SdlValues<'t, 'sdl>,
}

impl<'a, 'b, 't, 'sdl> Context<'a, 'b, 't, 'sdl> {
    pub fn new(
        canvas: &'a mut WindowCanvas,
        sv: &'b mut SdlValues<'t, 'sdl>,
    ) -> Context<'a, 'b, 't, 'sdl> {
        Context { canvas, sv }
    }

    pub fn set_viewport<R: Into<Option<Rect>>>(&mut self, rect: R) {
        self.canvas.set_viewport(rect);
    }

    pub fn render_tex<I>(&mut self, idx: I, dest: Rect)
    where
        for<'th> self::texture::TextureHolder<'th>:
            common::objholder::Holder<I, ReturnType = Texture<'th>>,
    {
        let tex = self.sv.tex().get(idx);
        try_sdl!(self.canvas.copy(tex, None, dest));
    }

    pub fn render_tex_n<I, O>(&mut self, idx: I, dest: Rect, n_image: u32)
    where
        for<'th> self::texture::TextureHolder<'th>:
            common::objholder::Holder<I, ReturnType = Texture<'th>>,
        I: common::objholder::ObjectIndex<ObjectType = O> + Copy,
        O: common::obj::ImgObject + 'static,
    {
        let tex = self.sv.tex().get(idx);
        let obj = common::gobj::get_obj(idx);
        let src: Rect = obj.img_rect_nth(n_image).into();
        try_sdl!(self.canvas.copy(tex, src, dest));
    }

    pub fn render_tex_n_center<I, O>(&mut self, idx: I, dest: Rect, n_image: u32)
    where
        for<'th> self::texture::TextureHolder<'th>:
            common::objholder::Holder<I, ReturnType = Texture<'th>>,
        I: common::objholder::ObjectIndex<ObjectType = O> + Copy,
        O: common::obj::ImgObject + 'static,
    {
        let tex = self.sv.tex().get(idx);
        let obj = common::gobj::get_obj(idx);
        let src: Rect = obj.img_rect_nth(n_image).into();
        let dest = Rect::new(
            dest.x + (dest.w - src.w) / 2,
            dest.y + (dest.h - src.h) / 2,
            src.w as u32,
            src.h as u32,
        );
        try_sdl!(self.canvas.copy(tex, src, dest));
    }

    pub fn render_tex_n_bottom<I, O>(&mut self, idx: I, dest: Rect, n_image: u32)
    where
        for<'th> self::texture::TextureHolder<'th>:
            common::objholder::Holder<I, ReturnType = Texture<'th>>,
        I: common::objholder::ObjectIndex<ObjectType = O> + Copy,
        O: common::obj::ImgObject + 'static,
    {
        let tex = self.sv.tex().get(idx);
        let obj = common::gobj::get_obj(idx);
        let src: Rect = obj.img_rect_nth(n_image).into();
        let dest = Rect::new(
            dest.x + (dest.w - src.w) / 2,
            dest.y + dest.h - src.h,
            src.w as u32,
            src.h as u32,
        );
        try_sdl!(self.canvas.copy(tex, src, dest));
    }

    pub fn draw_rect<R: Into<Rect>, T: Into<Color>>(&mut self, rect: R, color: T) {
        self.canvas.set_draw_color(color.into());
        try_sdl!(self.canvas.draw_rect(rect.into()))
    }

    pub fn fill_rect<R: Into<Rect>, T: Into<Color>>(&mut self, rect: R, color: T) {
        self.canvas.set_draw_color(color.into());
        try_sdl!(self.canvas.fill_rect(rect.into()))
    }
}
