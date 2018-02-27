
// Basic parameters for this game

/// Size of one tile
pub const TILE_SIZE: u32 = 48;
pub const TILE_SIZE_I: i32 = TILE_SIZE as i32;
/// The maximum height of wall images
pub const MAX_WALL_HEIGHT: u32 = 80;
/// Icon size
pub const ICON_SIZE: u32 = 24;

/// Maximum number of one character's items
pub const MAX_ITEM_CHARA: usize = 32;
/// Maximum number of players's items
pub const MAX_ITEM_PLAYER: usize = 32;
/// Maximum number of items on one tile
pub const MAX_ITEM_TILE: usize = 256;
/// Maximum number of equipment slots
pub const MAX_EQUIP_SLOT: usize = 16;

pub const WAIT_TIME_START: u32 = 10000;

// Path settings
pub const CFG_FILES_DIR: &'static str = "config";
pub const OBJ_TXT_DIR: &'static str = "obj";
pub const LOG_TXT_DIR: &'static str = "log";
pub const UI_TXT_DIR: &'static str = "ui";
pub const TALK_TXT_DIR: &'static str = "talk";
pub const MISC_TXT_DIR: &'static str = "misc";

/// The number of auto generated dungeons per region
pub const MAX_AUTO_GEN_DUNGEONS: u32 = 20;

/// If the number of items on one tile is more than this,
/// remaining items will be not drawed.
pub const MAX_ITEM_FOR_DRAW: usize = 5;

