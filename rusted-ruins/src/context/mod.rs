
use sdl2::render::WindowCanvas;

pub mod sdlvalues;
pub mod texture;
pub mod textrenderer;
pub mod textcachepool;

pub use self::sdlvalues::SdlValues;
pub use self::texture::IconIdx;
pub use self::textrenderer::FontKind;
pub use self::textcachepool::TextCache;

/// Wrapper for SDL drawing functions
pub struct Context<'a, 'b, 't, 'sdl> {
    pub canvas: &'a mut WindowCanvas,
    pub sv: &'b mut SdlValues<'t, 'sdl>,
}

impl<'a, 'b, 't, 'sdl> Context<'a, 'b, 't, 'sdl> {
    fn new(canvas: &'a mut WindowCanvas, sv: &'b mut SdlValues<'t, 'sdl>) -> Context<'a, 'b, 't, 'sdl> {
        Context {
            canvas, sv
        }
    }
}

