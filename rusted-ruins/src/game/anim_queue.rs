
use std::collections::VecDeque;
use crate::array2d::*;
use common::gobj;
use common::objholder::AnimImgIdx;
use super::animation::*;

/// Wraps Animation queue, and provides helper functions to push Animations
#[derive(Default)]
pub struct AnimQueue(VecDeque<Animation>);

impl AnimQueue {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    
    pub fn pop(&mut self) -> Option<Animation> {
        self.0.pop_front()
    }

    fn push(&mut self, animation: Animation) {
        self.0.push_back(animation);
    }
    
    pub fn push_player_move(&mut self, dir: Direction) {
        self.push(Animation::player_move(dir));
    }

    pub fn push_attack(&mut self, tile: Vec2d) {
        let idx: AnimImgIdx = gobj::id_to_idx("!damage-blunt");
        self.push(Animation::img_onetile(idx, tile));
    }

    pub fn push_shot(&mut self, start: Vec2d, target: Vec2d) {
        let idx: AnimImgIdx = gobj::id_to_idx("!arrow");
        self.push(Animation::shot(idx, start, target));
    }
}

