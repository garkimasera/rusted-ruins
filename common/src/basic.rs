// Basic parameters for this game

/// Size of one tile
pub const TILE_SIZE: u32 = 48;
pub const TILE_SIZE_I: i32 = TILE_SIZE as i32;
/// Size of piece image
/// One tile includes 4 pieces
pub const PIECE_SIZE: u32 = TILE_SIZE / 2;
pub const PIECE_SIZE_I: i32 = TILE_SIZE_I / 2;
/// The maximum height of wall images
pub const MAX_WALL_HEIGHT: u32 = 80;
/// Icon size
pub const ICON_SIZE: u32 = 24;
/// Tab icon width
pub const TAB_ICON_W: u32 = 48;
/// Tab icon height
pub const TAB_ICON_H: u32 = 32;
/// Tab text height
pub const TAB_TEXT_H: u32 = 16;
/// Window border thickness
pub const WINDOW_BORDER_THICKNESS: u32 = 3;

/// Maximum number of items on one tile
pub const MAX_ITEM_TILE: usize = 256;
/// Maximum number of equipment slots
pub const MAX_EQUIP_SLOT: usize = 16;

pub const WAIT_TIME_NUMERATOR: u32 = 100000;

/// Needed exp value to level up
pub const SKILL_EXP_LVUP: u16 = 10000;

// Path settings
pub const APP_DIR_NAME: &'static str = "rusted-ruins";
pub const CFG_FILES_DIR: &'static str = "config";
pub const OBJ_TXT_DIR: &'static str = "obj";
pub const LOG_TXT_DIR: &'static str = "log";
pub const UI_TXT_DIR: &'static str = "ui";
pub const TALK_TXT_DIR: &'static str = "talk";
pub const MISC_TXT_DIR: &'static str = "misc";
pub const SAVE_DIR_NAME: &'static str = "save";
pub const SAVE_EXTENSION: &'static str = "rrsve";

/// Id table
pub const ID_TABLE_SECTION_TAG: &'static str = "§";

/// The number of auto generated dungeons per region
pub const MAX_AUTO_GEN_DUNGEONS: u32 = 20;

/// If the number of items on one tile is more than this,
/// remaining items will be not drawed.
pub const MAX_ITEM_FOR_DRAW: usize = 5;

/// The number of tile image layers
pub const N_TILE_IMG_LAYER: usize = 4;
