
use common::objholder::TileIdx;
use array2d::*;

pub struct EditingMap {
    pub width: u32,
    pub height: u32,
    pub tile: Array2d<TileIdx>,
}

impl EditingMap {
    pub fn new(width: u32, height: u32) -> EditingMap {
        let tile = Array2d::new(width, height, TileIdx(0));
        EditingMap { tile, width, height }
    }
}

