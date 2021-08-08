use super::commonuse::*;
use super::widget::*;
use crate::config::UI_CFG;
use crate::context::textrenderer::FontKind;
use crate::text::ToText;
use common::gamedata::*;

/// Faction viewer
pub struct FactionWindow {
    rect: Rect,
    list: ListWidget<(TextCache, TextCache)>,
    escape_click: bool,
}

impl FactionWindow {
    pub fn new(gd: &GameData) -> FactionWindow {
        let rect: Rect = UI_CFG.info_window.rect.into();
        let cfg = &UI_CFG.faction_window;

        let mut list =
            ListWidget::with_scroll_bar(cfg.list_rect, cfg.column_pos.clone(), cfg.n_row, false);

        let items: Vec<_> = gd
            .faction
            .iter()
            .filter_map(|(id, relation)| {
                if id.as_str().starts_with('!') {
                    return None;
                }

                let faction =
                    TextCache::one(id.to_text(), FontKind::M, UI_CFG.color.normal_font.into());
                let relation = TextCache::one(
                    &format!("{}", Into::<i16>::into(*relation)),
                    FontKind::M,
                    UI_CFG.color.normal_font.into(),
                );
                Some((faction, relation))
            })
            .collect();

        list.set_items(items);

        FactionWindow {
            rect,
            list,
            escape_click: false,
        }
    }
}

impl Window for FactionWindow {
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

impl DialogWindow for FactionWindow {
    fn process_command(
        &mut self,
        command: &Command,
        _pa: &mut DoPlayerAction<'_, '_>,
    ) -> DialogResult {
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
