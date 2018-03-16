
use super::commonuse::*;
use super::widget::*;
use std::borrow::Cow;
use common::objholder::CharaTemplateIdx;
use common::basic::TILE_SIZE;
use sdlvalues::FontKind;
use config::UI_CFG;
use game::talk::*;
use super::miscwindow::ImageWindow;
use text;

pub struct TalkWindow {
    rect: Rect,
    talk_manager: TalkManager,
    label: LineSpecifiedLabelWidget,
    image_window: ImageWindow,
    msg_text: MsgText,
}

impl TalkWindow {
    pub fn new(talk_manager: TalkManager, chara_template_idx: CharaTemplateIdx) -> TalkWindow {
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
            talk_manager: talk_manager,
            label: label,
            image_window: ImageWindow::chara(rect_image_window, chara_template_idx),
            msg_text: MsgText::default(),
        };
        talk_window.update_page(true);
        talk_window
    }

    fn update_page(&mut self, section_changed: bool) {
        if section_changed {
            let text_id = self.talk_manager.get_text();
            self.msg_text = MsgText::new(&*text_id);
        }
        self.label.set_text(self.msg_text.page_lines());
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
                if self.msg_text.is_final_page() {
                    match self.talk_manager.proceed(pa) {
                        TalkResult::End => DialogResult::Close,
                        TalkResult::NoChange => DialogResult::Continue,
                        TalkResult::Continue => {
                            self.update_page(true);
                            DialogResult::Continue
                        },
                    }
                } else {
                    self.msg_text.next_page();
                    self.update_page(false);
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

/// Manage lines of text message
#[derive(Default)]
struct MsgText {
    lines: Vec<Cow<'static, str>>,
    current_line: usize,
    n_default_line: usize,
}

impl MsgText {
    fn new(text_id: &str) -> MsgText {
        let mut lines: Vec<Cow<'static, str>> = Vec::new();

        if let Some(s) = text::talk_txt_checked(text_id) {
            for line in s.lines() {
                lines.push(line.into());
            }
        } else {
            lines.push(text_id.to_owned().into());
        }
        
        MsgText {
            lines: lines,
            current_line: 0,
            n_default_line: UI_CFG.talk_window.n_default_line,
        }
    }

    fn page_lines(&self) -> &[Cow<'static, str>] {
        let e = ::std::cmp::min(self.n_line(), self.current_line + self.n_default_line);
        &self.lines[self.current_line..e]
    }

    fn n_line(&self) -> usize {
        self.lines.len()
    }

    fn next_page(&mut self) {
        self.current_line += self.n_default_line;
    }

    fn is_final_page(&self) -> bool {
        self.current_line + self.n_default_line >= self.n_line()
    }
}

