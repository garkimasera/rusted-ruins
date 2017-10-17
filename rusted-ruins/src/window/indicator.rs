
use config::{SCREEN_CFG, UI_CFG};
use super::commonuse::*;
use super::widget::*;

pub struct HPIndicator {
    rect: Rect,
    guage: GuageWidget,
}

impl HPIndicator {
    pub fn new() -> HPIndicator {
        let rect: Rect = SCREEN_CFG.hp_indicator.into();
        let color = UI_CFG.color.guage_hp;
        
        HPIndicator {
            rect,
            guage: GuageWidget::new(Rect::new(0, 0, rect.width(), rect.height()), 0.0, 1.0, color.into()),
        }
    }
}

impl Window for HPIndicator {
    fn redraw(
        &mut self, canvas: &mut WindowCanvas, _game: &Game, sv: &mut SdlValues,
        _anim: Option<(&Animation, u32)>) {

        canvas.set_viewport(self.rect);
        self.guage.set_params(0.0, 1.0, 0.5);
        self.guage.draw(canvas, sv);
        
    }
}

