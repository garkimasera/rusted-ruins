use super::commonuse::*;
use super::group_window::*;
use super::widget::*;
use crate::config::UI_CFG;
use crate::context::textrenderer::FontKind;
use crate::game::extrait::*;
use crate::text::ToText;
use common::basic::SKILL_EXP_LVUP;
use common::gamedata::*;
use common::gobj;
use once_cell::sync::Lazy;

const STATUS_WINDOW_GROUP_SIZE: u32 = 2;

pub fn create_status_window_group(game: &Game, cid: CharaId) -> GroupWindow {
    // Workaround to specify cid for window creation
    use std::sync::Mutex;
    static TARGET_CID: Lazy<Mutex<Option<CharaId>>> = Lazy::new(|| Mutex::new(None));
    *TARGET_CID.lock().unwrap() = Some(cid);

    let mem_info = vec![
        MemberInfo {
            idx: gobj::id_to_idx("!tab-icon-chara-stats"),
            text_id: "tab_text-chara_stats",
            creator: |game| {
                Box::new(StatusWindow::new(
                    &game.gd,
                    TARGET_CID.lock().unwrap().unwrap(),
                ))
            },
        },
        MemberInfo {
            idx: gobj::id_to_idx("!tab-icon-chara-skills"),
            text_id: "tab_text-chara_skills",
            creator: |game| {
                Box::new(SkillWindow::new(
                    &game.gd,
                    TARGET_CID.lock().unwrap().unwrap(),
                ))
            },
        },
    ];
    let rect: Rect = UI_CFG.info_window.rect.into();
    GroupWindow::new(
        "status",
        STATUS_WINDOW_GROUP_SIZE,
        None,
        game,
        mem_info,
        (rect.x, rect.y),
    )
}

/// Character status viewer
pub struct StatusWindow {
    rect: Rect,
    image: ImageWidget,
    name_label: LabelWidget,
    hp_label: LabelWidget,
    sp_label: LabelWidget,
    str_label: LabelWidget,
    vit_label: LabelWidget,
    dex_label: LabelWidget,
    int_label: LabelWidget,
    wil_label: LabelWidget,
    cha_label: LabelWidget,
    escape_click: bool,
}

impl StatusWindow {
    pub fn new(gd: &GameData, cid: CharaId) -> StatusWindow {
        let cfg = &UI_CFG.status_window;
        let rect: Rect = UI_CFG.info_window.rect.into();
        let chara = gd.chara.get(cid);
        let image = ImageWidget::chara(cfg.image_rect, chara.template);
        let chara_name = if let Some(chara_name) = chara.name.clone() {
            chara_name
        } else {
            chara.to_text().into()
        };
        let name_label = LabelWidget::new(cfg.name_label_rect, &chara_name, FontKind::M);
        let hp_label = LabelWidget::new(
            cfg.hp_label_rect,
            &format!("HP  {} / {}", chara.hp, chara.attr.max_hp),
            FontKind::MonoM,
        );
        let sp_label = LabelWidget::new(
            cfg.sp_label_rect,
            &format!("SP  {:2.0}", chara.sp),
            FontKind::MonoM,
        );
        let str_label = LabelWidget::new(
            cfg.str_label_rect,
            &format!("STR  {}", chara.attr.str),
            FontKind::MonoM,
        );
        let vit_label = LabelWidget::new(
            cfg.vit_label_rect,
            &format!("VIT  {}", chara.attr.vit),
            FontKind::MonoM,
        );
        let dex_label = LabelWidget::new(
            cfg.dex_label_rect,
            &format!("DEX  {}", chara.attr.dex),
            FontKind::MonoM,
        );
        let int_label = LabelWidget::new(
            cfg.int_label_rect,
            &format!("INT  {}", chara.attr.int),
            FontKind::MonoM,
        );
        let wil_label = LabelWidget::new(
            cfg.wil_label_rect,
            &format!("WIL  {}", chara.attr.wil),
            FontKind::MonoM,
        );
        let cha_label = LabelWidget::new(
            cfg.cha_label_rect,
            &format!("CHA  {}", chara.attr.cha),
            FontKind::MonoM,
        );
        StatusWindow {
            rect,
            image,
            name_label,
            hp_label,
            sp_label,
            str_label,
            vit_label,
            dex_label,
            int_label,
            wil_label,
            cha_label,
            escape_click: false,
        }
    }
}

impl Window for StatusWindow {
    fn draw(&mut self, context: &mut Context, _game: &Game, _anim: Option<(&Animation, u32)>) {
        draw_window_border(context, self.rect);
        self.image.draw(context);
        self.name_label.draw(context);
        self.hp_label.draw(context);
        self.sp_label.draw(context);
        self.str_label.draw(context);
        self.vit_label.draw(context);
        self.dex_label.draw(context);
        self.int_label.draw(context);
        self.wil_label.draw(context);
        self.cha_label.draw(context);
    }
}

impl DialogWindow for StatusWindow {
    fn process_command(&mut self, command: &Command, _pa: &mut DoPlayerAction) -> DialogResult {
        check_escape_click!(self, command);

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
    list: ListWidget<(TextCache, TextCache, TextCache)>,
    escape_click: bool,
}

impl SkillWindow {
    pub fn new(gd: &GameData, cid: CharaId) -> SkillWindow {
        let rect: Rect = UI_CFG.info_window.rect.into();
        let cfg = &UI_CFG.skill_window;

        let mut list = ListWidget::with_scroll_bar(
            cfg.list_rect,
            cfg.column_pos.clone(),
            cfg.list_size,
            false,
        );

        let chara = gd.chara.get(cid);
        let mut skills: Vec<SkillKind> = chara.skills.skills.keys().copied().collect();
        skills.sort();
        let items: Vec<_> = skills
            .into_iter()
            .map(|skill_kind| {
                let (lv, adj) = chara.skill_level(skill_kind);
                let skill_name = TextCache::one(
                    skill_kind.to_text(),
                    FontKind::M,
                    UI_CFG.color.normal_font.into(),
                );
                let skill_level = if adj == 0 {
                    format!("Lv. {}", lv)
                } else if adj < 0 {
                    format!("Lv. {} - {}", lv, -adj)
                } else {
                    format!("Lv. {} + {}", lv, adj)
                };
                let skill_level =
                    TextCache::one(skill_level, FontKind::M, UI_CFG.color.normal_font.into());
                let (_, skill_exp) = chara.skills.get_level_exp(skill_kind);
                let skill_exp = TextCache::one(
                    format!(
                        "({:0.1} %)",
                        skill_exp as f32 / SKILL_EXP_LVUP as f32 * 100.0
                    ),
                    FontKind::M,
                    UI_CFG.color.normal_font.into(),
                );
                (skill_name, skill_level, skill_exp)
            })
            .collect();

        list.set_items(items);

        SkillWindow {
            rect,
            list,
            escape_click: false,
        }
    }
}

impl Window for SkillWindow {
    fn draw(&mut self, context: &mut Context, _game: &Game, _anim: Option<(&Animation, u32)>) {
        draw_window_border(context, self.rect);
        self.list.draw(context);
    }
}

impl DialogWindow for SkillWindow {
    fn process_command(&mut self, command: &Command, _pa: &mut DoPlayerAction) -> DialogResult {
        check_escape_click!(self, command);
        let command = command.relative_to(self.rect);

        self.list.process_command(&command);

        match command {
            Command::Cancel => DialogResult::Close,
            _ => DialogResult::Continue,
        }
    }

    fn mode(&self) -> InputMode {
        InputMode::Dialog
    }
}
