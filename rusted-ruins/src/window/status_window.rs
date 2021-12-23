use super::commonuse::*;
use super::equip_window::EquipWindow;
use super::group_window::*;
use super::list_desc_window::ListWithDescWindow;
use super::widget::*;
use crate::config::UI_CFG;
use crate::context::textrenderer::FontKind;
use crate::game::extrait::*;
use crate::text::{ui_txt, ToText};
use common::basic::SKILL_EXP_LVUP;
use common::gamedata::*;
use common::gobj;

const STATUS_WINDOW_GROUP_SIZE: u32 = 4;

pub fn create_status_window_group(
    game: &Game,
    cid: CharaId,
    changeable_by_player: bool,
) -> GroupWindow {
    let mem_info: Vec<(MemberInfo, ChildWinCreator)> = vec![
        (
            MemberInfo {
                idx: gobj::id_to_idx("!tab-icon-chara-stats"),
                text_id: "tab_text-chara_stats",
            },
            Box::new(move |game| Box::new(StatusWindow::new(&game.gd, cid))),
        ),
        (
            MemberInfo {
                idx: gobj::id_to_idx("!tab-icon-chara-equipments"),
                text_id: "tab_text-chara_equipments",
            },
            Box::new(move |game| Box::new(EquipWindow::new(&game.gd, cid, changeable_by_player))),
        ),
        (
            MemberInfo {
                idx: gobj::id_to_idx("!tab-icon-chara-skills"),
                text_id: "tab_text-chara_skills",
            },
            Box::new(move |game| Box::new(SkillWindow::new(&game.gd, cid))),
        ),
        (
            MemberInfo {
                idx: gobj::id_to_idx("!tab-icon-chara-traits"),
                text_id: "tab_text-chara_traits",
            },
            Box::new(move |game| Box::new(CharaTraitWindow::new(&game.gd, cid))),
        ),
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
    faction_label: LabelWidget,
    lv_label: LabelWidget,
    hp_label: LabelWidget,
    sp_label: LabelWidget,
    str_label: LabelWidget,
    vit_label: LabelWidget,
    dex_label: LabelWidget,
    int_label: LabelWidget,
    wil_label: LabelWidget,
    cha_label: LabelWidget,
    spd_label: LabelWidget,
    carry_label: LabelWidget,
    travel_speed_label: LabelWidget,
    escape_click: bool,
}

impl StatusWindow {
    pub fn new(gd: &GameData, cid: CharaId) -> StatusWindow {
        let cfg = &UI_CFG.status_window;
        let rect: Rect = UI_CFG.info_window.rect.into();
        let chara = gd.chara.get(cid);
        let ct = gobj::get_obj(chara.idx);
        let image = ImageWidget::chara(cfg.image_rect, chara.idx);
        let chara_name = if let Some(chara_name) = chara.name.clone() {
            chara_name
        } else {
            chara.to_text().into()
        };
        let name_label = LabelWidget::new(cfg.name_label_rect, &chara_name, FontKind::M);
        let faction_label = LabelWidget::new(
            cfg.faction_label_rect,
            &format!(
                "{}  {}",
                ui_txt("label_text-status-faction"),
                chara.faction.to_text()
            ),
            FontKind::M,
        );
        let lv_label = LabelWidget::new(
            cfg.lv_label_rect,
            &format!("Lv. {}", chara.lv),
            FontKind::MonoM,
        );
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
        let spd_label = LabelWidget::new(
            cfg.spd_label_rect,
            &format!("SPD  {}", chara.attr.spd),
            FontKind::MonoM,
        );
        let carry_label = LabelWidget::new(
            cfg.carry_label_rect,
            &format!(
                "{}  {}",
                ui_txt("label_text-status-carry"),
                ct.base_attr.carry
            ),
            FontKind::S,
        );
        let travel_speed_label = LabelWidget::new(
            cfg.travel_speed_label_rect,
            &format!(
                "{}  {}",
                ui_txt("label_text-status-travel_speed"),
                ct.base_attr.travel_speed
            ),
            FontKind::S,
        );
        StatusWindow {
            rect,
            image,
            name_label,
            faction_label,
            lv_label,
            hp_label,
            sp_label,
            str_label,
            vit_label,
            dex_label,
            int_label,
            wil_label,
            cha_label,
            spd_label,
            carry_label,
            travel_speed_label,
            escape_click: false,
        }
    }
}

impl Window for StatusWindow {
    fn draw(
        &mut self,
        context: &mut Context<'_, '_, '_, '_>,
        _game: &Game,
        _anim: Option<(&Animation, u32)>,
    ) {
        draw_window_border(context, self.rect);
        self.image.draw(context);
        self.name_label.draw(context);
        self.faction_label.draw(context);
        self.lv_label.draw(context);
        self.hp_label.draw(context);
        self.sp_label.draw(context);
        self.str_label.draw(context);
        self.vit_label.draw(context);
        self.dex_label.draw(context);
        self.int_label.draw(context);
        self.wil_label.draw(context);
        self.cha_label.draw(context);
        self.spd_label.draw(context);
        self.carry_label.draw(context);
        self.travel_speed_label.draw(context);
    }
}

impl DialogWindow for StatusWindow {
    fn process_command(&mut self, command: &Command, _pa: &mut DoPlayerAction<'_>) -> DialogResult {
        check_escape_click!(self, command);

        match *command {
            Command::Cancel => DialogResult::Close,
            _ => DialogResult::Continue,
        }
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
                let (lv, adj) = chara.skill_level_with_adj(skill_kind);
                let skill_name =
                    TextCache::new(skill_kind.to_text(), FontKind::M, UI_CFG.color.normal_font);
                let skill_level = if adj == 0 {
                    format!("Lv. {}", lv)
                } else if adj < 0 {
                    format!("Lv. {} - {}", lv, -adj)
                } else {
                    format!("Lv. {} + {}", lv, adj)
                };
                let skill_level =
                    TextCache::new(skill_level, FontKind::M, UI_CFG.color.normal_font);
                let (_, skill_exp) = chara.skills.get_level_exp(skill_kind);
                let skill_exp = TextCache::new(
                    format!(
                        "({:0.1} %)",
                        skill_exp as f32 / SKILL_EXP_LVUP as f32 * 100.0
                    ),
                    FontKind::M,
                    UI_CFG.color.normal_font,
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
    fn draw(
        &mut self,
        context: &mut Context<'_, '_, '_, '_>,
        _game: &Game,
        _anim: Option<(&Animation, u32)>,
    ) {
        draw_window_border(context, self.rect);
        self.list.draw(context);
    }
}

impl DialogWindow for SkillWindow {
    fn process_command(&mut self, command: &Command, _pa: &mut DoPlayerAction<'_>) -> DialogResult {
        check_escape_click!(self, command);
        let command = command.relative_to(self.rect);

        self.list.process_command(&command);

        match command {
            Command::Cancel => DialogResult::Close,
            _ => DialogResult::Continue,
        }
    }
}

pub struct CharaTraitWindow {
    window: ListWithDescWindow<TextCache>,
    cid: CharaId,
    choice: u32,
}

impl CharaTraitWindow {
    fn new(gd: &GameData, cid: CharaId) -> Self {
        let rect: Rect = UI_CFG.info_window.rect.into();

        let items = gd
            .chara
            .get(cid)
            .traits
            .iter()
            .map(|(_origin, chara_trait)| {
                TextCache::new(chara_trait.to_text(), FontKind::M, UI_CFG.color.normal_font)
            })
            .collect();

        let mut window = CharaTraitWindow {
            window: ListWithDescWindow::new(
                rect,
                UI_CFG.chara_trait_window.column_pos.clone(),
                items,
            ),
            cid,
            choice: u32::max_value(),
        };

        window.update(gd, true);
        window
    }

    fn update(&mut self, gd: &GameData, init: bool) {
        let current_choice = self.window.list.get_current_choice();
        if current_choice != self.choice || init {
            self.choice = current_choice;

            if let Some((_origin, t)) = &gd.chara.get(self.cid).traits.get(current_choice as usize)
            {
                self.window
                    .text
                    .set_text(crate::text::desc::trait_description(t));
            }
        }
    }
}

impl Window for CharaTraitWindow {
    fn draw(
        &mut self,
        context: &mut Context<'_, '_, '_, '_>,
        game: &Game,
        anim: Option<(&Animation, u32)>,
    ) {
        self.window.draw(context, game, anim);
    }
}

impl DialogWindow for CharaTraitWindow {
    fn process_command(&mut self, command: &Command, pa: &mut DoPlayerAction<'_>) -> DialogResult {
        let result = self.window.process_command(command, pa);
        if let DialogResult::Continue = result {
            self.update(pa.gd(), false);
        }
        result
    }
}
