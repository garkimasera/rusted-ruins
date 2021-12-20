use super::commonuse::*;
use super::widget::*;
use crate::draw::border::draw_window_border;
use crate::game::quest::{available_quests, reportable_quests};
use crate::text::obj_txt;
use crate::text::{quest_txt_checked, ui_txt, ToText};
use common::gamedata::TownQuestKind;
use common::gamedata::TownQuestState;
use common::gamedata::{GameData, TownQuest};
use common::gobj;

enum QuestWindowMode {
    List,
    Offer {
        selected: Vec<bool>,
        take_button: ButtonWidget,
        cancel_button: ButtonWidget,
    },
    Report {
        selected: Vec<bool>,
        report_button: ButtonWidget,
        cancel_button: ButtonWidget,
        reportable_quests: Vec<u32>,
    },
}

pub struct QuestWindow {
    rect: Rect,
    mode: QuestWindowMode,
    list: ListWidget<(IconIdx, TextCache)>,
    desc: LabelWidget,
    line_y: i32,
    escape_click: bool,
}

impl QuestWindow {
    fn new(gd: &GameData, mode: QuestWindowMode, rows: Vec<(IconIdx, TextCache)>) -> QuestWindow {
        let mut rect: Rect = UI_CFG.info_window.rect.into();
        rect.h = UI_CFG.quest_window.h as _;

        let list_rect = Rect::new(
            0,
            0,
            rect.w as _,
            (UI_CFG.list_widget.h_row_default * UI_CFG.quest_window.n_row) as _,
        );

        let mut list = ListWidget::with_scroll_bar(
            list_rect,
            UI_CFG.quest_window.column_pos.clone(),
            UI_CFG.quest_window.n_row,
            false,
        );
        list.set_items(rows);

        let desc_rect = Rect::new(
            0,
            list_rect.h + 2,
            rect.width(),
            rect.height() - list_rect.height() - 2,
        );
        let desc = LabelWidget::wrapped(desc_rect, "", FontKind::S, desc_rect.width());

        let mut window = QuestWindow {
            rect,
            mode,
            list,
            desc,
            line_y: list_rect.h,
            escape_click: false,
        };
        window.update_desc_text(gd, 0);
        window
    }

    pub fn new_list(gd: &GameData) -> QuestWindow {
        let rows: Vec<(IconIdx, TextCache)> = gd
            .quest
            .town_quests
            .iter()
            .map(|(state, quest)| {
                let icon = match *state {
                    TownQuestState::Active => IconIdx::empty(),
                    TownQuestState::Reportable => IconIdx::checked(),
                };
                (
                    icon,
                    TextCache::new(quest.to_text(), FontKind::M, UI_CFG.color.normal_font),
                )
            })
            .collect();

        let mode = QuestWindowMode::List;

        Self::new(gd, mode, rows)
    }

    pub fn new_offer(gd: &GameData) -> QuestWindow {
        let rows: Vec<(IconIdx, TextCache)> = available_quests(gd)
            .iter()
            .map(|quest| {
                (
                    IconIdx::empty(),
                    TextCache::new(quest.to_text(), FontKind::M, UI_CFG.color.normal_font),
                )
            })
            .collect();

        let mode = QuestWindowMode::Offer {
            selected: vec![false; rows.len()],
            take_button: ButtonWidget::new(
                UI_CFG.quest_window.ok_button_rect,
                ui_txt("button_text-quest-take"),
                FontKind::M,
            ),
            cancel_button: ButtonWidget::new(
                UI_CFG.quest_window.cancel_button_rect,
                ui_txt("button_text-cancel"),
                FontKind::M,
            ),
        };

        Self::new(gd, mode, rows)
    }

    pub fn new_report(gd: &GameData) -> QuestWindow {
        let reportable_quests = reportable_quests(gd);
        let rows: Vec<(IconIdx, TextCache)> = reportable_quests
            .iter()
            .map(|i| {
                (
                    IconIdx::checked(),
                    TextCache::new(
                        gd.quest.town_quests[*i as usize].1.to_text(),
                        FontKind::M,
                        UI_CFG.color.normal_font,
                    ),
                )
            })
            .collect();

        let mode = QuestWindowMode::Report {
            selected: vec![true; rows.len()],
            report_button: ButtonWidget::new(
                UI_CFG.quest_window.ok_button_rect,
                ui_txt("button_text-quest-report"),
                FontKind::M,
            ),
            cancel_button: ButtonWidget::new(
                UI_CFG.quest_window.cancel_button_rect,
                ui_txt("button_text-cancel"),
                FontKind::M,
            ),
            reportable_quests,
        };

        Self::new(gd, mode, rows)
    }

    pub fn update_desc_text(&mut self, gd: &GameData, i: u32) {
        let desc_text = match &self.mode {
            QuestWindowMode::List { .. } => {
                if let Some((_, town_quest)) = &gd.quest.town_quests.get(i as usize) {
                    town_quest_desc_text(town_quest)
                } else {
                    return;
                }
            }
            QuestWindowMode::Offer { .. } => {
                if let Some(town_quest) = &available_quests(gd).get(i as usize) {
                    town_quest_desc_text(town_quest)
                } else {
                    return;
                }
            }
            QuestWindowMode::Report {
                reportable_quests, ..
            } => {
                if let Some(i) = reportable_quests.get(i as usize) {
                    let town_quest = &gd.quest.town_quests[*i as usize].1;
                    town_quest_desc_text(town_quest)
                } else {
                    return;
                }
            }
        };
        self.desc.set_text(desc_text);
    }
}

