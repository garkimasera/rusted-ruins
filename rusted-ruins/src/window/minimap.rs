
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use game::Game;
use sdlvalues::SdlValues;
use game::Animation;
use window::Window;
use config::SCREEN_CFG;

pub struct MiniMapWindow {
    rect: Rect,
}

impl MiniMapWindow {
    pub fn new() -> MiniMapWindow {
        MiniMapWindow {
            rect: SCREEN_CFG.minimap_window.into(),
        }
    }
}

impl Window for MiniMapWindow {
    fn redraw(
        &mut self, canvas: &mut WindowCanvas, game: &Game, sv: &mut SdlValues,
        _anim: Option<(&Animation, u32)>) {

        canvas.set_viewport(self.rect);
    }
}

fn draw_minimap(canvas: &mut WindowCanvas, game: &Game, sv: &mut SdlValues) {
    
}

