use crate::config::{CfgColor, CfgPos, CfgRect, SCREEN_CFG};
use sdl2::pixels::Color;
use sdl2::rect::Rect;

/// Centering for main window
pub const CENTERING_POS: i32 = -999;
/// Centering for the screen
pub const CENTERING_POS_FOR_SCREEN: i32 = -1000;

impl Into<Rect> for CfgRect {
    fn into(self) -> Rect {
        let x = if self.x == CENTERING_POS {
            SCREEN_CFG.main_window.x + (SCREEN_CFG.main_window.w - self.w) as i32 / 2
        } else if self.x == CENTERING_POS_FOR_SCREEN {
            (SCREEN_CFG.screen_w - self.w) as i32 / 2
        } else {
            self.x
        };
        let y = if self.y == CENTERING_POS {
            SCREEN_CFG.main_window.y + (SCREEN_CFG.main_window.h - self.h) as i32 / 2
        } else if self.y == CENTERING_POS_FOR_SCREEN {
            (SCREEN_CFG.screen_h - self.h) as i32 / 2
        } else {
            self.y
        };
        Rect::new(x, y, self.w, self.h)
    }
}

impl From<CfgPos> for (i32, i32) {
    fn from(c: CfgPos) -> Self {
        (c.x, c.y)
    }
}

impl From<CfgColor> for Color {
    fn from(c: CfgColor) -> Self {
        if let Some(a) = c.a {
            Color::RGBA(c.r, c.g, c.b, a)
        } else {
            Color::RGB(c.r, c.g, c.b)
        }
    }
}
