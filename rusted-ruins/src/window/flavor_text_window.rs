use super::commonuse::*;
use super::widget::*;
use crate::text::{obj_txt, ui_txt};

pub struct FlavorTextWindow {
    rect: Rect,
    escape_click: bool,
    name: LabelWidget,
    label: LabelWidget,
    image: ImageWidget,
    close_button: ButtonWidget,
}

impl FlavorTextWindow {
    pub fn new(id: &str, image_idx: ImageIdx) -> Option<FlavorTextWindow> {
        let cfg = &UI_CFG.flavor_text_window;
        let rect: Rect = cfg.rect.into();
        let text = crate::text::flavor_txt_checked(id)?;
        let name = LabelWidget::new(cfg.name_rect, &obj_txt(id), FontKind::M);
        let label = LabelWidget::wrapped(cfg.text_rect, &text, FontKind::Talk, cfg.text_rect.w);
        let image = ImageWidget::new(cfg.image_rect, image_idx);
        let close_button = ButtonWidget::new(
            cfg.close_button_rect,
            &ui_txt("button_text-close"),
            FontKind::M,
        );

        Some(FlavorTextWindow {
            rect,
            escape_click: false,
            name,
            label,
            image,
            close_button,
        })
    }
}

impl Window for FlavorTextWindow {
    fn draw(&mut self, context: &mut Context, _game: &Game, _anim: Option<(&Animation, u32)>) {
        draw_window_border(context, self.rect);
        self.image.draw(context);
        self.name.draw(context);
        self.label.draw(context);
        self.close_button.draw(context);
    }
}

impl DialogWindow for FlavorTextWindow {
    fn process_command(&mut self, command: &Command, _pa: &mut DoPlayerAction) -> DialogResult {
        check_escape_click!(self, command, false);

        let command = command.relative_to(self.rect);
        if self.close_button.process_command(&command).is_some() {
            return DialogResult::Close;
        }

        DialogResult::Continue
    }

    fn mode(&self) -> InputMode {
        InputMode::Dialog
    }
}
