use super::animation::*;
use geom::*;
use std::collections::VecDeque;

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

    pub fn push(&mut self, animation: Animation) {
        self.0.push_back(animation);
    }

    pub fn push_player_move(&mut self, dir: Direction) {
        self.push(Animation::player_move(dir));
    }

    pub fn push_destroy(&mut self, tile: Coords) {
        self.push(Animation::destroy(vec![tile]));
    }

    pub fn push_work(&mut self, ratio: f32) {
        self.push(Animation::Work { n_frame: 2, ratio });
    }
}