impl Window for QuestWindow {
    fn draw(
        &mut self,
        context: &mut Context<'_, '_, '_, '_>,
        _game: &Game<'_>,
        _anim: Option<(&Animation, u32)>,
    ) {
        draw_window_border(context, self.rect);
        self.list.draw(context);

        context.draw_line(
            (0, self.line_y),
            (self.rect.w, self.line_y),
            UI_CFG.color.border_light,
        );
        context.draw_line(
            (0, self.line_y + 1),
            (self.rect.w, self.line_y + 1),
            UI_CFG.color.border_dark,
        );

        match &mut self.mode {
            QuestWindowMode::Offer {
                take_button,
                cancel_button,
                selected,
            } => {
                if selected.iter().any(|state| *state) {
                    take_button.draw(context);
                }
                cancel_button.draw(context);
            }
            QuestWindowMode::Report {
                report_button,
                cancel_button,
                selected,
                ..
            } => {
                if selected.iter().any(|state| *state) {
                    report_button.draw(context);
                }
                cancel_button.draw(context);
            }
            _ => (),
        }

        self.desc.draw(context);
    }
}

impl DialogWindow for QuestWindow {
    fn process_command(
        &mut self,
        command: &Command,
        pa: &mut DoPlayerAction<'_, '_>,
    ) -> DialogResult {
        check_escape_click!(
            self,
            command,
            matches!(self.mode, QuestWindowMode::List { .. })
        );
        let command = command.relative_to(self.rect);

        match &mut self.mode {
            QuestWindowMode::Offer {
                take_button,
                cancel_button,
                selected,
            } => {
                if selected.iter().any(|state| *state)
                    && take_button.process_command(&command) == Some(true)
                {
                    let target_quests = selected
                        .iter()
                        .enumerate()
                        .filter_map(|(i, state)| state.then(|| i))
                        .collect();
                    pa.undertake_quests(target_quests);

                    return DialogResult::Close;
                }
                if cancel_button.process_command(&command) == Some(true) {
                    return DialogResult::Close;
                }
            }
            QuestWindowMode::Report {
                report_button,
                cancel_button,
                selected,
                reportable_quests,
            } => {
                if selected.iter().any(|state| *state)
                    && report_button.process_command(&command) == Some(true)
                {
                    let target_quests = selected
                        .iter()
                        .enumerate()
                        .filter_map(|(i, state)| state.then(|| reportable_quests[i] as usize))
                        .collect();
                    pa.report_quests(target_quests);

                    return DialogResult::Close;
                }
                if cancel_button.process_command(&command) == Some(true) {
                    return DialogResult::Close;
                }
            }
            _ => (),
        }

        match self.list.process_command(&command) {
            Some(ListWidgetResponse::Select(i)) => match &mut self.mode {
                QuestWindowMode::Offer { selected, .. }
                | QuestWindowMode::Report { selected, .. } => {
                    selected[i as usize] = !selected[i as usize];
                    self.list.get_item_mut(i).unwrap().0 = if selected[i as usize] {
                        IconIdx::checked()
                    } else {
                        IconIdx::empty()
                    };
                }
                _ => (),
            },
            Some(ListWidgetResponse::SelectionChanged(i)) => {
                self.update_desc_text(pa.gd(), i);
            }
            _ => (),
        }

        DialogResult::Continue
    }

    fn draw_mode(&self) -> WindowDrawMode {
        if let QuestWindowMode::List { .. } = self.mode {
            WindowDrawMode::Normal
        } else {
            WindowDrawMode::SkipUnderWindows
        }
    }
}

fn town_quest_desc_text(quest: &TownQuest) -> String {
    let desc_text_id = format!("{}-desc", &quest.text_id);
    let mut text = quest_txt_checked(&desc_text_id).unwrap_or_else(|| "".into());

    text.push_str("\n\n");
    match &quest.kind {
        TownQuestKind::ItemDelivering { items } => {
            text.push_str(&ui_txt("label_text-quest-delivery_chest"));
            text.push_str(": ");
            let len = items.len();
            for (i, &(item_idx, n)) in items.iter().enumerate() {
                let id = gobj::idx_to_id(item_idx);
                text.push_str(&format!("{}x{}", obj_txt(id), n));
                if i < len - 1 {
                    text.push(',');
                }
            }
        }
        _ => todo!(),
    }

    text.push('\n');
    text.push_str(&ui_txt("label_text-quest-reward"));
    text.push_str(": ");

    if quest.reward.money > 0 {
        text.push_str(&format!("{} gold", quest.reward.money));
        if !quest.reward.items.is_empty() {
            text.push(',');
        }
    }

    let len = quest.reward.items.len();
    for (i, &(item_idx, n)) in quest.reward.items.iter().enumerate() {
        let id = gobj::idx_to_id(item_idx);
        text.push_str(&format!("{}x{}", obj_txt(id), n));
        if i < len - 1 {
            text.push(',');
        }
    }

    text
}
