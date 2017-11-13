
use std::collections::VecDeque;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;
use game::{Game, Animation};
use sdlvalues::*;
use window::Window;
use config::{SCREEN_CFG, UI_CFG};
use log;

pub struct LogWindow {
    rect: Rect,
    line_cache: LineCache,
}

impl LogWindow {
    pub fn new() -> LogWindow {
        LogWindow {
            rect: SCREEN_CFG.log_window.into(),
            line_cache: LineCache::new(),
        }
    }
}

impl Window for LogWindow {
    
    fn redraw(
        &mut self, canvas: &mut WindowCanvas, _game: &Game, sv: &mut SdlValues,
        _anim: Option<(&Animation, u32)>) {
        
        canvas.set_viewport(None);
        canvas.set_draw_color(UI_CFG.color.log_window_bg.into());
        check_draw!(canvas.fill_rect(self.rect));
        canvas.set_viewport(self.rect);

        self.line_cache.update();

        let end = self.line_cache.lines.len();
        let n_display_line = UI_CFG.log_window.n_display_line as usize;
        let start = if end > n_display_line {
            end - n_display_line
        }else{
            0
        };
        let dy = UI_CFG.log_window.h;

        for (i, line) in (start..end).enumerate() {
            let line_texs = sv.tt_group(&mut self.line_cache.lines[line]);
            let mut x = 0;
            for t in line_texs {
                let w = t.query().width;
                let h = t.query().height;
                let orig = Rect::new(0, 0, w, h);
                let dest = Rect::new(x, dy * i as i32, w, h);
                check_draw!(canvas.copy(t, orig, dest));
                x += w as i32;
            }
        }
    }
}

/// Stores TextCache for log rendering
struct LineCache {
    lines: VecDeque<TextCache>,
    latest_line: usize,
}

impl LineCache {
    fn new() -> LineCache {
        LineCache {
            lines: VecDeque::new(),
            latest_line: 0,
        }
    }

    /// Update from log data
    fn update(&mut self) {
        log::with_lines(self.latest_line, |s| {
            self.append(s);
        });
        self.latest_line = log::latest_line()
    }

    /// Append one line
    fn append(&mut self, s: &Vec<String>) {
        let t = TextCache::new(s, FontKind::Log, Color::RGB(255, 255, 255));
        self.lines.push_back(t);

        if self.lines.len() > 20 {
            self.lines.pop_front();
        }
    }
}

