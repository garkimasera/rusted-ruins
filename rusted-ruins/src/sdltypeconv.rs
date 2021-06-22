use crate::config::{CfgColor, CfgPos, CfgRect, SCREEN_CFG};
use sdl2::pixels::Color;
use sdl2::rect::Rect;

/// Centering for main window
pub const CENTERING_POS: i32 = -999;
/// Centering for the screen
pub const CENTERING_POS_FOR_SCREEN: i32 = -1000;

impl From<CfgRect> for Rect {
    fn from(value: CfgRect) -> Rect {
        let x = if value.x == CENTERING_POS {
            SCREEN_CFG.main_window.x + (SCREEN_CFG.main_window.w - value.w) as i32 / 2
        } else if value.x == CENTERING_POS_FOR_SCREEN {
            (SCREEN_CFG.screen_w - value.w) as i32 / 2
        } else {
            value.x
        };
        let y = if value.y == CENTERING_POS {
            SCREEN_CFG.main_window.y + (SCREEN_CFG.main_window.h - value.h) as i32 / 2
        } else if value.y == CENTERING_POS_FOR_SCREEN {
            (SCREEN_CFG.screen_h - value.h) as i32 / 2
        } else {
            value.y
        };
        Rect::new(x, y, value.w, value.h)
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
