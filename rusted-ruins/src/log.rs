
use std::sync::Mutex;
use std::collections::VecDeque;

pub fn init() {
    ::lazy_static::initialize(&GAME_LOG);
}

lazy_static! {
    static ref GAME_LOG: Mutex<GameLog> = Mutex::new(GameLog::new());
    static ref LOG_MAX_LINE: usize = 30;
}

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

    fn update(&mut self) {
        if self.buf.is_empty() { return; }

        let b = ::std::mem::replace(&mut self.buf, Vec::new());
        self.lines.push_back(b);
        if self.lines.len() > *LOG_MAX_LINE {
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
        }else{
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

const GAME_LOG_LOCK_ERR: &'static str = "Game log lock error";

pub fn push(s: String) {
    let mut gamelog = GAME_LOG.try_lock().expect(GAME_LOG_LOCK_ERR);
    gamelog.push(s);
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

macro_rules! game_log {
    ($textid:expr) => {
        $crate::log::push(text::get_txt($textid));
    };
    ($textid:expr; $($target:ident = $value:expr),*) => {
        let text_raw = $crate::text::log_txt($textid);
        let mut table: Vec<(&str, String)> = Vec::new();
        $(
            table.push((stringify!($target), $value.to_string()));
        )*;
        
        let t = $crate::util::replace_str(text_raw, table.as_slice());
        $crate::log::push(t);
    }
}

/// Instantly add a new line after logging
macro_rules! game_log_i {
    ($textid:expr) => {
        $crate::log::push(text::get_txt($textid));
        $crate::log::new_line()
    };
    ($textid:expr; $($target:ident = $value:expr),*) => {
        let text_raw = $crate::text::log_txt($textid);
        let mut table: Vec<(&str, String)> = Vec::new();
        $(
            table.push((stringify!($target), $value.to_string()));
        )*;
        
        let t = $crate::util::replace_str(text_raw, table.as_slice());
        $crate::log::push(t);
        $crate::log::new_line();
    }
}

