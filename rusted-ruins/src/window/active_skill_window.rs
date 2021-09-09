use super::commonuse::*;
use super::widget::*;
use crate::text::ToText;
use common::gamedata::*;
use rules::RULES;

/// Character active skill viewer
pub struct ActiveSkillWindow {
    rect: Rect,
    cid: CharaId,
    list: ListWidget<(TextCache, TextCache)>,
    active_skills: Vec<ActiveSkillId>,
    escape_click: bool,
}

impl ActiveSkillWindow {
    pub fn new(gd: &GameData, cid: CharaId) -> Self {
        let rect: Rect = UI_CFG.info_window.rect.into();
        let cfg = &UI_CFG.active_skill_window;

        let mut list =
            ListWidget::with_scroll_bar(cfg.list_rect, cfg.column_pos.clone(), cfg.n_row, false);

        let chara = gd.chara.get(cid);
        let mut active_skills = Vec::new();
        let mut items = Vec::new();

        for (_, active_skill_id) in &chara.active_skills {
            let active_skill = if let Some(active_skill) = RULES.active_skills.get(active_skill_id)
            {
                active_skill
            } else {
                warn!("unknown active skill id \"{}\"", active_skill_id);
                continue;
            };

            active_skills.push(active_skill_id.clone());

            let mut cost = if active_skill.cost_sp > 0 {
                format!("SP {} ", active_skill.cost_sp)
            } else {
                "".into()
            };

            if active_skill.cost_mp > 0 {
                cost.push_str(&format!("MP {} ", active_skill.cost_mp));
            }

            items.push((
                TextCache::new(
                    active_skill_id.to_text(),
                    FontKind::M,
                    UI_CFG.color.normal_font.into(),
                ),
                TextCache::new(cost, FontKind::M, UI_CFG.color.normal_font.into()),
            ));
        }

        list.set_items(items);

        ActiveSkillWindow {
            rect,
            cid,
            list,
            active_skills,
            escape_click: false,
        }
    }
}

impl Window for ActiveSkillWindow {
    fn draw(
        &mut self,
        context: &mut Context<'_, '_, '_, '_>,
        _game: &Game<'_>,
        _anim: Option<(&Animation, u32)>,
    ) {
        draw_window_border(context, self.rect);
        self.list.draw(context);
    }
}

impl DialogWindow for ActiveSkillWindow {
    fn process_command(
        &mut self,
        command: &Command,
        pa: &mut DoPlayerAction<'_, '_>,
    ) -> DialogResult {
        check_escape_click!(self, command);
        let command = command.relative_to(self.rect);

        if let Some(ListWidgetResponse::Select(i)) = self.list.process_command(&command) {
            if self.cid == CharaId::Player && pa.use_active_skill(&self.active_skills[i as usize]) {
                return DialogResult::Close;
            }
            return DialogResult::Continue;
        }

        match command {
            Command::Cancel => DialogResult::Close,
            _ => DialogResult::Continue,
        }
    }
}
