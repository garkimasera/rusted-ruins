
use super::commonuse::*;
use super::widget::*;
use sdlvalues::FontKind;
use config::UI_CFG;
use game::talk::*;
use text;

pub struct TalkWindow {
    rect: Rect,
    text: String,
    talk_status: TalkStatus,
    current_line: usize,
    n_line: usize,
    label: LineSpecifiedLabelWidget,
}

impl TalkWindow {
    pub fn new(talk_status: TalkStatus) -> TalkWindow {
        let rect: Rect = UI_CFG.talk_window.rect.into();
        let label = LineSpecifiedLabelWidget::new(
            Rect::new(0, 0, rect.width(), rect.height()),
            &[""], FontKind::M, UI_CFG.talk_window.n_default_line);
        let mut talk_window = TalkWindow {
            rect: rect,
            text: "".to_owned(),
            current_line: 0,
            n_line: 0,
            talk_status: talk_status,
            label: label,
        };
        talk_window.update_text();
        talk_window
    }

    fn update_text(&mut self) {
        let mut lines: Vec<&str> = Vec::new();
        let s = text::talk_txt(self.talk_status.get_text());
        self.n_line = s.lines().count();
        for line in s.lines().skip(self.current_line).
            take(UI_CFG.talk_window.n_default_line) {
            lines.push(line);
        }
        self.label.set_text(&lines);
    }
}

impl Window for TalkWindow {
    fn redraw(
        &mut self, canvas: &mut WindowCanvas, _game: &Game, sv: &mut SdlValues,
        _anim: Option<(&Animation, u32)>) {

        draw_rect_border(canvas, self.rect);
        self.label.draw(canvas, sv);
    }
}

impl DialogWindow for TalkWindow {
    fn process_command(&mut self, command: &Command, pa: &mut DoPlayerAction) -> DialogResult {
        match *command {
            Command::Enter => {
                // If all text of the section has been displayed,
                // proceeds the talk to next section
                if self.current_line + UI_CFG.talk_window.n_default_line >= self.n_line {
                    match self.talk_status.proceed(pa, None) {
                        TalkResult::End => { DialogResult::Close },
                        TalkResult::Continue => { DialogResult::Continue },
                    }
                } else {
                    self.current_line += UI_CFG.talk_window.n_default_line;
                    self.update_text();
                    DialogResult::Continue
                }
            },
            _ => DialogResult::Continue,
        }
    }

    fn mode(&self) -> InputMode {
        InputMode::Dialog
    }
}
