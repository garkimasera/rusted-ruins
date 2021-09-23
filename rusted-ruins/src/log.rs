use once_cell::sync::Lazy;
use std::collections::VecDeque;
use std::sync::Mutex;

pub fn init() {
    Lazy::force(&GAME_LOG);
}

static GAME_LOG: Lazy<Mutex<GameLog>> = Lazy::new(|| Mutex::new(GameLog::new()));
static LOG_MAX_LINE: usize = 30;

pub struct GameLog {
    lines: VecDeque<Vec<String>>,
    buf: Vec<String>,
    line_count: usize,
}

impl GameLog {
    fn new() -> GameLog {
        GameLog {
            lines: VecDeque::new(),
            buf: Vec::new(),
            line_count: 0,
        }
    }

    fn push(&mut self, s: String) {
        self.buf.push(s);
    }

    fn clear(&mut self) {
        self.lines.clear();
        self.buf.clear();
        self.line_count = 0;
    }

    fn update(&mut self) {
        if self.buf.is_empty() {
            return;
        }

        let b = std::mem::take(&mut self.buf);
        self.lines.push_back(b);
        if self.lines.len() > LOG_MAX_LINE {
            let _ = self.lines.pop_front();
        }
        self.line_count += 1;
    }

    fn with_lines<F: FnMut(&Vec<String>)>(&mut self, from: usize, mut f: F) -> bool {
        let someline_lost;
        let diff = self.line_count - from;
        let start = if self.lines.len() > diff {
            someline_lost = false;
            self.lines.len() - diff
        } else {
            someline_lost = true;
            0
        };

        let len = self.lines.len();
        for i in start..len {
            f(&self.lines[i]);
        }

        someline_lost
    }
}

const GAME_LOG_LOCK_ERR: &str = "Game log lock error";

pub fn push(s: String) {
    let mut gamelog = GAME_LOG.try_lock().expect(GAME_LOG_LOCK_ERR);
    gamelog.push(s);
}

pub fn clear() {
    let mut gamelog = GAME_LOG.try_lock().expect(GAME_LOG_LOCK_ERR);
    gamelog.clear();
}

pub fn with_lines<F: FnMut(&Vec<String>)>(from: usize, f: F) {
    let mut gamelog = GAME_LOG.try_lock().expect(GAME_LOG_LOCK_ERR);
    gamelog.with_lines(from, f);
}

pub fn new_line() {
    let mut gamelog = GAME_LOG.try_lock().expect(GAME_LOG_LOCK_ERR);
    gamelog.update();
}

pub fn latest_line() -> usize {
    let gamelog = GAME_LOG.try_lock().expect(GAME_LOG_LOCK_ERR);
    gamelog.line_count
}

macro_rules! game_log_i {
    ($id:expr) => {
        $crate::log::push($crate::text::log_txt($id));
    };
    ($id:expr; $($target:ident = $value:expr),*) => {{
        use crate::text::ToText;
        let mut table = fluent::FluentArgs::new();
        $(
            let value = fluent::FluentValue::String($value.to_text());
            table.add(stringify!($target), value);
        )*

        let s = crate::text::log_txt_with_args($id, Some(&table));
        crate::log::push(s);
    }}
}

/// Instantly add a new line after logging
macro_rules! game_log {
    ($id:expr) => {
        $crate::log::push($crate::text::log_txt($id));
        $crate::log::new_line()
    };
    ($id:expr; $($target:ident = $value:expr),*) => {{
        use crate::text::ToText;
        let mut table = fluent::FluentArgs::new();
        $(
            let value = fluent::FluentValue::String($value.to_text());
            table.add(stringify!($target), value);
        )*

        let s = crate::text::log_txt_with_args($id, Some(&table));
        crate::log::push(s);
        crate::log::new_line();
    }}
}
