use super::commonuse::*;
use super::widget::*;
use crate::config::UI_CFG;
use crate::context::textrenderer::FontKind;
use crate::text::{ui_txt, ToText};
use common::gamedata::*;

/// Faction viewer
pub struct FactionWindow {
    rect: Rect,
    list: ListWidget<(TextCache, LabelWidget)>,
    escape_click: bool,
}

impl FactionWindow {
    pub fn new(gd: &GameData) -> FactionWindow {
        let rect: Rect = UI_CFG.info_window.rect.into();
        let cfg = &UI_CFG.faction_window;

        let column1_width = rect.width() - cfg.column_pos[1] as u32 - UI_CFG.vscroll_widget.width;
        let header = (
            TextCache::new(
                ui_txt("list_header-faction"),
                FontKind::M,
                UI_CFG.color.normal_font.into(),
            ),
            LabelWidget::new(
                Rect::new(0, 0, column1_width, UI_CFG.list_widget.h_row_default),
                &ui_txt("list_header-relation"),
                FontKind::M,
            ),
        );
        let mut list = ListWidget::with_header(
            cfg.list_rect,
            cfg.column_pos.clone(),
            cfg.n_row,
            false,
            header,
        );

        let items: Vec<_> = gd
            .faction
            .iter()
            .filter_map(|(id, relation)| {
                if id.as_str().starts_with('!') {
                    return None;
                }

                let faction =
                    TextCache::new(id.to_text(), FontKind::M, UI_CFG.color.normal_font.into());
                let relation = format!("{}", Into::<i16>::into(*relation));
                let relation = LabelWidget::new(
                    Rect::new(0, 0, column1_width, UI_CFG.list_widget.h_row_default),
                    &relation,
                    FontKind::M,
                )
                .right();
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
