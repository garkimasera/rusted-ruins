use super::commonuse::*;
use super::widget::*;
use crate::config::{INPUT_CFG, UI_CFG};
use crate::text::ToText;

pub struct HelpWindow {
    rect: Rect,
    key_labels: Vec<LabelWidget>,
}

const COMMANDS: &[Command] = &[
    Command::OpenHelpWin,
    Command::OpenStatusWin,
    Command::OpenGameInfoWin,
    Command::OpenItemMenu,
    Command::OpenEquipWin,
    Command::EatItem,
    Command::DrinkItem,
    Command::DropItem,
    Command::OpenExitWin,
    Command::OpenCreationWin,
];

impl HelpWindow {
    pub fn new() -> HelpWindow {
        let cfg = &UI_CFG.help_window;
        let rect = cfg.rect.into();

        let key_labels = COMMANDS
            .iter()
            .enumerate()
            .map(|(i, c)| {
                let s = format!("{} {}", c.to_text(), INPUT_CFG.find_key(c));
                let mut r: Rect = cfg.key_label_start.into();
                r.offset(0, cfg.key_label_h * i as i32);
                LabelWidget::new(r, &s, FontKind::M)
            })
            .collect::<Vec<_>>();

        HelpWindow { rect, key_labels }
    }
}

impl Window for HelpWindow {
    fn draw(
        &mut self,
        context: &mut Context<'_, '_, '_, '_>,
        _game: &Game,
        _anim: Option<(&Animation, u32)>,
    ) {
        draw_window_border(context, self.rect);
        for label in &mut self.key_labels {
            label.draw(context);
        }
    }
}

impl DialogWindow for HelpWindow {
    fn process_command(&mut self, command: &Command, _pa: &mut DoPlayerAction<'_>) -> DialogResult {
        match command {
            Command::Cancel => DialogResult::Close,
            _ => DialogResult::Continue,
        }
    }
}
