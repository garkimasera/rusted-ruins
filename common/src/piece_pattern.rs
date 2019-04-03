//! Tile pattern will be defined by surround 8 tiles.
//! TilePatternFlag helps to search appropriate pattern.

use crate::basic::{PIECE_SIZE, PIECE_SIZE_I};
use crate::obj::ImgObject;
use crate::objholder::{ObjectIndex, TileIdx, WallIdx};
use geom::*;
use std::marker::PhantomData;

const INDEX_BIT: u32 = 0b11111111_11111111_11110000_00000000;
const PIECE_PATTERN_BIT: u32 = 0b00000000_00000000_00001111_11111111;

/// Represents 4 pieces pattern of tile images
#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct PiecePattern {
    pub top_left: u8,
    pub top_right: u8,
    pub bottom_left: u8,
    pub bottom_right: u8,
}

impl Default for PiecePattern {
    fn default() -> PiecePattern {
        PiecePattern::SURROUNDED
    }
}

impl PiecePattern {
    pub fn as_int(&self) -> u32 {
        let a = (self.top_left as u32) << 9
            | (self.top_right as u32) << 6
            | (self.bottom_left as u32) << 3
            | self.bottom_right as u32;
        assert!(a <= 0b111111111111);
        a
    }

    pub fn from_int(i: u32) -> PiecePattern {
        PiecePattern {
            top_left: ((i & 0b111000000000) >> 9) as u8,
            top_right: ((i & 0b000111000000) >> 6) as u8,
            bottom_left: ((i & 0b000000111000) >> 3) as u8,
            bottom_right: (i & 0b000000000111) as u8,
        }
    }

    pub const SURROUNDED: PiecePattern = PiecePattern {
        top_left: 0,
        top_right: 0,
        bottom_left: 0,
        bottom_right: 0,
    };
}

pub const EMPTY_PIECE: u8 = 0b111;

/// TileIdx or WallIdx with piece pattern
/// This is 32bit integer. Upper 20bit is for index. Lower 12 bit is for piece pattern.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize)]
#[serde(transparent)]
pub struct IdxWithPiecePattern<T> {
    i: u32,
    #[serde(skip)]
    _p: PhantomData<T>,
}

impl<T> IdxWithPiecePattern<T> {
    pub fn is_empty(&self) -> bool {
        self.i == 0
    }

    pub fn piece_pattern(&self) -> PiecePattern {
        PiecePattern::from_int(self.i)
    }

    pub fn set_piece_pattern(&mut self, piece_pattern: PiecePattern) {
        self.i = (self.i & INDEX_BIT) | piece_pattern.as_int();
    }

    pub(crate) fn as_raw_int(&self) -> u32 {
        self.i >> 12
    }

    pub(crate) fn from_raw_int(i: u32) -> Self {
        IdxWithPiecePattern {
            i: i << 12,
            _p: PhantomData,
        }
    }
}

impl<T> IdxWithPiecePattern<T>
where
    T: ObjectIndex,
{
    pub fn new(idx: T) -> Self {
        IdxWithPiecePattern {
            i: idx.as_raw_int() << 12,
            _p: PhantomData,
        }
    }

    pub fn empty() -> Self {
        IdxWithPiecePattern {
            i: 0,
            _p: PhantomData,
        }
    }

    pub fn with_piece_pattern(idx: T, pp: PiecePattern) -> Self {
        IdxWithPiecePattern {
            i: idx.as_raw_int() << 12 | pp.as_int(),
            _p: PhantomData,
        }
    }

    pub fn idx(&self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            T::from_raw_int(self.i >> 12)
        }
    }

    pub fn set_idx(&mut self, idx: T) {
        let pp = self.i & PIECE_PATTERN_BIT;
        self.i = pp | (idx.as_raw_int() << 12);
    }

    pub fn get(&self) -> Option<(T, PiecePattern)> {
        self.idx().map(|idx| (idx, self.piece_pattern()))
    }
}

impl<T> Default for IdxWithPiecePattern<T>
where
    T: Default,
{
    fn default() -> IdxWithPiecePattern<T> {
        IdxWithPiecePattern {
            i: 0,
            _p: PhantomData,
        }
    }
}

pub type TileIdxPP = IdxWithPiecePattern<TileIdx>;
pub type WallIdxPP = IdxWithPiecePattern<WallIdx>;
pub type ConvertedIdxPP = IdxWithPiecePattern<u32>;

macro_rules! impl_deserialize_for_idxpp {
    ($idxpp:ident, $idx:ident, $mem:ident) => {
        impl<'de> serde::Deserialize<'de> for $idxpp {
            fn deserialize<D>(deserializer: D) -> Result<$idxpp, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let lock = crate::idx_conv::IDX_CONV_TABLE
                    .read()
                    .expect("IDX_CONV_TABLE lock error");
                let i = u32::deserialize(deserializer)?;
                if i == 0 {
                    return Ok($idxpp::empty());
                }

                let pp = i & PIECE_PATTERN_BIT;
                let i = if let Some(idx_conv_table) = lock.as_ref() {
                    // TODO: Should generate error if None
                    let idx = $idx::from_raw_int((i & INDEX_BIT) >> 12).unwrap();
                    let new_idx = idx_conv_table.$mem(idx);
                    (new_idx.as_raw_int() << 12) | pp
                } else {
                    i
                };
                Ok(IdxWithPiecePattern { i, _p: PhantomData })
            }
        }
    };
}

