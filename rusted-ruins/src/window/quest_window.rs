
use sdl2::rect::Rect;
use crate::text::ToText;
use crate::context::*;
use crate::window::{Window, DialogWindow, DialogResult, WindowDrawMode};
use crate::game::{Game, Animation, Command, DoPlayerAction};
use crate::game::quest::available_quests;
use crate::eventhandler::InputMode;
use crate::config::UI_CFG;
use crate::draw::border::draw_rect_border;
use super::widget::*;

pub struct QuestWindow {
    rect: Rect,
    list: TextListWidget,
}

impl QuestWindow {
    pub fn new(game: &Game) -> QuestWindow {
        let rect = UI_CFG.quest_window.rect.into();
        let mut w = QuestWindow {
            rect,
            list: TextListWidget::new(
                (0i32, 0i32, rect.w as u32, rect.h as u32),
                vec![6],
                UI_CFG.quest_window.n_row,
                26,
                true,
                false),
        };
        w.update(game);
        w
    }

    pub fn update(&mut self, game: &Game) {

        let rows: Vec<TextCache> = available_quests(&game.gd).iter()
            .map(|quest| {
                let text = quest.to_text();
                TextCache::one(text, FontKind::M, UI_CFG.color.normal_font.into())
            })
            .collect();

        self.list.set_items(rows);
    }
}

impl Window for QuestWindow {
    
    fn draw(
        &mut self, context: &mut Context, game: &Game, anim: Option<(&Animation, u32)>) {
        
        draw_rect_border(context, self.rect);
    }
}

impl DialogWindow for QuestWindow {
    fn process_command(&mut self, command: &Command, pa: &mut DoPlayerAction) -> DialogResult {
        if let Some(response) = self.list.process_command(&command) {
            match response {
                ListWidgetResponse::Select(i) => { // Any item is selected
                }
                ListWidgetResponse::PageChanged => {
                }
                _ => (),
            }
            return DialogResult::Continue;
        }

        match *command {
            Command::Cancel => {
                DialogResult::Close
            },
            _ => DialogResult::Continue,
        }
    }

    fn mode(&self) -> InputMode {
        InputMode::Dialog
    }

    fn draw_mode(&self) -> WindowDrawMode {
        WindowDrawMode::SkipUnderWindows
    }
}

