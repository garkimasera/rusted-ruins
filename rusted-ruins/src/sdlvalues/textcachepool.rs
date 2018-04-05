
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;
use sdl2::pixels::Color;

use super::TextRenderer;
use super::textrenderer::FontKind;
use super::textrenderer::{ERR_MSG_FONT_REND, ERR_MSG_FONT_TEX};

use std::sync::Mutex;

lazy_static! {
    static ref CACHE_DROP_STACK: Mutex<Vec<usize>> = Mutex::new(Vec::new());
}

const CACHE_DROP_STACK_LOCK_ERR_MSG: &'static str = "Cache drop stack lock error";
fn push_drop_stack(i: usize) {
    let mut a = CACHE_DROP_STACK.lock().expect(CACHE_DROP_STACK_LOCK_ERR_MSG);
    a.push(i);
}

fn pop_drop_stack() -> Option<usize> {
    let mut a = CACHE_DROP_STACK.lock().expect(CACHE_DROP_STACK_LOCK_ERR_MSG);
    a.pop()
}

pub struct TextCache {
    i: Option<usize>,
    pub s: Vec<String>,
    pub font: FontKind,
    pub color: Color,
    pub wrap_size: Option<u32>,
    pub is_bordered: bool,
}

impl TextCache {
    pub fn new<S: AsRef<str>>(s: &[S], font: FontKind, color: Color) -> TextCache {
        let s: Vec<String> = s.iter().map(|a| a.as_ref().to_string()).collect();
        
        TextCache {
            i: None,
            s: s,
            font: font, color: color, wrap_size: None, is_bordered: false,
        }
    }

    pub fn one<S: Into<String>>(s: S, font: FontKind, color: Color) -> TextCache {
        TextCache {
            i: None,
            s: vec![s.into()],
            font: font, color: color, wrap_size: None, is_bordered: false,
        }
    }

    pub fn one_wrapped<S: Into<String>>(s: S, font: FontKind, color: Color, w: u32) -> TextCache {
        TextCache {
            i: None,
            s: vec![s.into()],
            font: font, color: color, wrap_size: Some(w), is_bordered: false,
        }
    }

    pub fn one_bordered<S: Into<String>>(s: S, font: FontKind, color: Color) -> TextCache {
        TextCache {
            i: None,
            s: vec![s.into()],
            font: font, color: color, wrap_size: None, is_bordered: true,
        }
    }

    pub fn get_str(&self, i: usize) -> &str {
        &self.s[i]
    }
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
        
        TextCachePool {
            data: data
        }
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

    pub fn group(&mut self, c: &mut TextCache, tr: &TextRenderer, tc: &'t TextureCreator<WindowContext>)
                 -> &[Texture] {
        
        if c.i.is_none() { // Render and add cache
            let mut v = Vec::new();
            for ref s in c.s.iter() {
                let surface = tr
                    .surface(c.font, s, c.color, c.wrap_size, c.is_bordered)
                    .expect(ERR_MSG_FONT_REND);
                let t = tc.create_texture_from_surface(surface).expect(ERR_MSG_FONT_TEX);
                v.push(t);
            }
            c.i = Some(self.append(v));
        }
        
        self.data[c.i.unwrap()].as_ref().expect("Requested cache doesn't exist")
    }

    pub fn gc(&mut self) {
        while let Some(i) = pop_drop_stack() {
            self.data[i] = None;
        }
    }
}

