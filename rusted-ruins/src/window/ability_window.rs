use super::commonuse::*;
use super::widget::*;
use crate::text::ToText;
use common::gamedata::*;
use rules::RULES;

/// Character active skill viewer
pub struct AbilityWindow {
    rect: Rect,
    cid: CharaId,
    list: ListWidget<(TextCache, TextCache)>,
    abilities: Vec<AbilityId>,
    escape_click: bool,
}

impl AbilityWindow {
    pub fn new(gd: &GameData, cid: CharaId) -> Self {
        let rect: Rect = UI_CFG.info_window.rect.into();
        let cfg = &UI_CFG.ability_window;

        let mut list =
            ListWidget::with_scroll_bar(cfg.list_rect, cfg.column_pos.clone(), cfg.n_row, false);

        let chara = gd.chara.get(cid);
        let mut abilities = Vec::new();
        let mut items = Vec::new();

        for (_, ability_id) in &chara.abilities {
            let ability = if let Some(ability) = RULES.abilities.get(ability_id) {
                ability
            } else {
                warn!("unknown active skill id \"{}\"", ability_id);
                continue;
            };

            abilities.push(ability_id.clone());

            let mut cost = if ability.cost_sp > 0 {
                format!("SP {} ", ability.cost_sp)
            } else {
                "".into()
            };

            if ability.cost_mp > 0 {
                cost.push_str(&format!("MP {} ", ability.cost_mp));
            }

            items.push((
                TextCache::new(ability_id.to_text(), FontKind::M, UI_CFG.color.normal_font),
                TextCache::new(cost, FontKind::M, UI_CFG.color.normal_font),
            ));
        }

        list.set_items(items);

        AbilityWindow {
            rect,
            cid,
            list,
            abilities,
            escape_click: false,
        }
    }
}

impl Window for AbilityWindow {
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

impl DialogWindow for AbilityWindow {
    fn process_command(
        &mut self,
        command: &Command,
        pa: &mut DoPlayerAction<'_, '_>,
    ) -> DialogResult {
        check_escape_click!(self, command);
        let command = command.relative_to(self.rect);

        if let Some(ListWidgetResponse::Select(i)) = self.list.process_command(&command) {
            if self.cid == CharaId::Player && pa.use_ability(&self.abilities[i as usize]) {
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
