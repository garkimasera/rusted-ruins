
use common::basic::SKILL_EXP_LVUP;
use common::gobj;
use super::commonuse::*;
use super::widget::*;
use sdlvalues::FontKind;
use config::UI_CFG;
use common::gamedata::*;
use game::extrait::*;
use text::ToText;
use super::group_window::*;

const STATUS_WINDOW_GROUP_SIZE: usize = 2;

pub fn create_status_window_group(game: &Game) -> GroupWindow {
    let mem_info = vec![
        MemberInfo {
            idx: gobj::id_to_idx("!tab-icon-chara-stats"),
            text_id: "tab_text.chara_stats",
            creator: |game| Box::new(StatusWindow::new(&game.gd)),
        },
        MemberInfo {
            idx: gobj::id_to_idx("!tab-icon-chara-skills"),
            text_id: "tab_text.chara_skills",
            creator: |game| Box::new(SkillWindow::new(&game.gd)),
        },
    ];
    GroupWindow::new(STATUS_WINDOW_GROUP_SIZE, 0, game, mem_info)
}

/// Character status viewer
pub struct StatusWindow {
    rect: Rect,
    image: ImageWidget,
    name_label: LabelWidget,
    hp_label: LabelWidget,
    str_label: LabelWidget,
    vit_label: LabelWidget,
    dex_label: LabelWidget,
    int_label: LabelWidget,
    wil_label: LabelWidget,
    cha_label: LabelWidget,
}

impl StatusWindow {
    pub fn new(gd: &GameData) -> StatusWindow {
        let cfg = &UI_CFG.status_window;
        let rect: Rect = cfg.rect.into();
        let chara = gd.chara.get(CharaId::Player);
        let image = ImageWidget::chara(cfg.image_rect, chara.template);
        let name_label = LabelWidget::new(cfg.name_label_rect, &chara.to_text(), FontKind::M);
        let hp_label = LabelWidget::new(
            cfg.hp_label_rect, &format!("HP  {} / {}", chara.hp, chara.attr.max_hp), FontKind::MonoM);
        let str_label = LabelWidget::new(
            cfg.str_label_rect, &format!("STR  {}", chara.attr.str), FontKind::MonoM);
        let vit_label = LabelWidget::new(
            cfg.vit_label_rect, &format!("VIT  {}", chara.attr.vit), FontKind::MonoM);
        let dex_label = LabelWidget::new(
            cfg.dex_label_rect, &format!("DEX  {}", chara.attr.dex), FontKind::MonoM);
        let int_label = LabelWidget::new(
            cfg.int_label_rect, &format!("INT  {}", chara.attr.int), FontKind::MonoM);
        let wil_label = LabelWidget::new(
            cfg.wil_label_rect, &format!("WIL  {}", chara.attr.wil), FontKind::MonoM);
        let cha_label = LabelWidget::new(
            cfg.cha_label_rect, &format!("CHA  {}", chara.attr.cha), FontKind::MonoM);
        StatusWindow {
            rect,
            image, name_label, hp_label,
            str_label, vit_label, dex_label, int_label, wil_label, cha_label,
        }
    }
}

impl Window for StatusWindow {
    fn draw(
        &mut self, canvas: &mut WindowCanvas, _game: &Game, sv: &mut SdlValues,
        _anim: Option<(&Animation, u32)>) {

        draw_rect_border(canvas, self.rect);
        self.image.draw(canvas, sv);
        self.name_label.draw(canvas, sv);
        self.hp_label.draw(canvas, sv);
        self.str_label.draw(canvas, sv);
        self.vit_label.draw(canvas, sv);
        self.dex_label.draw(canvas, sv);
        self.int_label.draw(canvas, sv);
        self.wil_label.draw(canvas, sv);
        self.cha_label.draw(canvas, sv);
    }
}

impl DialogWindow for StatusWindow {
    fn process_command(&mut self, command: &Command, _pa: &mut DoPlayerAction) -> DialogResult {
        match *command {
            Command::Cancel => DialogResult::Close,
            _ => DialogResult::Continue,
        }
    }

    fn mode(&self) -> InputMode {
        InputMode::Dialog
    }
}

/// Character skill viewer
pub struct SkillWindow {
    rect: Rect,
    /// Gauge widget to display skill level and exp
    gauges: Vec<GaugeWidget>,
    /// Skill name label
    labels: Vec<LabelWidget>,
}

impl SkillWindow {
    pub fn new(gd: &GameData) -> SkillWindow {
        let rect: Rect = UI_CFG.skill_window.rect.into();
        let mut gauges: Vec<GaugeWidget> = Vec::new();
        let mut labels: Vec<LabelWidget> = Vec::new();
        
        let chara = gd.chara.get(::common::gamedata::chara::CharaId::Player);
        for (i, skill_kind) in chara.skills.skills.keys().enumerate() {
            let (skill_level, exp) = chara.skills.get_level_exp(*skill_kind);
            let i0 = i as i32 / UI_CFG.skill_window.n_row as i32;
            let i1 = i as i32 % UI_CFG.skill_window.n_row as i32;

            let pos_x = i0 * UI_CFG.skill_window.gauge_w;
            let pos_y = i1 * UI_CFG.skill_window.gauge_h;

            let mut label_rect: Rect = UI_CFG.skill_window.label_rect.into();
            label_rect.offset(pos_x, pos_y);

            let label_text = skill_kind.to_text();
            let label = LabelWidget::new(label_rect, &label_text, FontKind::S);
            labels.push(label);

            let mut gauge_rect: Rect = UI_CFG.skill_window.gauge_rect.into();
            gauge_rect.offset(pos_x, pos_y);

            let level_text = format!("{}", skill_level);

            let mut gauge = GaugeWidget::with_label(
                gauge_rect, 0.0, 1.0, GaugeColorMode::Exp, &level_text);
            gauge.set_value(exp as f32 / SKILL_EXP_LVUP as f32);
            gauges.push(gauge);
        }
        
        SkillWindow {
            rect,
            gauges,
            labels,
        }
    }
}

impl Window for SkillWindow {
    fn draw(
        &mut self, canvas: &mut WindowCanvas, _game: &Game, sv: &mut SdlValues,
        _anim: Option<(&Animation, u32)>) {

        draw_rect_border(canvas, self.rect);
        for w in &mut self.gauges {
            w.draw(canvas, sv);
        }
        for w in &mut self.labels {
            w.draw(canvas, sv);
        }
    }
}

impl DialogWindow for SkillWindow {
    fn process_command(&mut self, command: &Command, _pa: &mut DoPlayerAction) -> DialogResult {
        match *command {
            Command::Cancel => DialogResult::Close,
            _ => DialogResult::Continue,
        }
    }

    fn mode(&self) -> InputMode {
        InputMode::Dialog
    }
}
