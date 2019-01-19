
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;
use super::texture::TextureHolder;
use super::textrenderer::TextRenderer;
use super::textcachepool::TextCachePool;
use super::textcachepool::TextCache;
use common::gobj;
use crate::SdlContext;

/// Includes data that isn't used by Game
/// Used for rendering, or music/sound playing
pub struct SdlValues<'sdl, 't> {
    pub tc: &'t TextureCreator<WindowContext>,
    pub texture_holder: TextureHolder<'t>,
    pub text_renderer: TextRenderer<'sdl>,
    pub tcp: TextCachePool<'t>,
}

impl<'sdl, 't> SdlValues<'sdl, 't> {
    pub fn new(
        sdl_context: &'sdl SdlContext,
        tc: &'t TextureCreator<WindowContext>) -> SdlValues<'sdl, 't> {

        SdlValues {
            tc,
            texture_holder: TextureHolder::new(gobj::get_objholder(), tc),
            text_renderer: TextRenderer::new(sdl_context),
            tcp: TextCachePool::new(),
        }
    }

    pub fn tex(&self) -> &TextureHolder {
        &self.texture_holder
    }

    pub fn tt_group(&mut self, c: &mut TextCache) -> &[Texture] {
        self.tcp.group(c, &self.text_renderer, self.tc)
    }

    pub fn tt_one(&mut self, c: &mut TextCache) -> &Texture {
        &self.tcp.group(c, &self.text_renderer, self.tc)[0]
    }
}



