//! Helper functions to calculate Window position
#![allow(unused)]

use crate::config::SCREEN_CFG;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum WindowHPos {
    Center, LeftMargin(i32), RightMargin(i32), RightX(i32),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum WindowVPos {
    Center, TopMargin(i32), BottomMargin(i32),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct WindowPos {
    pub h: WindowHPos,
    pub v: WindowVPos
}

impl WindowPos {
    pub fn new(h: WindowHPos, v: WindowVPos) -> WindowPos {
        WindowPos { h, v }
    }

    pub fn calc_left_top(&self, w: u32, h: u32) -> (i32, i32) {
        let parent_w = SCREEN_CFG.screen_w as i32;
        let parent_h = SCREEN_CFG.screen_h as i32;
        let w = w as i32;
        let h = h as i32;
        
        let x = match self.h {
            WindowHPos::Center => {
                (parent_w - w) as i32 / 2
            }
            WindowHPos::LeftMargin(m) => {
                m
            }
            WindowHPos::RightMargin(m) => {
                parent_w - w - m
            }
            WindowHPos::RightX(x) => {
                x - w
            }
        };
        let y = match self.v {
            WindowVPos::Center => {
                (parent_h - h) as i32 / 2
            }
            WindowVPos::TopMargin(m) => {
                m
            }
            WindowVPos::BottomMargin(m) => {
                parent_h - h - m
            }
        };
        (x, y)
    }
}

