
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
}

impl Animation {
    pub fn get_n_frame(&self) -> u32 {
        match self {
            &Animation::PlayerMove { n_frame, .. } => n_frame,
            &Animation::Img { n_frame, .. } => n_frame,
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
}

