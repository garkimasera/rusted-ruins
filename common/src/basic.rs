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

/// Maximum number of registered action shortcuts
pub const MAX_ACTION_SHORTCUTS: usize = 32;

// Path settings
pub const APP_DIR_NAME: &str = "rusted-ruins";
pub const CFG_FILES_DIR: &str = "config";
pub const ABILITY_TXT_DIR: &str = "ability";
pub const FLAVOR_TXT_DIR: &str = "flavor";
pub const OBJ_TXT_DIR: &str = "obj";
pub const QUEST_TXT_DIR: &str = "quest";
pub const LOG_TXT_DIR: &str = "log";
pub const UI_TXT_DIR: &str = "ui";
pub const TALK_TXT_DIR: &str = "talk";
pub const MISC_TXT_DIR: &str = "misc";
pub const READABLE_TXT_DIR: &str = "readable";
pub const SAVE_DIR_NAME: &str = "save";
pub const SAVE_EXTENSION: &str = "rrsve";

/// Id table
pub const ID_TABLE_SECTION_TAG: &str = "§";

/// The number of auto generated dungeons per region
pub const MAX_AUTO_GEN_DUNGEONS: u32 = 20;

/// If the number of items on one tile is more than this,
/// remaining items will be not drawed.
pub const MAX_ITEM_FOR_DRAW: usize = 5;

/// The number of tile image layers
pub const N_TILE_IMG_LAYER: usize = 4;

/// Length of ArrayString id types
pub const ARRAY_STR_ID_LEN: usize = 15;

/// Copyable string type for id
pub type ArrayStringId = arrayvec::ArrayString<ARRAY_STR_ID_LEN>;

/// Bonus / penalty representation used in this game
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
#[repr(i8)]
pub enum BonusLevel {
    Awful = -4,
    VeryBad = -3,
    Bad = -2,
    SlightlyBad = -1,
    None = 0,
    SlightlyGood = 1,
    Good = 2,
    VeryGood = 3,
    Excellent = 4,
    Superb = 5,
}

impl Default for BonusLevel {
    fn default() -> Self {
        BonusLevel::None
    }
}
