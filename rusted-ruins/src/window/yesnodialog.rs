
use sdl2::rect::Rect;
use config::UI_CFG;
use game::DoPlayerAction;
use super::commonuse::*;
use super::widget::*;
use super::textwindow::TextWindow;
use super::choosewindow::ChooseWindow;
use super::winpos::{WindowPos, WindowHPos, WindowVPos};
use sdlvalues::FontKind;

pub struct YesNoDialog {
    text_win: TextWindow,
    choose_win: ChooseWindow,
    action_callback: Box<FnMut(&mut DoPlayerAction, u32) -> DialogResult + 'static>
}

impl YesNoDialog {
    pub fn new<F>(msg: &str, f: F) -> YesNoDialog
        where F: FnMut(&mut DoPlayerAction, u32) -> DialogResult + 'static {

        let rect = Rect::new(100, 0, 0, 0);
        let text_win = TextWindow::new(rect, msg);
        let winpos = WindowPos::new(
            WindowHPos::RightX(rect.right()),
            WindowVPos::TopMargin(rect.bottom() + UI_CFG.gap_len_between_dialogs));
        YesNoDialog {
            text_win: text_win,
            choose_win: ChooseWindow::with_yesno(winpos, None),
            action_callback: Box::new(f),
        }
    }
}

impl Window for YesNoDialog {
    
    fn redraw(
        &mut self, canvas: &mut WindowCanvas, game: &Game, sv: &mut SdlValues,
        anim: Option<(&Animation, u32)>) {

        self.text_win.redraw(canvas, game, sv, anim);
        let rect = self.text_win.get_rect();
        let winpos = WindowPos::new(
            WindowHPos::RightX(rect.right()),
            WindowVPos::TopMargin(rect.bottom() + UI_CFG.gap_len_between_dialogs));
        self.choose_win.set_winpos(winpos);
        self.choose_win.redraw(canvas, game, sv, anim);
    }
}

impl DialogWindow for YesNoDialog {
    fn process_command(&mut self, command: &Command, pa: &mut DoPlayerAction) -> DialogResult {
        match *command {
            Command::Cancel => { return DialogResult::Close; },
            _ => (),
        }

        match self.choose_win.process_command(command, pa) {
            DialogResult::CloseWithValue(v) => { // An choice is choosed
                let n = *v.downcast::<u32>().unwrap();
//                return (self.action_callback)(&mut pa, n);
            },
            _ => (),
        }
        DialogResult::Continue
    }

    fn mode(&self) -> InputMode {
        InputMode::Dialog
    }
}

