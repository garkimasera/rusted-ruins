
use config::SCREEN_CFG;
use super::commonuse::*;
use super::widget::WidgetTrait;
use super::widget::ImageWidget;

pub struct StartWindow {
    title_screen: ImageWidget,
}

impl StartWindow {
    pub fn new() -> StartWindow {
        let rect = Rect::new(0, 0, SCREEN_CFG.screen_w, SCREEN_CFG.screen_h);
        
        StartWindow {
            title_screen: ImageWidget::ui_img(rect, "!title-screen"),
        }
    }
    
    pub fn redraw(&mut self, canvas: &mut WindowCanvas, game: &Game, sv: &mut SdlValues,
              anim: Option<(&Animation, u32)>) {

        self.title_screen.draw(canvas, sv);
    }
}



