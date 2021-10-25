use super::commonuse::*;
use super::widget::*;

pub type CallbackSelected<T> = Box<dyn FnMut(u32, &mut ListWidget<T>)>;

pub struct ListWithDescWindow<T> {
    rect: Rect,
    pub list: ListWidget<T>,
    pub text: LabelWidget,
    escape_click: bool,
    cb_selection_changed: Option<Box<dyn FnMut(u32, &mut LabelWidget)>>,
    cb_selected: Option<CallbackSelected<T>>,
}

impl<T: ListWidgetRow> ListWithDescWindow<T> {
    pub fn new(rect: Rect, column_pos: Vec<i32>, items: Vec<T>) -> Self {
        let w2 = rect.width() / 2 - 1;
        let text_rect = Rect::new(0, 0, w2, rect.height());
        let list_rect = Rect::new(w2 as i32 + 2, 0, w2, rect.height());
        let list_page_size = rect.height() / UI_CFG.list_widget.h_row_default;

        let mut list = ListWidget::with_scroll_bar(list_rect, column_pos, list_page_size, false);
        list.set_items(items);

        let text = LabelWidget::wrapped(text_rect, "", FontKind::S, w2);

        ListWithDescWindow {
            rect,
            list,
            text,
            escape_click: false,
            cb_selection_changed: None,
            cb_selected: None,
        }
    }

    pub fn set_cb_selection_changed(&mut self, cb: Box<dyn FnMut(u32, &mut LabelWidget)>) {
        self.cb_selection_changed = Some(cb);
    }

    pub fn set_cb_selected(&mut self, cb: CallbackSelected<T>) {
        self.cb_selected = Some(cb);
    }
}

impl<T: ListWidgetRow> Window for ListWithDescWindow<T> {
    fn draw(
        &mut self,
        context: &mut Context<'_, '_, '_, '_>,
        _game: &Game<'_>,
        _anim: Option<(&Animation, u32)>,
    ) {
        draw_window_border(context, self.rect);

        let line_x = self.rect.w / 2;
        let h = self.rect.h;
        context.draw_line((line_x - 1, 0), (line_x - 1, h), UI_CFG.color.border_light);
        context.draw_line((line_x, 0), (line_x, h), UI_CFG.color.border_dark);

        self.list.draw(context);
        self.text.draw(context);
    }
}

impl<T: ListWidgetRow> DialogWindow for ListWithDescWindow<T> {
    fn process_command(
        &mut self,
        command: &Command,
        _pa: &mut DoPlayerAction<'_, '_>,
    ) -> DialogResult {
        check_escape_click!(self, command);
        let command = command.relative_to(self.rect);

        match self.list.process_command(&command) {
            Some(ListWidgetResponse::SelectionChanged(i)) => {
                if let Some(cb) = self.cb_selection_changed.as_mut() {
                    cb(i, &mut self.text);
                }
            }
            Some(ListWidgetResponse::Select(i)) => {
                if let Some(cb) = self.cb_selected.as_mut() {
                    cb(i, &mut self.list);
                }
            }
            _ => (),
        }

        match command {
            Command::Cancel => DialogResult::Close,
            _ => DialogResult::Continue,
        }
    }
}
