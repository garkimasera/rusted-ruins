
// Basic parameters for this game

/// Size of one tile
pub const TILE_SIZE: u32 = 48;
pub const TILE_SIZE_I: i32 = TILE_SIZE as i32;
/// The maximum height of wall images
pub const MAX_WALL_HEIGHT: u32 = 80;
/// Icon size
pub const ICON_SIZE: u32 = 24;

/// Maximum number of one character's items
pub const INVENTORY_MAX_ITEM_CHARA: usize = 32;
/// Maximum number of players's items
pub const INVENTORY_MAX_ITEM_PLAYER: usize = 32;
/// Maximum number of items on one tile
pub const INVENTORY_MAX_ITEM_TILE: usize = 256;

pub const WAIT_TIME_DEFAULT: f32 = 100.0;
/// The resolution of turns.
/// wait_time is decreased by SPD / TURN_RESOLUTION
pub const TURN_RESOLUTION: u32 = 100;

// Path settings
pub const CFG_FILES_DIR: &'static str = "config";
pub const OBJ_TXT_DIR: &'static str = "obj";
pub const LOG_TXT_DIR: &'static str = "log";
pub const UI_TXT_DIR: &'static str = "ui";

/// Special tile object id for downstairs
pub const SPECIAL_TILE_OBJ_DOWNSTAIRS: &'static str = "!downstairs";
/// Special tile object id for upstairs
pub const SPECIAL_TILE_OBJ_UPSTAIRS: &'static str = "!upstairs";

