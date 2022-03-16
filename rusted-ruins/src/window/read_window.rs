use super::commonuse::*;
use super::widget::*;
use common::gobj;
use common::objholder::UiImgIdx;

pub struct ReadWindow {
    rect: Rect,
    closer: DialogCloser,
    label: LabelWidget,
    text: Vec<String>,
    current_page: usize,
    n_page: usize,
    next_button: ButtonWidget,
    prev_button: ButtonWidget,
    page_label: LabelWidget,
}

impl ReadWindow {
    pub fn new(title: &str) -> ReadWindow {
        let cfg = &UI_CFG.read_window;
        let rect: Rect = cfg.rect.into();
        let text = crate::text::readable::readable_txt(title);
        let label = LabelWidget::wrapped(cfg.text_rect, &text[0], FontKind::Talk, cfg.text_rect.w);
        let next_icon_idx: UiImgIdx = gobj::id_to_idx("!icon-next");
        let next_button =
            ButtonWidget::with_icon(cfg.next_button_rect, IconIdx::from(next_icon_idx));
        let prev_icon_idx: UiImgIdx = gobj::id_to_idx("!icon-prev");
        let prev_button =
            ButtonWidget::with_icon(cfg.prev_button_rect, IconIdx::from(prev_icon_idx));
        let n_page = text.len();
        let page_label = format!("{} / {}", 1, n_page);
        let page_label =
            LabelWidget::new(cfg.page_label_rect, &page_label, FontKind::Talk).centering();

        ReadWindow {
            rect,
            closer: DialogCloser::new(rect),
            label,
            text,
            current_page: 0,
            n_page,
            next_button,
            prev_button,
            page_label,
        }
    }

    pub fn set_page(&mut self, new_page: usize) {
        self.current_page = new_page;
        let page_label = format!("{} / {}", new_page + 1, self.n_page);
        self.page_label.set_text(&page_label);
        self.label.set_text(&self.text[new_page]);
    }

    pub fn button_available(&self) -> (bool, bool) {
        (self.current_page + 1 < self.n_page, self.current_page > 0)
    }
}

impl Window for ReadWindow {
    fn draw(
        &mut self,
        context: &mut Context<'_, '_, '_, '_>,
        _game: &Game,
        _anim: Option<(&Animation, u32)>,
    ) {
        self.closer.draw(context);
        draw_window_border(context, self.rect);
        self.label.draw(context);
        self.page_label.draw(context);
        let button_available = self.button_available();
        if button_available.0 {
            self.next_button.draw(context);
        }
        if button_available.1 {
            self.prev_button.draw(context);
        }
    }
}

impl DialogWindow for ReadWindow {
    fn process_command(&mut self, command: &Command, _pa: &mut DoPlayerAction<'_>) -> DialogResult {
        closer!(self, command);
        let command = command.relative_to(self.rect);

        let button_available = self.button_available();
        if button_available.0 && self.next_button.process_command(&command).is_some() {
            self.set_page(self.current_page + 1);
        }
        if button_available.1 && self.prev_button.process_command(&command).is_some() {
            self.set_page(self.current_page - 1);
        }

        DialogResult::Continue
    }

    fn draw_mode(&self) -> WindowDrawMode {
        WindowDrawMode::SkipUnderWindows
    }
}
