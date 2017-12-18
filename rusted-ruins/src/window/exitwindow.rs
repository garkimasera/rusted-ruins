
use sdl2::rect::Rect;
use config::UI_CFG;
use super::commonuse::*;
use super::widget::*;
use super::choosewindow::ChooseWindow;
use super::winpos::{WindowPos, WindowHPos, WindowVPos};
use sdlvalues::FontKind;
use text;

pub struct ExitWindow {
    rect: Rect,
    label: LabelWidget,
    choose_win: ChooseWindow,
}

impl ExitWindow {
    pub fn new() -> ExitWindow {
        let choices = vec!["Yes".to_owned(), "No".to_owned()];
        let choices_a = choices.clone();
        let rect: Rect = UI_CFG.exit_window.rect.into();
        let winpos = WindowPos::new(
            WindowHPos::RightX(rect.right()),
            WindowVPos::TopMargin(rect.bottom()));
        ExitWindow {
            rect: rect,
            label: LabelWidget::wrapped(
                (0, 0, rect.w as u32, 0), text::ui_txt("dialog.exit"), FontKind::M, rect.w as u32),
            choose_win: ChooseWindow::new(winpos, choices_a, None),
        }
    }
}

impl Window for ExitWindow {
    
    fn redraw(
        &mut self, canvas: &mut WindowCanvas, game: &Game, sv: &mut SdlValues,
        anim: Option<(&Animation, u32)>) {
        
        draw_rect_border(canvas, self.rect);

        self.label.draw(canvas, sv);
        self.choose_win.redraw(canvas, game, sv, anim);
    }
}

impl DialogWindow for ExitWindow {
    fn process_command(&mut self, command: Command, pa: DoPlayerAction) -> DialogResult {
        match &command {
            &Command::Cancel => {
                return DialogResult::Close;
            }
            _ => (),
        }
        
        match self.choose_win.process_command(command, pa) {
            DialogResult::CloseWithValue(v) => { // An choice is choosed
                let n = *v.downcast::<u32>().unwrap();
                match n {
                    0 => { return DialogResult::Quit }
                    1 => { return DialogResult::Close }
                    _ => panic!(),
                }
            },
            _ => (),
        }
        return DialogResult::Continue;
    }

    fn mode(&self) -> InputMode {
        InputMode::Dialog
    }
}


