
use sdl2::rect::Rect;
use super::commonuse::*;
use super::widget::*;
use super::winpos::WindowPos;

/// Player chooses one item from list.
/// The choices cannot be changed.
/// This handles text list only.
pub struct ChooseWindow {
    winpos: WindowPos,
    answer_list: TextListWidget,
    default_choose: Option<u32>,
}

impl ChooseWindow {
    pub fn new(winpos: WindowPos, choices: Vec<String>, default_choose: Option<u32>) -> ChooseWindow {
        ChooseWindow {
            winpos,
            answer_list: TextListWidget::text_choices((0, 0, 0, 0), choices),
            default_choose,
        }
    }

    /// Create ChooseWindow with two choices, yes and no
    /// default_choose: When Esc is inputed, which choice will be returned
    pub fn with_yesno(winpos: WindowPos, default_choose: Option<bool>) -> ChooseWindow {
        let choices = vec!["Yes".to_owned(), "No".to_owned()];
        let default_choose = default_choose.map(|a| if a { 0 } else { 1 });
        ChooseWindow::new(winpos, choices, default_choose)
    }

    pub fn set_winpos(&mut self, winpos: WindowPos) {
        self.winpos = winpos;
    }
}

impl Window for ChooseWindow {
    fn draw(
        &mut self, context: &mut Context, _game: &Game, _anim: Option<(&Animation, u32)>) {

        // Update window size
        let list_widget_size = self.answer_list.adjust_widget_size(context.sv);
        let left_top_point = self.winpos.calc_left_top(list_widget_size.0, list_widget_size.1);
        let rect = Rect::new(left_top_point.0, left_top_point.1, list_widget_size.0, list_widget_size.1);

        // Drawing
        draw_rect_border(context, rect);
        
        self.answer_list.draw(context);
    }
}

impl DialogWindow for ChooseWindow {
    fn process_command(&mut self, command: &Command, _pa: &mut DoPlayerAction) -> DialogResult {
        if let Some(response) = self.answer_list.process_command(&command) {
            match response {
                ListWidgetResponse::Select(n) => {
                    return DialogResult::CloseWithValue(Box::new(n));
                },
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
}

