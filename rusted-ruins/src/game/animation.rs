
use array2d::*;
use common::objholder::AnimImgIdx;
use common::gobj;

#[derive(Clone, Copy)]
pub enum Animation {
    PlayerMove {
        n_frame: u32,
        dir: Direction,
    },
    Img {
        n_frame: u32,
        idx: AnimImgIdx,
        range: RectIter,
    },
    Shot {
        n_frame: u32,
        idx: AnimImgIdx,
        dir: (f32, f32),
        start: Vec2d,
        target: Vec2d,
    },
}

impl Animation {
    pub fn get_n_frame(&self) -> u32 {
        match self {
            &Animation::PlayerMove { n_frame, .. } => n_frame,
            &Animation::Img { n_frame, .. } => n_frame,
            &Animation::Shot { n_frame, .. } => n_frame,
        }
    }
    
    pub fn player_move(dir: Direction) -> Animation {
        Animation::PlayerMove {
            n_frame: 6,
            dir: dir
        }
    }

    pub fn img_onetile(idx: AnimImgIdx, p: Vec2d) -> Animation {
        Animation::Img {
            n_frame: gobj::get_obj(idx).img.n_frame,
            idx: idx,
            range: RectIter::one(p),
        }
    }

    pub fn shot(idx: AnimImgIdx, start: Vec2d, target: Vec2d) -> Animation {
        let dx = (target.0 - start.0) as f32;
        let dy = (target.1 - start.1) as f32;
        let d = (dx * dx + dy * dy).sqrt();
        let dir = (dx / d, dy / d);

        const SHOT_ANIM_WIDTH: f32 = 1.0;
        let n_frame = (d / SHOT_ANIM_WIDTH).ceil() as u32;
        
        Animation::Shot {
            n_frame: n_frame,
            idx: idx,
            dir: dir,
            start: start,
            target: target,
        }
    }
}

