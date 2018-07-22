
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
    pub date_info: CfgRect,
    pub time_info: CfgRect,
    pub status_info: CfgRect,
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
    pub gap_len_between_dialogs: i32,
    
    pub color: UIColorConfig,
    pub font: Font,
    pub log_window: LogWindowConfig,
    pub exit_window: ExitWindowConfig,
    pub talk_window: TalkWindowConfig,
    pub start_dialog: StartDialogConfig,
    pub msg_dialog: MsgDialogConfig,
    pub text_input_dialog: TextInputDialogConfig,
    pub newgame_dialog: NewGameDialogConfig,
    pub choose_class_dialog: ChooseClassDialogConfig,
    pub item_window: ItemWindowConfig,
    pub equip_window: EquipWindowConfig,
    pub scrolling_text_window: ScrollingTextWindowConfig,
    pub status_window: StatusWindowConfig,
    pub game_info_window: GameInfoWindowConfig,
    pub skill_window: SkillWindowConfig,
    pub page_window: PageWindowConfig,
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
    pub size: u16,
}

/// Font config for each use case
#[derive(Debug, Deserialize)]
pub struct Font {
    /// For logging window
    pub log: FontConfig,
    /// Small size ui text
    pub s: FontConfig,
    /// Mediam size text
    pub m: FontConfig,
}

#[derive(Debug, Deserialize)]
pub struct LogWindowConfig {
    pub h: i32,
    pub n_display_line: usize,
}

#[derive(Debug, Deserialize)]
pub struct ExitWindowConfig {
    pub rect: CfgRect,
}

#[derive(Debug, Deserialize)]
pub struct TalkWindowConfig {
    pub rect: CfgRect,
    pub n_default_line: usize,
    /// Relative position to parent talk window
    pub image_window_pos_x: i32,
    /// Relative position to parent talk window
    pub image_window_pos_y: i32,
}

#[derive(Debug, Deserialize)]
pub struct MsgDialogConfig {
    pub rect: CfgRect,
}

#[derive(Debug, Deserialize)]
pub struct ScrollingTextWindowConfig {
    pub line_space: i32,
    pub speed: f64,
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
pub struct NewGameDialogConfig {
    pub explanation_text_rect: CfgRect,
}

#[derive(Debug, Deserialize)]
pub struct ChooseClassDialogConfig {
    pub rect: CfgRect,
}

#[derive(Debug, Deserialize)]
pub struct ItemWindowConfig {
    pub rect: CfgRect,
    pub n_row: u32,
    pub column_pos: Vec<i32>,
}

#[derive(Debug, Deserialize)]
pub struct EquipWindowConfig {
    pub rect: CfgRect,
    pub n_row: u32,
    pub column_pos: Vec<i32>,
}

#[derive(Debug, Deserialize)]
pub struct StatusWindowConfig {
    pub rect: CfgRect,
    pub image_rect: CfgRect,
    pub name_label_rect: CfgRect,
    pub hp_label_rect: CfgRect,
    pub str_label_rect: CfgRect,
    pub vit_label_rect: CfgRect,
    pub dex_label_rect: CfgRect,
    pub int_label_rect: CfgRect,
    pub wil_label_rect: CfgRect,
    pub cha_label_rect: CfgRect,
}

#[derive(Debug, Deserialize)]
pub struct GameInfoWindowConfig {
    pub rect: CfgRect,
    pub money_label_rect: CfgRect,
}

#[derive(Debug, Deserialize)]
pub struct SkillWindowConfig {
    pub rect: CfgRect,
    pub n_row: u32,
}

#[derive(Debug, Deserialize)]
pub struct PageWindowConfig {
    pub rect: CfgRect,
    pub margin_to_parent: i32,
    pub label_y: i32,
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

