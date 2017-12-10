
use common::gamedata::chara::CharaTalk;
use super::commonuse::*;
use super::widget::*;
use sdlvalues::FontKind;
use config::UI_CFG;
use game::TalkStatus;
use text;

pub struct TalkWindow {
    rect: Rect,
    text: String,
    talk_status: TalkStatus,
    current_line: usize,
}

impl TalkWindow {
    pub fn new(talk_status: TalkStatus) -> TalkWindow {
        println!("{}", talk_status.get_text());
        let rect = UI_CFG.talk_window.rect.into();
        TalkWindow {
            rect: rect,
            text: "".to_owned(),
            current_line: 0,
            talk_status: talk_status,
        }
    }

    fn set_text<T: Into<String>>(&mut self, text: T) {
        let text: String = text.into();
        
        println!("{}", text::talk_txt(&text));
        // for line in text.lines().skip(self.current_line).take(UI_CFG. {
            
        // }
    }
}



impl Window for TalkWindow {
    fn redraw(
        &mut self, canvas: &mut WindowCanvas, _game: &Game, sv: &mut SdlValues,
        _anim: Option<(&Animation, u32)>) {

        draw_rect_border(canvas, self.rect);
    }
}

impl DialogWindow for TalkWindow {
    fn process_command(&mut self, command: Command, pa: DoPlayerAction) -> DialogResult {
        match command {
            Command::Cancel => {
                DialogResult::Close
            },
            _ => DialogResult::Continue,
        }
    }

    fn mode(&self) -> InputMode {
        InputMode::Dialog
    }
}
