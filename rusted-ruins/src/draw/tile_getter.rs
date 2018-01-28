
use common::objholder::*;

/// Needed infomation to draw background parts of an tile
/// "Background" means that they are drawed behind any characters
pub struct BackgroundDrawInfo {
    pub tile: TileIdx,
    pub deco: Option<DecoIdx>,
    pub wall: Option<WallIdx>,
}

/// If the number of items on one tile is more than this,
/// remaining items will be not drawed.
const MAX_ITEM_FOR_DRAW: usize = 5;

/// Needed infomation to draw foreground parts of an tile
/// "Foreground" means that they are drawed infront characters
/// whose are on the prev row
pub struct ForegroundDrawInfo {
    pub wall: Option<WallIdx>,
    pub n_item: usize,
    pub items: [ItemIdx; MAX_ITEM_FOR_DRAW],
    pub chara: Option<CharaTemplateIdx>,
}

