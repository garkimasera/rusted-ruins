
use super::commonuse::*;
use super::widget::*;
use common::objholder::CharaTemplateIdx;
use common::basic::TILE_SIZE;
use sdlvalues::FontKind;
use config::UI_CFG;
use game::talk::*;
use super::miscwindow::ImageWindow;
use text;

pub struct TalkWindow {
    rect: Rect,
    text: String,
    talk_status: TalkStatus,
    current_line: usize,
    n_line: usize,
    label: LineSpecifiedLabelWidget,
    image_window: ImageWindow,
}

impl TalkWindow {
    pub fn new(talk_status: TalkStatus, chara_template_idx: CharaTemplateIdx) -> TalkWindow {
        let rect: Rect = UI_CFG.talk_window.rect.into();
        let label = LineSpecifiedLabelWidget::new(
            Rect::new(0, 0, rect.width(), rect.height()),
            &[""], FontKind::M, UI_CFG.talk_window.n_default_line);
        let rect_image_window = Rect::new(
            rect.x + UI_CFG.talk_window.image_window_pos_x,
            rect.y + UI_CFG.talk_window.image_window_pos_y,
            TILE_SIZE,
            TILE_SIZE * 2);
        let mut talk_window = TalkWindow {
            rect: rect,
            text: "".to_owned(),
            current_line: 0,
            n_line: 0,
            talk_status: talk_status,
            label: label,
            image_window: ImageWindow::chara(rect_image_window, chara_template_idx),
        };
        talk_window.update_text();
        talk_window
    }

    fn update_text(&mut self) {
        let text_id = self.talk_status.get_text();
        let s = text::talk_txt(&*text_id);
        self.n_line = s.lines().count();
        let mut lines: Vec<&str> = Vec::new();
        for line in s.lines().skip(self.current_line).
            take(UI_CFG.talk_window.n_default_line) {
            lines.push(line);
        }
        self.label.set_text(&lines);
    }
}

impl Window for TalkWindow {
    fn draw(
        &mut self, canvas: &mut WindowCanvas, game: &Game, sv: &mut SdlValues,
        anim: Option<(&Animation, u32)>) {

        self.image_window.draw(canvas, game, sv, anim);
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
                    match self.talk_status.proceed(pa) {
                        TalkResult::End => DialogResult::Close,
                        TalkResult::NoChange => DialogResult::Continue,
                        TalkResult::Continue => {
                            self.update_text();
                            DialogResult::Continue
                        },
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
