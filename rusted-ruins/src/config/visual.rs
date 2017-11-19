
use super::{CfgRect, CfgColor};

/// Size of screen and rects of windows
/// These parameters will change if screen size is different
#[derive(Debug, Deserialize)]
pub struct ScreenConfig {
    pub screen_w: u32, pub screen_h: u32,
    pub main_window: CfgRect,
    pub log_window: CfgRect,
    pub minimap_window: CfgRect,
    pub hp_indicator: CfgRect,
    pub floor_info: CfgRect,
    pub hborders: Vec<BorderConfig>,
    pub vborders: Vec<BorderConfig>,
}

#[derive(Debug, Deserialize)]
pub struct BorderConfig {
    pub x: i32,
    pub y: i32,
    pub len: u32,
}

/// UI colors, fonts, and other widget settings
#[derive(Debug, Deserialize)]
pub struct UIConfig {
    pub cursor_move_duration: u64,
    
    pub color: UIColorConfig,
    pub font: Font,
    pub log_window: LogWindowConfig,
    pub exit_window: ExitWindowConfig,
    pub start_dialog: StartDialogConfig,
    pub text_input_dialog: TextInputDialogConfig,
    pub item_window: ItemWindowConfig,
    pub label_widget: LabelWidgetConfig,
    pub list_widget: ListWidgetConfig,
}

#[derive(Debug, Deserialize)]
pub struct UIColorConfig {
    pub border_light: CfgColor,
    pub border_dark: CfgColor,
    pub border_highlight_light: CfgColor,
    pub border_highlight_dark: CfgColor,
    pub window_bg: CfgColor,
    pub window_bg_highlight: CfgColor,
    pub log_window_bg: CfgColor,
    pub log_font: CfgColor,
    pub normal_font: CfgColor,
    pub guage_border_light: CfgColor,
    pub guage_border_dark: CfgColor,
    pub guage_bg: CfgColor,
    pub guage_hp: CfgColor,
}

#[derive(Debug, Deserialize)]
pub struct FontConfig {
    pub file: String,
    pub size: u16,
}

#[derive(Debug, Deserialize)]
pub struct Font {
    pub log: FontConfig,
    pub s: FontConfig,
    pub m: FontConfig,
}

#[derive(Debug, Deserialize)]
pub struct LogWindowConfig {
    pub h: i32,
    pub n_display_line: u32,
}

#[derive(Debug, Deserialize)]
pub struct ExitWindowConfig {
    pub rect: CfgRect,
    pub list_y: i32,
}

#[derive(Debug, Deserialize)]
pub struct StartDialogConfig {
    pub rect: CfgRect,
}

#[derive(Debug, Deserialize)]
pub struct TextInputDialogConfig {
    pub rect: CfgRect,
}

#[derive(Debug, Deserialize)]
pub struct ItemWindowConfig {
    pub rect: CfgRect,
}

#[derive(Debug, Deserialize)]
pub struct LabelWidgetConfig {
    pub h: i32,
    pub left_margin: i32,
}

#[derive(Debug, Deserialize)]
pub struct ListWidgetConfig {
    pub h_row_with_text: i32,
    pub left_margin: i32,
}

