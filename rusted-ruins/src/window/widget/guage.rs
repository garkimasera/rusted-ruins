
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use super::WidgetTrait;
use sdlvalues::SdlValues;
use config::UI_CFG;

/// Guage bar widget.
pub struct GuageWidget {
    rect: Rect,
    color: Color,
    value: f32, min: f32, max: f32,
}

impl GuageWidget {
    pub fn new(rect: Rect, min: f32, max: f32, color: Color) -> GuageWidget {
        GuageWidget {
            rect, color,
            value: min, min, max,
        }
    }

    pub fn set_params(&mut self, min: f32, max: f32, value: f32) {
        self.value = value;
        self.max = max;
        self.min = min;
    }
}

impl WidgetTrait for GuageWidget {
    type Response =  ();

    fn draw(&mut self, canvas: &mut WindowCanvas, _sv: &mut SdlValues) {
        let light_color: Color = UI_CFG.color.guage_border_light.into();
        let dark_color: Color = UI_CFG.color.guage_border_dark.into();
        let bg: Color = UI_CFG.color.guage_bg.into();

        canvas.set_draw_color(bg);
        check_draw!(canvas.fill_rect(self.rect));

        let value = if self.value >= self.min { self.value } else { self.min };
        let bar_width =
            ((self.rect.w - 4) as f32 * ((value - self.min) / (self.max - self.min))) as u32;
        let bar_rect = Rect::new(2, 2, bar_width, self.rect.height() - 2);

        canvas.set_draw_color(self.color);
        check_draw!(canvas.fill_rect(bar_rect));
        
        for n in 0..2 {
            let r = Rect::new(
                self.rect.x + n, self.rect.y + n,
                (self.rect.w - 2 * n) as u32, (self.rect.h - 2 * n) as u32);
            let c: Color = if n == 0 { dark_color } else { light_color };

            canvas.set_draw_color(c);
            check_draw!(canvas.draw_rect(r));
        }
    }
}

