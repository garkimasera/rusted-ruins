use super::commonuse::*;
use super::widget::*;
use common::objholder::CharaTemplateIdx;

/// Displays one image
pub struct ImageWindow {
    rect: Rect,
    image: ImageWidget,
}

impl ImageWindow {
    pub fn chara(rect: Rect, chara: CharaTemplateIdx) -> ImageWindow {
        use common::basic::TILE_SIZE;
        ImageWindow {
            rect,
            image: ImageWidget::chara(Rect::new(0, 0, TILE_SIZE, TILE_SIZE), chara),
        }
    }
}

impl Window for ImageWindow {
    fn draw(
        &mut self,
        context: &mut Context<'_, '_, '_, '_>,
        _game: &Game,
        _anim: Option<(&Animation, u32)>,
    ) {
        draw_window_border(context, self.rect);
        self.image.draw(context);
    }
}
