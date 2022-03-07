use crate::config::visual::FontConfig;
use crate::config::{abs_path, FONT_CFG, UI_CFG};
use crate::SdlContext;
use common::gobj;
use common::objholder::{ItemIdx, UiImgIdx};
use once_cell::sync::Lazy;
use regex::Regex;
use sdl2::image::ImageRWops;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::rwops::RWops;
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
    /// For tiny UI texts
    T,
    /// For talk or book texts
    Talk,
    /// For monospace texts. This font may include ascii characters only
    MonoM,
}

pub struct TextRenderer<'sdl> {
    font_t: Font<'sdl, 'static>,
    font_s: Font<'sdl, 'static>,
    font_m: Font<'sdl, 'static>,
    font_log: Font<'sdl, 'static>,
    font_talk: Font<'sdl, 'static>,
    font_mono_m: Font<'sdl, 'static>,
}

pub const ERR_MSG_FONT_REND: &str = "Error occured during font rendering to surface";
pub const ERR_MSG_FONT_TEX: &str = "Error occured during texture creation from font surface";

#[derive(Clone, Copy, Default, Debug)]
pub struct SurfaceConf {
    pub bordered: bool,
    pub wrapped: Option<u32>,
    pub image_inline: bool,
}

impl<'sdl> TextRenderer<'sdl> {
    pub fn new(sdl_context: &'sdl SdlContext) -> TextRenderer<'sdl> {
        const ERR_MSG: &str = "font loading error";
        let font_name = FONT_CFG.font_name();
        let font = &UI_CFG.font;
        let f = |fc: &FontConfig| -> Font<'_, '_> {
            sdl_context
                .ttf_context
                .load_font(&font_path(font_name), fc.size)
                .expect(ERR_MSG)
        };

        TextRenderer {
            font_t: f(&font.t),
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
        font_kind: FontKind,
        text: &str,
        c: Color,
        conf: SurfaceConf,
    ) -> Result<Surface<'static>, FontError> {
        let text = if !text.is_empty() { text } else { " " };

        if conf.image_inline && RE_IMG_INLINE.find(text).is_some() {
            return self.text_with_image_to_surface(font_kind, text, c, conf.bordered);
        }

        let font = self.select_font(font_kind);

        let mut surface = if let Some(w) = conf.wrapped {
            font.render(text).blended_wrapped(c, w)?
        } else {
            font.render(text.trim_end_matches('\n')).blended(c)?
        };

        if conf.bordered {
            border_text(&mut surface);
        }

        Ok(surface)
    }

    fn select_font(&self, font_kind: FontKind) -> &Font<'_, '_> {
        match font_kind {
            FontKind::Log => &self.font_log,
            FontKind::T => &self.font_t,
            FontKind::S => &self.font_s,
            FontKind::M => &self.font_m,
            FontKind::Talk => &self.font_talk,
            FontKind::MonoM => &self.font_mono_m,
        }
    }

    pub fn text_with_image_to_surface(
        &self,
        font_kind: FontKind,
        text: &str,
        c: Color,
        bordered: bool,
    ) -> Result<Surface<'static>, FontError> {
        let mut w = 0;
        let mut h = 0;
        let surfaces = self.text_with_image_to_surfaces(font_kind, text, c, bordered)?;
        for surface in &surfaces {
            w += surface.width();
            h = std::cmp::max(h, surface.height());
        }

        let mut target = Surface::new(w, h, sdl2::pixels::PixelFormatEnum::ARGB8888)
            .map_err(FontError::SdlError)?;
        let mut x = 0;
        for mut surface in surfaces.into_iter() {
            surface
                .set_blend_mode(sdl2::render::BlendMode::None)
                .map_err(FontError::SdlError)?;
            let src_rect = Rect::new(0, 0, surface.width(), surface.height());
            let y = (target.height() - surface.height()) / 2;
            let dst_rect = Rect::new(x, y as i32, surface.width(), surface.height());
            let _ = surface.blit(src_rect, &mut target, dst_rect);
            x += surface.width() as i32;
        }

        Ok(target)
    }

    fn text_with_image_to_surfaces(
        &self,
        font_kind: FontKind,
        text: &str,
        c: Color,
        bordered: bool,
    ) -> Result<Vec<Surface<'static>>, FontError> {
        let mut surfaces = Vec::new();

        for block in split_to_block(text).into_iter() {
            let img = match block {
                Block::Text(text) => {
                    let conf = SurfaceConf {
                        bordered,
                        ..SurfaceConf::default()
                    };
                    surfaces.push(self.surface(font_kind, text, c, conf)?);
                    continue;
                }
                Block::UiImg(id) => {
                    let idx: UiImgIdx = if let Some(idx) = gobj::id_to_idx_checked(id) {
                        idx
                    } else {
                        continue;
                    };
                    &gobj::get_obj(idx).img
                }
                Block::Item(id) => {
                    let idx: ItemIdx = if let Some(idx) = gobj::id_to_idx_checked(id) {
                        idx
                    } else {
                        continue;
                    };
                    &gobj::get_obj(idx).img
                }
            };
            let rwops = match RWops::from_bytes(&img.data) {
                Ok(rwops) => rwops,
                Err(e) => {
                    warn!("{}", e);
                    continue;
                }
            };
            let surface = match rwops.load() {
                Ok(surface) => surface,
                Err(e) => {
                    warn!("{}", e);
                    continue;
                }
            };
            surfaces.push(surface);
        }

        Ok(surfaces)
    }
}

fn font_path(fontname: &str) -> PathBuf {
    let mut path = abs_path("fonts");
    path.push(fontname);
    path
}

/// If opacity is larger than this, the pixel is handled as opacity
const OPACITY_BORDER: u8 = 64;

fn border_text(surface: &mut Surface<'_>) {
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

#[derive(PartialEq, Eq, Debug)]
enum Block<'a> {
    Text(&'a str),
    UiImg(&'a str),
    Item(&'a str),
}

static RE_IMG_INLINE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r":(ui_img|item)/([a-zA-Z!][a-zA-Z_./-]+):").unwrap());

fn split_to_block(s: &str) -> Vec<Block<'_>> {
    let mut blocks = Vec::new();
    let mut p = 0;

    for m in RE_IMG_INLINE.find_iter(s) {
        if p < m.start() {
            blocks.push(Block::Text(&s[p..m.start()]));
        }

        let caps = RE_IMG_INLINE.captures(m.as_str()).unwrap();

        let id = caps.get(2).unwrap().as_str();
        let block = match caps.get(1).unwrap().as_str() {
            "ui_img" => Block::UiImg(id),
            "item" => Block::Item(id),
            _ => unreachable!(),
        };
        blocks.push(block);

        p = m.end();
    }

    if p < s.len() {
        blocks.push(Block::Text(&s[p..]));
    }
    blocks
}

#[test]
fn split_to_block_test() {
    assert_eq!(
        split_to_block(":ui_img/!test-icon:"),
        vec![Block::UiImg("!test-icon")],
    );
    assert_eq!(split_to_block("aaa : bbb"), vec![Block::Text("aaa : bbb"),],);
    assert_eq!(
        split_to_block("aaa :ui_img/!test-icon: bbb :item/hoge:"),
        vec![
            Block::Text("aaa "),
            Block::UiImg("!test-icon"),
            Block::Text(" bbb "),
            Block::Item("hoge")
        ],
    );
}
