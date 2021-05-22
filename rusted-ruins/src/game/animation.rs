use common::gobj;
use common::objholder::AnimImgIdx;
use geom::*;

#[derive(Clone)]
pub enum Animation {
    PlayerMove {
        n_frame: u32,
        dir: Direction,
    },
    Img {
        n_frame: u32,
        idx: AnimImgIdx,
        tiles: Vec<Vec2d>,
    },
    Shot {
        n_frame: u32,
        n_image: u32,
        idx: AnimImgIdx,
        dir: (f32, f32),
        start: Vec2d,
        target: Vec2d,
    },
    Destroy {
        n_frame: u32,
        idx: AnimImgIdx,
        tiles: Vec<Vec2d>,
    },
    Work {
        n_frame: u32,
        ratio: f32,
    },
}

impl Animation {
    pub fn get_n_frame(&self) -> u32 {
        match self {
            Animation::PlayerMove { n_frame, .. } => *n_frame,
            Animation::Img { n_frame, .. } => *n_frame,
            Animation::Shot { n_frame, .. } => *n_frame,
            Animation::Destroy { n_frame, .. } => *n_frame,
            Animation::Work { n_frame, .. } => *n_frame,
        }
    }

    pub fn player_move(dir: Direction) -> Animation {
        Animation::PlayerMove { n_frame: 6, dir }
    }

    pub fn img_tiles(idx: AnimImgIdx, tiles: Vec<Vec2d>) -> Animation {
        let img = &gobj::get_obj(idx).img;
        let n_frame = if img.duration != 0 {
            img.duration
        } else {
            img.n_frame
        };
        Animation::Img {
            n_frame,
            idx,
            tiles,
        }
    }

    pub fn img_onetile(idx: AnimImgIdx, pos: Vec2d) -> Animation {
        Self::img_tiles(idx, vec![pos])
    }

    pub fn shot(idx: AnimImgIdx, start: Vec2d, target: Vec2d) -> Animation {
        let dx = (target.0 - start.0) as f32;
        let dy = (target.1 - start.1) as f32;
        let d = (dx * dx + dy * dy).sqrt();
        let dir = (dx / d, dy / d);

        const SHOT_ANIM_WIDTH: f32 = 1.0;
        let n_frame = (d / SHOT_ANIM_WIDTH).ceil() as u32;
        let anim_obj = gobj::get_obj(idx);
        let n_image = match anim_obj.img.n_frame {
            8 => calc_arrow_dir(dir),
            _ => 1,
        };

        Animation::Shot {
            n_frame,
            n_image,
            idx,
            dir,
            start,
            target,
        }
    }

    pub fn destroy(tiles: Vec<Vec2d>) -> Animation {
        let idx: AnimImgIdx = gobj::id_to_idx("!destroy-blood");
        let animobj = gobj::get_obj(idx);

        Animation::Destroy {
            n_frame: animobj.img.n_anim_frame,
            idx,
            tiles,
        }
    }
}

/// Calculates image number of arrow animation from dir vector
fn calc_arrow_dir(dir: (f32, f32)) -> u32 {
    use std::f32::consts::PI;
    let degree = dir.0.acos();
    const PI8: f32 = PI / 8.0;

    if dir.1 > 0.0 {
        if degree < PI8 {
            0
        } else if degree < PI8 * 3.0 {
            1
        } else if degree < PI8 * 5.0 {
            2
        } else if degree < PI8 * 7.0 {
            3
        } else {
            4
        }
    } else if degree < PI8 {
        0
    } else if degree < PI8 * 3.0 {
        7
    } else if degree < PI8 * 5.0 {
        6
    } else if degree < PI8 * 7.0 {
        5
    } else {
        4
    }
}
