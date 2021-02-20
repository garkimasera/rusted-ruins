use crate::config::visual::FontConfig;
use crate::config::{abs_path, FONT_CFG, UI_CFG};
use crate::SdlContext;
use sdl2::pixels::Color;
use sdl2::surface::Surface;
use sdl2::ttf::*;
use std::path::PathBuf;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum FontKind {
    /// For log window
    Log,
    /// For most of window texts
    M,
    /// For small UI texts
    S,
    /// For talk or book texts
    Talk,
    /// For monospace texts. This font may include ascii characters only
    MonoM,
}

pub struct TextRenderer<'sdl> {
    font_s: Font<'sdl, 'static>,
    font_m: Font<'sdl, 'static>,
    font_log: Font<'sdl, 'static>,
    font_talk: Font<'sdl, 'static>,
    font_mono_m: Font<'sdl, 'static>,
}

pub const ERR_MSG_FONT_REND: &str = "Error occured during font rendering to surface";
pub const ERR_MSG_FONT_TEX: &str = "Error occured during texture creation from font surface";

impl<'sdl> TextRenderer<'sdl> {
    pub fn new(sdl_context: &'sdl SdlContext) -> TextRenderer<'sdl> {
        const ERR_MSG: &str = "Font loading error";
        let font_name = FONT_CFG.font_name();
        let font = &UI_CFG.font;
        let f = |fc: &FontConfig| -> Font {
            sdl_context
                .ttf_context
                .load_font(&font_path(&font_name), fc.size)
                .expect(ERR_MSG)
        };

        TextRenderer {
            font_s: f(&font.s),
            font_m: f(&font.m),
            font_log: f(&font.log),
            font_talk: f(&font.talk),
            font_mono_m: sdl_context
                .ttf_context
                .load_font(&font_path(&FONT_CFG.mono_font), font.m.size)
                .expect(ERR_MSG),
        }
    }

    pub fn surface(
        &self,
        font_usage: FontKind,
        text: &str,
        c: Color,
        wrap: Option<u32>,
        is_bordered: bool,
    ) -> Result<Surface<'static>, FontError> {
        let text = if !text.is_empty() { text } else { " " };

        let font = self.select_font(font_usage);

        let mut surface = if let Some(w) = wrap {
            font.render(text).blended_wrapped(c, w)?
        } else {
            font.render(text.trim_end_matches('\n')).blended(c)?
        };

        if is_bordered {
            border_text(&mut surface);
        }

        Ok(surface)
    }

    fn select_font(&self, font_usage: FontKind) -> &Font {
        match font_usage {
            FontKind::Log => &self.font_log,
            FontKind::S => &self.font_s,
            FontKind::M => &self.font_m,
            FontKind::Talk => &self.font_talk,
            FontKind::MonoM => &self.font_mono_m,
        }
    }
}

fn font_path(fontname: &str) -> PathBuf {
    let mut path = abs_path("fonts");
    path.push(fontname);
    path
}

/// If opacity is larger than this, the pixel is handled as opacity
const OPACITY_BORDER: u8 = 64;

fn border_text(surface: &mut Surface) {
    use sdl2::pixels::PixelFormatEnum;

    let size = surface.size();
    let (w, h) = (size.0 as i32, size.1 as i32);
    assert!(surface.pixel_format_enum() == PixelFormatEnum::ARGB8888);

    surface.with_lock_mut(|pixel: &mut [u8]| {
        assert!((w * h * 4) as usize == pixel.len());

        let mut opacity: Vec<bool> = Vec::with_capacity((w * h) as usize);
        for y in 0..h {
            for x in 0..w {
                let p = (w * y + x) as usize * 4;
                opacity.push(pixel[p + 3] > OPACITY_BORDER);
            }
        }

        let is_opaque = |x: i32, y: i32| -> bool {
            if x < 0 || x >= w || y < 0 || y >= h {
                false
            } else {
                opacity[(w * y + x) as usize]
            }
        };

        for y in 0..h {
            for x in 0..w {
                if is_opaque(x - 1, y)
                    || is_opaque(x + 1, y)
                    || is_opaque(x, y - 1)
                    || is_opaque(x, y + 1)
                {
                    let p = (w * y + x) as usize * 4;
                    let o = pixel[p + 3] as u32;
                    pixel[p] = (pixel[p] as u32 * o / 0xFF) as u8;
                    pixel[p + 1] = (pixel[p + 1] as u32 * o / 0xFF) as u8;
                    pixel[p + 2] = (pixel[p + 2] as u32 * o / 0xFF) as u8;
                    pixel[p + 3] = 255;
                }
            }
        }
    });
}
