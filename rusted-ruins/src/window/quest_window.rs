use super::commonuse::*;
use super::msg_dialog::MsgDialog;
use super::widget::*;
use crate::draw::border::draw_window_border;
use crate::game::quest::available_quests;
use crate::text::ToText;
use common::gamedata::Quest;

pub struct QuestWindow {
    rect: Rect,
    list: TextListWidget,
    description: LabelWidget,
    dialog: Option<MsgDialog>,
    escape_click: bool,
}

impl QuestWindow {
    pub fn new(game: &Game<'_>) -> QuestWindow {
        let rect = UI_CFG.quest_window.rect.into();
        let mut w = QuestWindow {
            rect,
            list: TextListWidget::new(
                (0i32, 0i32, rect.w as u32, rect.h as u32),
                vec![6],
                UI_CFG.quest_window.n_row,
                true,
            ),
            dialog: None,
            description: LabelWidget::wrapped(
                (
                    0i32,
                    (UI_CFG.quest_window.n_row as i32 + 1)
                        * UI_CFG.list_widget.h_row_default as i32,
                    rect.width(),
                    0,
                ),
                "",
                FontKind::M,
                rect.width(),
            ),
            escape_click: false,
        };
        w.update(game);
        w
    }

    pub fn update(&mut self, game: &Game<'_>) {
        let rows: Vec<TextCache> = available_quests(&game.gd)
            .iter()
            .map(|quest| {
                let text = quest.to_text();
                TextCache::new(text, FontKind::M, UI_CFG.color.normal_font.into())
            })
            .collect();

        let n_item = rows.len();
        self.list.set_items(rows);

        if n_item == 0 {
            self.description.set_text("");
        } else {
            let q = &available_quests(&game.gd)[self.list.get_current_choice() as usize];
            self.description.set_text(&quest_decription_text(q));
        }
    }
}

impl Window for QuestWindow {
    fn draw(
        &mut self,
        context: &mut Context<'_, '_, '_, '_>,
        game: &Game<'_>,
        anim: Option<(&Animation, u32)>,
    ) {
        draw_window_border(context, self.rect);
        self.list.draw(context);
        self.description.draw(context);
        if let Some(dialog) = self.dialog.as_mut() {
            dialog.draw(context, game, anim);
        }
    }
}

impl DialogWindow for QuestWindow {
    fn process_command(
        &mut self,
        command: &Command,
        pa: &mut DoPlayerAction<'_, '_>,
    ) -> DialogResult {
        check_escape_click!(self, command, false);

        if let Some(dialog) = self.dialog.as_mut() {
            match dialog.process_command(command, pa) {
                DialogResult::Close => {
                    self.dialog = None;
                }
                DialogResult::CloseWithValue(v) => {
                    let n = if let DialogCloseValue::Index(n) = v {
                        n
                    } else {
                        unreachable!()
                    };
                    self.dialog = None;
                    if n == 0 {
                        // Undertake quest
                        pa.undertake_quest(n);
                        self.update(pa.game())
                    }
                }
                _ => (),
            }
            return DialogResult::Continue;
        }

        let command = command.relative_to(self.rect);
        if let Some(response) = self.list.process_command(&command) {
            match response {
                ListWidgetResponse::Select(_) => {
                    // Any item is selected
                    self.dialog = Some(MsgDialog::with_yesno(
                        &crate::text::ui_txt("dialog-undertake_quest"),
                        |_, n| DialogResult::CloseWithValue(DialogCloseValue::Index(n)),
                    ));
                }
                ListWidgetResponse::Scrolled => {}
                _ => (),
            }
            return DialogResult::Continue;
        }

        match command {
            Command::Cancel => DialogResult::Close,
            _ => DialogResult::Continue,
        }
    }

    fn draw_mode(&self) -> WindowDrawMode {
        WindowDrawMode::SkipUnderWindows
    }
}

fn quest_decription_text(quest: &Quest) -> String {
    match quest {
        Quest::SlayMonsters { idx, goal, .. } => {
            misc_txt_format!("desc-quest-slay_monsters"; monster=idx, n=goal)
        }
    }
}
