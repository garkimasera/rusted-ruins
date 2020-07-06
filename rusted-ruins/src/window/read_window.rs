use super::commonuse::*;
use super::widget::*;

pub struct ReadWindow {
    rect: Rect,
    escape_click: bool,
    label: LabelWidget,
    text: Vec<String>,
}

impl ReadWindow {
    pub fn new(title: &str) -> ReadWindow {
        let c = &UI_CFG.read_window;
        let rect: Rect = c.rect.into();
        let text = crate::text::readable::readable_txt(title);
        let label = LabelWidget::wrapped(
            Rect::new(0, 0, rect.width(), rect.height()),
            &text[0],
            FontKind::Talk,
            c.text_wrap_width,
        );

        ReadWindow {
            rect,
            escape_click: false,
            label,
            text,
        }
    }
}

impl Window for ReadWindow {
    fn draw(&mut self, context: &mut Context, _game: &Game, _anim: Option<(&Animation, u32)>) {
        draw_window_border(context, self.rect);
        self.label.draw(context);
    }
}

impl DialogWindow for ReadWindow {
    fn process_command(&mut self, command: &Command, _pa: &mut DoPlayerAction) -> DialogResult {
        check_escape_click!(self, command);

        DialogResult::Continue
    }

    fn mode(&self) -> InputMode {
        InputMode::Dialog
    }
}
