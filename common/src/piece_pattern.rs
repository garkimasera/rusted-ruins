//! Tile pattern will be defined by surround 8 tiles.
//! TilePatternFlag helps to search appropriate pattern.

use array2d::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PiecePatternFlags(pub u8);

impl PiecePatternFlags {
    pub fn new() -> PiecePatternFlags {
        PiecePatternFlags(0)
    }

    pub fn set(&mut self, dir: Direction, is_same_tile: bool) {
        let flag = match dir {
            Direction::N  => Self::N,
            Direction::S  => Self::S,
            Direction::E  => Self::E,
            Direction::W  => Self::W,
            Direction::NE => Self::NE,
            Direction::NW => Self::NW,
            Direction::SE => Self::SE,
            Direction::SW => Self::SW,
            Direction::NONE => { return; }
        };
        if is_same_tile {
            self.0 |= flag;
        } else {
            self.0 &= !flag;
        }
    }

    pub fn to_piece_pattern(self) -> [u8; 4] {
        [
            get_corner_piece_pattern( // Top left
                self.0 & Self::N,
                self.0 & Self::NW,
                self.0 & Self::W),
            get_corner_piece_pattern( // Top right
                self.0 & Self::N,
                self.0 & Self::NE,
                self.0 & Self::E),
            get_corner_piece_pattern( // Bottom left
                self.0 & Self::S,
                self.0 & Self::SW,
                self.0 & Self::W),
            get_corner_piece_pattern( // Bottom right
                self.0 & Self::S,
                self.0 & Self::SE,
                self.0 & Self::E),
        ]
    }

    const E:  u8 = 0b00000001;
    const SE: u8 = 0b00000010;
    const S:  u8 = 0b00000100;
    const SW: u8 = 0b00001000;
    const W:  u8 = 0b00010000;
    const NW: u8 = 0b00100000;
    const N:  u8 = 0b01000000;
    const NE: u8 = 0b10000000;
}

fn get_corner_piece_pattern(ns: u8, between: u8, ew: u8) -> u8 {
    let ns = ns != 0;
    let between = between != 0;
    let ew = ew != 0;
    
    match (ns, between, ew) {
        (false, false, false) => 3,
        (false, false, true ) => 1,
        (false, true , false) => 3,
        (false, true , true ) => 1,
        (true , false, false) => 2,
        (true , false, true ) => 4,
        (true , true , false) => 2,
        (true , true , true ) => 0,
    }
}

