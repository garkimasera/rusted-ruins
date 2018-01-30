//! This module processes the view of characters

use array2d::Array2d;
use game::Game;

/// The cache for determining player's view
pub struct ViewMap {
    visible: Array2d<bool>,
}

impl ViewMap {
    pub fn new() -> ViewMap {
        ViewMap {
            visible: Array2d::new(128, 128, false),
        }
    }

    fn reserve_size(&mut self, w: u32, h: u32) {
        let size = self.visible.size();
        if size.0 >= w || size.1 >= h {
            use std::cmp::max;
            self.visible = Array2d::new(max(size.0, w), max(size.1, h), false);
        }
    }
}

pub fn update_view_map(game: &mut Game) {
    
}

