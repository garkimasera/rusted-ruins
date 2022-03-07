use once_cell::sync::Lazy;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

use super::textrenderer::TextRenderer;
use super::textrenderer::{FontKind, SurfaceConf};
use super::textrenderer::{ERR_MSG_FONT_REND, ERR_MSG_FONT_TEX};
use super::Context;

use std::sync::Mutex;

static CACHE_DROP_STACK: Lazy<Mutex<Vec<usize>>> = Lazy::new(|| Mutex::new(Vec::new()));

const CACHE_DROP_STACK_LOCK_ERR_MSG: &str = "Cache drop stack lock error";
fn push_drop_stack(i: usize) {
    let mut a = CACHE_DROP_STACK
        .lock()
        .expect(CACHE_DROP_STACK_LOCK_ERR_MSG);
    a.push(i);
}

fn pop_drop_stack() -> Option<usize> {
    let mut a = CACHE_DROP_STACK
        .lock()
        .expect(CACHE_DROP_STACK_LOCK_ERR_MSG);
    a.pop()
}

pub struct TextCache {
    i: Option<usize>,
    pub s: Vec<String>,
    pub font: FontKind,
    pub color: Color,
    pub conf: SurfaceConf,
}

impl TextCache {
    pub fn group<S: AsRef<str>>(s: &[S], font: FontKind, color: Color) -> TextCache {
        let s: Vec<String> = s.iter().map(|a| a.as_ref().to_string()).collect();

        TextCache {
            i: None,
            s,
            font,
            color,
            conf: SurfaceConf::default(),
        }
    }

    pub fn new<S: Into<String>, C: Into<Color>>(s: S, font: FontKind, color: C) -> TextCache {
        TextCache {
            i: None,
            s: vec![s.into()],
            font,
            color: color.into(),
            conf: SurfaceConf {
                image_inline: true,
                ..SurfaceConf::default()
            },
        }
    }

    pub fn wrapped<S: Into<String>, C: Into<Color>>(
        s: S,
        font: FontKind,
        color: C,
        w: u32,
    ) -> TextCache {
        TextCache {
            i: None,
            s: vec![s.into()],
            font,
            color: color.into(),
            conf: SurfaceConf {
                wrapped: Some(w),
                ..SurfaceConf::default()
            },
        }
    }

    pub fn bordered<S: Into<String>, C: Into<Color>>(s: S, font: FontKind, color: C) -> TextCache {
        TextCache {
            i: None,
            s: vec![s.into()],
            font,
            color: color.into(),
            conf: SurfaceConf {
                bordered: true,
                ..SurfaceConf::default()
            },
        }
    }

    pub fn render_at(
        &mut self,
        context: &mut Context<'_, '_, '_, '_>,
        x: i32,
        y: i32,
        top: bool,
        left: bool,
    ) {
        let canvas = &mut context.canvas;
        let sv = &mut context.sv;
        let tex = sv.tt_one(self);

        let w = tex.query().width;
        let h = tex.query().height;
        let dest = Rect::new(
            x - if top { 0 } else { w as i32 },
            y - if left { 0 } else { h as i32 },
            w,
            h,
        );
        try_sdl!(canvas.copy(tex, None, dest));
    }

    // pub fn get_str(&self, i: usize) -> &str {
    //     &self.s[i]
    // }
}

impl Drop for TextCache {
    fn drop(&mut self) {
        if let Some(i) = self.i {
            push_drop_stack(i);
        }
    }
}

const DEFAULT_CACHE_SIZE: usize = 256;

/// Stores Texture
/// When ui rendering, Textcache is passed to this pool,
/// and it creates texture.
pub struct TextCachePool<'t> {
    data: Vec<Option<Vec<Texture<'t>>>>,
}

impl<'t> TextCachePool<'t> {
    pub fn new() -> TextCachePool<'t> {
        let mut data = Vec::with_capacity(DEFAULT_CACHE_SIZE);
        for _ in 0..DEFAULT_CACHE_SIZE {
            data.push(None);
        }

        TextCachePool { data }
    }

    fn append(&mut self, t: Vec<Texture<'t>>) -> usize {
        self.gc();

        for (i, d) in self.data.iter_mut().enumerate() {
            if d.is_none() {
                *d = Some(t);
                return i;
            }
        }
        let i = self.data.len();
        self.data.push(Some(t));
        i
    }

    pub fn group(
        &mut self,
        c: &mut TextCache,
        tr: &TextRenderer<'_>,
        tc: &'t TextureCreator<WindowContext>,
    ) -> &[Texture<'_>] {
        if c.i.is_none() {
            // Render and add cache
            let mut v = Vec::new();
            for s in c.s.iter() {
                let surface = tr
                    .surface(c.font, s, c.color, c.conf)
                    .expect(ERR_MSG_FONT_REND);
                let t = tc
                    .create_texture_from_surface(surface)
                    .expect(ERR_MSG_FONT_TEX);
                v.push(t);
            }
            c.i = Some(self.append(v));
        }

        self.data[c.i.unwrap()]
            .as_ref()
            .expect("Requested cache doesn't exist")
    }

    pub fn gc(&mut self) {
        while let Some(i) = pop_drop_stack() {
            self.data[i] = None;
        }
    }
}
