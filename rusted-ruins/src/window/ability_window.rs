use super::commonuse::*;
use super::widget::*;
use crate::text::ToText;
use common::gamedata::*;
use rules::RULES;
use std::fmt::Write;

/// Character active skill viewer
pub struct AbilityWindow {
    rect: Rect,
    closer: DialogCloser,
    cid: CharaId,
    list: ListWidget<(TextCache, LabelWidget)>,
    abilities: Vec<AbilityId>,
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
                write!(cost, "MP {} ", ability.cost_mp).unwrap();
            }

            let cost_w = rect.width() - cfg.column_pos[1] as u32 - UI_CFG.vscroll_widget.width;
            let cost_label = LabelWidget::new(
                Rect::new(0, 0, cost_w as u32, UI_CFG.list_widget.h_row_default),
                cost,
                FontKind::M,
            )
            .right();

            items.push((
                TextCache::new(ability_id.to_text(), FontKind::M, UI_CFG.color.normal_font),
                cost_label,
            ));
        }

        list.set_items(items);

        AbilityWindow {
            rect,
            closer: DialogCloser::new(rect),
            cid,
            list,
            abilities,
        }
    }
}

impl Window for AbilityWindow {
    fn draw(
        &mut self,
        context: &mut Context<'_, '_, '_, '_>,
        _game: &Game,
        _anim: Option<(&Animation, u32)>,
    ) {
        self.closer.draw(context);
        draw_window_border(context, self.rect);
        self.list.draw(context);
    }
}

impl DialogWindow for AbilityWindow {
    fn process_command(&mut self, command: &Command, pa: &mut DoPlayerAction<'_>) -> DialogResult {
        closer!(self, command);
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
