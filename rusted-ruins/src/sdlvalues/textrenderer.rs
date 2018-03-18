
use std::path::PathBuf;
use sdl2::surface::Surface;
use sdl2::pixels::Color;
use sdl2::ttf::*;
use SdlContext;
use config::{UI_CFG, FONT_CFG, abs_path};
use config::visual::FontConfig;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum FontKind {
    /// For log window
    Log,
    /// For most of window texts
    M,
    /// For small UI texts
    S,
    /// For monospace texts. This font may include ascii characters only
    MonoM,
}

pub struct TextRenderer<'sdl> {
    font_s: Font<'sdl, 'static>,
    font_m: Font<'sdl, 'static>,
    font_log: Font<'sdl, 'static>,
    font_mono_m: Font<'sdl, 'static>,
}

pub const ERR_MSG_FONT_REND: &'static str = "Error occured during font rendering to surface";
pub const ERR_MSG_FONT_TEX: &'static str = "Error occured during texture creation from font surface";

impl<'sdl> TextRenderer<'sdl> {
    pub fn new(sdl_context: &'sdl SdlContext) -> TextRenderer<'sdl> {
        
        const ERR_MSG: &'static str = "Font loading error";
        let font_name = FONT_CFG.font_name();
        let font = &UI_CFG.font;
        let f = |fc: &FontConfig| -> Font {
            sdl_context.ttf_context.load_font(&font_path(&font_name), fc.size).expect(ERR_MSG)
        };
        
        TextRenderer {
            font_s: f(&font.s),
            font_m: f(&font.m),
            font_log: f(&font.log),
            font_mono_m: sdl_context.ttf_context.load_font(
                &font_path(&FONT_CFG.mono_font), font.m.size).expect(ERR_MSG)
        }
    }

    pub fn surface(&self, font_usage: FontKind, text: &str, c: Color, wrap: Option<u32>)
                   -> Result<Surface<'static>, FontError> {
        let text = if text != "" { text } else { " " };
        
        let font = self.select_font(font_usage);

        if let Some(w) = wrap {
            Ok(font.render(text).blended_wrapped(c, w)?)
        }else{
            Ok(font.render(text.trim_right_matches('\n')).blended(c)?)
        }
    }

    fn select_font(&self, font_usage: FontKind) -> &Font {
        match font_usage {
            FontKind::Log => &self.font_log,
            FontKind::S => &self.font_s,
            FontKind::M => &self.font_m,
            FontKind::MonoM => &self.font_mono_m,
        }
    }
}

fn font_path(fontname: &str) -> PathBuf {
    let mut path = abs_path("fonts");
    path.push(fontname);
    path
}

