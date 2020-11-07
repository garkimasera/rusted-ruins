use super::animation::*;
use common::gamedata::Effect;
use common::gobj;
use common::objholder::AnimImgIdx;
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

    pub fn push_effect(&mut self, effect: &Effect, tile: Vec2d, start: Option<Vec2d>) {
        if !effect.anim_img_shot.is_empty() {
            if let Some(start) = start {
                if let Some(idx) = gobj::id_to_idx_checked::<AnimImgIdx>(&effect.anim_img_shot) {
                    self.push(Animation::shot(idx, start, tile));
                } else {
                    warn!("unknown AnimImgObject: {}", effect.anim_img_shot);
                }
            }
        }
        if !effect.anim_img.is_empty() {
            let idx: AnimImgIdx = gobj::id_to_idx(&effect.anim_img);
            self.push(Animation::img_onetile(idx, tile));
        }
    }

    pub fn push_destroy(&mut self, tile: Vec2d) {
        self.push(Animation::destroy(vec![tile]));
    }

    pub fn push_work(&mut self, ratio: f32) {
        self.push(Animation::Work { n_frame: 2, ratio });
    }
}
