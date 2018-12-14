
use sdl2::rect::Rect;
use super::commonuse::*;
use super::widget::*;
use super::winpos::WindowPos;

/// Player chooses one item from list
/// The choices cannot be changed
/// This handles text list only
pub struct ChooseWindow {
    winpos: WindowPos,
    answer_list: ListWidget,
    default_choose: Option<u32>,
}

impl ChooseWindow {
    pub fn new(winpos: WindowPos, choices: Vec<String>, default_choose: Option<u32>) -> ChooseWindow {
        ChooseWindow {
            winpos: winpos,
            answer_list: ListWidget::texts_choices((0, 0, 0, 0), choices),
            default_choose: default_choose,
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
        draw_rect_border(context.canvas, rect);
        
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

/// Player chooses one item from list
/// The choices cannot be changed
/// Includes page functions
pub struct PagedChooseWindow {
    rect: Rect,
    answer_list: ListWidget,
    choices: Vec<ListRow>,
}

impl PagedChooseWindow {
    pub fn new(rect: Rect, choices: Vec<ListRow>, page_size: u32, column_pos: Option<Vec<i32>>)
               -> PagedChooseWindow {
        let n_item = choices.len() as u32;
        
        let answer_list = ListWidget::new(
            Rect::new(0, 0, rect.width(), rect.height()),
            choices[0].kind(),
            column_pos.unwrap_or(vec![0]),
            Some(page_size),
            26);
        
        let mut w = PagedChooseWindow {
            rect, answer_list, choices
        };
        w.answer_list.set_n_item(n_item);
        w.update_list();
        w
    }

    pub fn get_current_choice(&self) -> u32 {
        self.answer_list.get_current_choice()
    }

    fn update_list(&mut self) {
        let choices = &self.choices;
        self.answer_list.update_rows_by_func(|start, page_size| {
            let mut rows = Vec::new();
            for i in start..(start + page_size) {
                if let Some(choice) = choices.get(i as usize) {
                    rows.push(choice.clone());
                }
            }
            rows
        });
    }
}

impl Window for PagedChooseWindow {
    
    fn draw(
        &mut self, context: &mut Context, _game: &Game, _anim: Option<(&Animation, u32)>) {

        // Drawing
        draw_rect_border(context.canvas, self.rect);
        
        self.answer_list.draw(context);
    }
}

impl DialogWindow for PagedChooseWindow {
    fn process_command(&mut self, command: &Command, _pa: &mut DoPlayerAction) -> DialogResult {
        if let Some(response) = self.answer_list.process_command(&command) {
            match response {
                ListWidgetResponse::Select(n) => {
                    return DialogResult::CloseWithValue(Box::new(n));
                },
                ListWidgetResponse::PageChanged => {
                    self.update_list();
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
}
