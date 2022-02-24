use super::{CfgColor, CfgRect};

/// Size of screen and rects of windows
/// These parameters will change if screen size is different
#[derive(Debug, Deserialize)]
pub struct ScreenConfig {
    pub screen_w: u32,
    pub screen_h: u32,
    pub main_window: CfgRect,
    pub log_window: CfgRect,
    pub minimap_window: CfgRect,
    pub sidebar: CfgRect,
    pub toolbar: CfgRect,
    pub shortcut_list: CfgRect,
    pub hp_indicator: CfgRect,
    pub mp_indicator: CfgRect,
    pub sp_indicator: CfgRect,
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
pub struct UiConfig {
    pub cursor_move_duration: u64,
    pub gap_len_between_dialogs: i32,

    pub color: UiColorConfig,
    pub font: Font,
    pub log_window: LogWindowConfig,
    pub exit_window: ExitWindowConfig,
    pub help_window: HelpWindowConfig,
    pub talk_window: TalkWindowConfig,
    pub read_window: ReadWindowConfig,
    pub start_dialog: StartDialogConfig,
    pub msg_dialog: MsgDialogConfig,
    pub text_input_dialog: TextInputDialogConfig,
    pub newgame_dialog: NewGameDialogConfig,
    pub build_obj_dialog: BuildObjDialogConfig,
    pub choose_save_file_dialog: ChooseSaveFileDialogConfig,
    pub choose_class_dialog: ChooseClassDialogConfig,
    pub creation_window: CreationWindowConfig,
    pub creation_detail_dialog: CreationDetailDialogConfig,
    pub faction_window: FactionWindowConfig,
    pub item_window: ItemWindowConfig,
    pub item_info_window: ItemInfoWindowConfig,
    pub equip_window: EquipWindowConfig,
    pub flavor_text_window: FlavorTextWindowConfig,
    pub scrolling_text_window: ScrollingTextWindowConfig,
    pub info_window: InfoWindowConfig,
    pub status_window: StatusWindowConfig,
    pub chara_trait_window: CharaTraitWindowConfig,
    pub ability_window: AbilityWindowConfig,
    pub game_info_window: GameInfoWindowConfig,
    pub skill_window: SkillWindowConfig,
    pub quest_window: QuestWindowConfig,
    pub slot_window: SlotWindowConfig,
    pub label_widget: LabelWidgetConfig,
    pub list_widget: ListWidgetConfig,
    pub supplement_info: SupplementInfoConfig,
    pub time_info: TimeInfoConfig,
    pub progress_bar: ProgressBarConfig,
    pub vscroll_widget: VScrollWidgetConfig,
    pub sidebar: SidebarConfig,
    pub toolbar: ToolbarConfig,
    pub chara_info: CharaInfoConfig,
    pub damage_popup: DamagePopupConfig,
}

#[derive(Debug, Deserialize)]
pub struct UiColorConfig {
    pub border_dark: CfgColor,
    pub border_highlight_dark: CfgColor,
    pub border_highlight_light: CfgColor,
    pub border_light: CfgColor,
    pub button_normal_bg: CfgColor,
    pub button_normal_bg_covered: CfgColor,
    pub button_normal_border_dark: CfgColor,
    pub button_normal_border_light: CfgColor,
    pub gauge_bg: CfgColor,
    pub gauge_border_dark: CfgColor,
    pub gauge_border_light: CfgColor,
    pub gauge_hp: CfgColor,
    pub gauge_mp: CfgColor,
    pub gauge_sp: CfgColor,
    pub gauge_work: CfgColor,
    pub list_border: CfgColor,
    pub list_header_border: CfgColor,
    pub log_font: CfgColor,
    pub log_window_bg: CfgColor,
    pub normal_font: CfgColor,
    pub sidebar_bg: CfgColor,
    pub toolbar_bg: CfgColor,
    pub vscroll_border: CfgColor,
    pub vscroll_border_inner: CfgColor,
    pub vscroll_knob: CfgColor,
    pub vscroll_knob_border: CfgColor,
    pub window_bg: CfgColor,
    pub window_bg_highlight: CfgColor,
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
    /// Tiny size ui text
    pub t: FontConfig,
    /// Small size ui text
    pub s: FontConfig,
    /// Mediam size text
    pub m: FontConfig,
    /// For talk or book texts
    pub talk: FontConfig,
}

#[derive(Debug, Deserialize)]
pub struct LogWindowConfig {
    pub h: i32,
    pub n_display_line: usize,
}

#[derive(Debug, Deserialize)]
pub struct ExitWindowConfig {
    pub rect: CfgRect,
    pub save_button_rect: CfgRect,
    pub exit_button_rect: CfgRect,
    pub title_screen_button_rect: CfgRect,
    pub cancel_button_rect: CfgRect,
}

#[derive(Debug, Deserialize)]
pub struct HelpWindowConfig {
    pub rect: CfgRect,
    pub key_label_start: CfgRect,
    pub key_label_h: i32,
}

#[derive(Debug, Deserialize)]
pub struct TalkWindowConfig {
    pub rect: CfgRect,
    pub text_wrap_width: u32,
    /// Relative position to parent talk window
    pub image_window_pos_x: i32,
    /// Relative position to parent talk window
    pub image_window_pos_y: i32,
}

#[derive(Debug, Deserialize)]
pub struct ReadWindowConfig {
    pub rect: CfgRect,
    pub text_rect: CfgRect,
    pub next_button_rect: CfgRect,
    pub prev_button_rect: CfgRect,
    pub page_label_rect: CfgRect,
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
    pub class_column_pos: Vec<i32>,
    pub trait_column_pos: Vec<i32>,
    pub cost_label_width: u32,
    pub remaining_point_text_rect: CfgRect,
    pub remaining_point_text_margin: i32,
    pub choose_trait_bottom_margin: i32,
    pub top_margin: i32,
    pub bottom_margin: i32,
    pub button_w: u32,
    pub button_h: u32,
    pub button_space: u32,
}

#[derive(Debug, Deserialize)]
pub struct BuildObjDialogConfig {
    pub rect: CfgRect,
    pub n_row: u32,
    pub column_pos: Vec<i32>,
}

#[derive(Debug, Deserialize)]
pub struct ChooseSaveFileDialogConfig {
    pub rect: CfgRect,
    pub list_size: u32,
}

#[derive(Debug, Deserialize)]
pub struct ChooseClassDialogConfig {
    pub rect: CfgRect,
}

#[derive(Debug, Deserialize)]
pub struct CreationWindowConfig {
    pub rect: CfgRect,
    pub n_row: u32,
    pub column_pos: Vec<i32>,
}

#[derive(Debug, Deserialize)]
pub struct CreationDetailDialogConfig {
    pub rect: CfgRect,
    pub n_row: u32,
    pub column_pos: Vec<i32>,
    pub list_margin: i32,
    pub product_name: CfgRect,
    pub facility_ok_icon_rect: CfgRect,
    pub facility_label_rect: CfgRect,
    pub enough_ingredients_icon_rect: CfgRect,
    pub enough_ingredients_label_rect: CfgRect,
    pub required_skill_icon_rect: CfgRect,
    pub required_skill_label_rect: CfgRect,
    pub start_button_rect: CfgRect,
    pub cancel_button_rect: CfgRect,
}

#[derive(Debug, Deserialize)]
pub struct FactionWindowConfig {
    pub list_rect: CfgRect,
    pub n_row: u32,
    pub column_pos: Vec<i32>,
}

#[derive(Debug, Deserialize)]
pub struct ItemWindowConfig {
    pub rect: CfgRect,
    pub n_row: u32,
    pub column_pos: Vec<i32>,
    pub info_label_rect0: CfgRect,
    pub info_label_rect1: CfgRect,
}

#[derive(Debug, Deserialize)]
pub struct ItemInfoWindowConfig {
    pub rect: CfgRect,
    pub item_image: CfgRect,
    pub item_name: CfgRect,
    pub item_kind: CfgRect,
    pub column_pos: Vec<i32>,
    pub flavor_text_y: i32,
    pub list_y: i32,
}

#[derive(Debug, Deserialize)]
pub struct EquipWindowConfig {
    pub n_row: u32,
    pub column_pos: Vec<i32>,
}

#[derive(Debug, Deserialize)]
pub struct FlavorTextWindowConfig {
    pub rect: CfgRect,
    pub image_rect: CfgRect,
    pub name_rect: CfgRect,
    pub text_rect: CfgRect,
    pub close_button_rect: CfgRect,
}

#[derive(Debug, Deserialize)]
pub struct InfoWindowConfig {
    pub rect: CfgRect,
}

#[derive(Debug, Deserialize)]
pub struct StatusWindowConfig {
    pub image_rect: CfgRect,
    pub name_label_rect: CfgRect,
    pub faction_label_rect: CfgRect,
    pub lv_label_rect: CfgRect,
    pub hp_label_rect: CfgRect,
    pub mp_label_rect: CfgRect,
    pub sp_label_rect: CfgRect,
    pub str_label_rect: CfgRect,
    pub vit_label_rect: CfgRect,
    pub dex_label_rect: CfgRect,
    pub int_label_rect: CfgRect,
    pub wil_label_rect: CfgRect,
    pub cha_label_rect: CfgRect,
    pub spd_label_rect: CfgRect,
    pub power_labels_rect: CfgRect,
    pub power_labels_icon_rect: CfgRect,
    pub power_labels_label_dy: i32,
    pub carry_label_rect: CfgRect,
    pub travel_speed_label_rect: CfgRect,
}

#[derive(Debug, Deserialize)]
pub struct CharaTraitWindowConfig {
    pub column_pos: Vec<i32>,
}

#[derive(Debug, Deserialize)]
pub struct AbilityWindowConfig {
    pub rect: CfgRect,
    pub list_rect: CfgRect,
    pub n_row: u32,
    pub column_pos: Vec<i32>,
}

#[derive(Debug, Deserialize)]
pub struct GameInfoWindowConfig {
    pub money_label_rect: CfgRect,
    pub play_time_label_rect: CfgRect,
}

#[derive(Debug, Deserialize)]
pub struct SkillWindowConfig {
    pub list_rect: CfgRect,
    pub list_size: u32,
    pub column_pos: Vec<i32>,
}

#[derive(Debug, Deserialize)]
pub struct QuestWindowConfig {
    pub h: u32,
    pub n_row: u32,
    pub column_pos: Vec<i32>,
    pub ok_button_rect: CfgRect,
    pub cancel_button_rect: CfgRect,
}

#[derive(Debug, Deserialize)]
pub struct SlotWindowConfig {
    pub rect: CfgRect,
    pub column_pos: Vec<i32>,
    pub n_row: u32,
}

#[derive(Debug, Deserialize)]
pub struct LabelWidgetConfig {
    pub h: i32,
    pub left_margin: i32,
}

#[derive(Debug, Deserialize)]
pub struct ListWidgetConfig {
    pub h_row_default: u32,
    pub h_row_with_text: u32,
    pub icon_column_w: u32,
    pub left_margin: i32,
}

#[derive(Debug, Deserialize)]
pub struct SupplementInfoConfig {
    pub margin: i32,
    pub h: i32,
}

#[derive(Debug, Deserialize)]
pub struct TimeInfoConfig {
    pub time_label: CfgRect,
    pub date_label: CfgRect,
}

#[derive(Debug, Deserialize)]
pub struct ProgressBarConfig {
    pub rect: CfgRect,
}

#[derive(Debug, Deserialize)]
pub struct VScrollWidgetConfig {
    pub width: u32,
    pub button_height: u32,
    pub min_knob_size: u32,
    pub button_repeat_duration: u64,
}

#[derive(Debug, Deserialize)]
pub struct SidebarConfig {
    pub icon_w: u32,
    pub icon_h: u32,
    pub space: u32,
}

#[derive(Debug, Deserialize)]
pub struct ToolbarConfig {
    pub icon_w: u32,
    pub icon_h: u32,
    pub n_shortcut: u32,
}

#[derive(Debug, Deserialize)]
pub struct CharaInfoConfig {
    pub target_icon_x: i32,
    pub target_icon_y: i32,
}

#[derive(Debug, Deserialize)]
pub struct DamagePopupConfig {
    pub start_dy: i32,
    pub digit_h: i32,
    pub n_frame: u32,
    pub speed: i32,
}
