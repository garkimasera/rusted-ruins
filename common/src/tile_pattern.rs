//! Tile pattern will be defined by surround 8 tiles.
//! TilePatternFlag helps to search appropriate pattern.

use array2d::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct TilePatternFlags(pub u8);

impl TilePatternFlags {
    pub fn new() -> TilePatternFlags {
        TilePatternFlags(0)
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
    
    pub fn to_i_pattern(self) -> u8 {
        match self.0 {
            0b11111111 => 0,
            0b00111110 => 1,
            0b10001111 => 2,
            0b11100011 => 3,
            0b11111000 => 4,
            0b00111000 => 5,
            0b00001110 => 6,
            0b10000011 => 7,
            0b11100000 => 8,
            0b11111110 => 9,
            0b10111111 => 10,
            0b11101111 => 11,
            0b11111011 => 12,
            _ => 0,
        }
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