impl_deserialize_for_idxpp!(TileIdxPP, TileIdx, tile);
impl_deserialize_for_idxpp!(WallIdxPP, WallIdx, wall);

impl<'de> serde::Deserialize<'de> for ConvertedIdxPP {
    fn deserialize<D>(deserializer: D) -> Result<ConvertedIdxPP, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let i = u32::deserialize(deserializer)?;
        Ok(IdxWithPiecePattern { i, _p: PhantomData })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PiecePatternFlags(pub u8);

impl PiecePatternFlags {
    pub fn new() -> PiecePatternFlags {
        PiecePatternFlags(0)
    }

    pub fn set(&mut self, dir: Direction, is_same_tile: bool) {
        let flag = match dir {
            Direction::N => Self::N,
            Direction::S => Self::S,
            Direction::E => Self::E,
            Direction::W => Self::W,
            Direction::NE => Self::NE,
            Direction::NW => Self::NW,
            Direction::SE => Self::SE,
            Direction::SW => Self::SW,
            Direction::NONE => {
                return;
            }
        };
        if is_same_tile {
            self.0 |= flag;
        } else {
            self.0 &= !flag;
        }
    }

    pub fn to_piece_pattern(self, n_pattern: u32) -> PiecePattern {
        PiecePattern {
            top_left: get_corner_piece_pattern(
                n_pattern,
                self.0 & Self::N,
                self.0 & Self::NW,
                self.0 & Self::W,
            ),
            top_right: get_corner_piece_pattern(
                n_pattern,
                self.0 & Self::N,
                self.0 & Self::NE,
                self.0 & Self::E,
            ),
            bottom_left: get_corner_piece_pattern(
                n_pattern,
                self.0 & Self::S,
                self.0 & Self::SW,
                self.0 & Self::W,
            ),
            bottom_right: get_corner_piece_pattern(
                n_pattern,
                self.0 & Self::S,
                self.0 & Self::SE,
                self.0 & Self::E,
            ),
        }
    }

    const E: u8 = 0b00000001;
    const SE: u8 = 0b00000010;
    const S: u8 = 0b00000100;
    const SW: u8 = 0b00001000;
    const W: u8 = 0b00010000;
    const NW: u8 = 0b00100000;
    const N: u8 = 0b01000000;
    const NE: u8 = 0b10000000;
}

fn get_corner_piece_pattern(n_pattern: u32, ns: u8, between: u8, ew: u8) -> u8 {
    let ns = ns != 0;
    let between = between != 0;
    let ew = ew != 0;

    match n_pattern {
        1 => {
            if (ns || ew) && between {
                0
            } else {
                EMPTY_PIECE
            }
        }
        5 => match (ns, between, ew) {
            (false, false, false) => 3,
            (false, false, true) => 1,
            (false, true, false) => 3,
            (false, true, true) => 1,
            (true, false, false) => 2,
            (true, false, true) => 4,
            (true, true, false) => 2,
            (true, true, true) => 0,
        },
        _ => 0,
    }
}

pub trait PieceImgObject: ImgObject {
    /// Get rect of the specified piece
    fn piece_rect(
        &self,
        i_pattern: u8,
        i_piece: u32,
        i_anim_frame: u32,
    ) -> Option<(i32, i32, u32, u32)> {
        let img = self.get_img();
        let n_anim_frame = img.n_anim_frame;
        let img_rect = self.img_rect_nth(n_anim_frame * i_pattern as u32 + i_anim_frame);

        if i_pattern == EMPTY_PIECE {
            return None;
        }

        match i_piece {
            0 => Some((
                // Top left piece
                img_rect.0, img_rect.1, PIECE_SIZE, PIECE_SIZE,
            )),
            1 => Some((
                // Top right piece
                img_rect.0 + PIECE_SIZE_I,
                img_rect.1,
                PIECE_SIZE,
                PIECE_SIZE,
            )),
            2 => Some((
                // Bottom left piece
                img_rect.0,
                img_rect.1 + PIECE_SIZE_I,
                PIECE_SIZE,
                img_rect.3 - PIECE_SIZE,
            )),
            3 => Some((
                // Bottom right piece
                img_rect.0 + PIECE_SIZE_I,
                img_rect.1 + PIECE_SIZE_I,
                PIECE_SIZE,
                img_rect.3 - PIECE_SIZE,
            )),
            _ => {
                warn!("unknown piece index {}", i_piece);
                None
            }
        }
    }
}

impl PieceImgObject for crate::obj::TileObject {}
impl PieceImgObject for crate::obj::WallObject {}
impl PieceImgObject for crate::obj::EffectObject {}
