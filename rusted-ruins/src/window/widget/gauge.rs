use super::{LabelWidget, WidgetTrait};
use crate::config::UI_CFG;
use crate::context::*;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

/// Bar gauge widget.
pub struct GaugeWidget {
    rect: Rect,
    colors: Colors,
    value: f32,
    min: f32,
    max: f32,
    label: Option<LabelWidget>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GaugeColorMode {
    Hp,
    Sp,
    // Exp,
    Work,
}

impl GaugeColorMode {
    fn colors(&self) -> Colors {
        match self {
            GaugeColorMode::Hp => Colors {
                bar: UI_CFG.color.gauge_hp.into(),
                bg: UI_CFG.color.gauge_bg.into(),
                border_light: UI_CFG.color.border_light.into(),
                border_dark: UI_CFG.color.border_dark.into(),
            },
            GaugeColorMode::Sp => Colors {
                bar: UI_CFG.color.gauge_sp.into(),
                bg: UI_CFG.color.gauge_bg.into(),
                border_light: UI_CFG.color.border_light.into(),
                border_dark: UI_CFG.color.border_dark.into(),
            },
            GaugeColorMode::Work => Colors {
                bar: UI_CFG.color.gauge_work.into(),
                bg: UI_CFG.color.gauge_bg.into(),
                border_light: UI_CFG.color.border_light.into(),
                border_dark: UI_CFG.color.border_dark.into(),
            },
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Colors {
    bg: Color,
    bar: Color,
    border_light: Color,
    border_dark: Color,
}

impl GaugeWidget {
    pub fn new(rect: Rect, min: f32, max: f32, mode: GaugeColorMode) -> GaugeWidget {
        GaugeWidget {
            rect,
            colors: mode.colors(),
            label: None,
            value: min,
            min,
            max,
        }
    }

    // pub fn with_label(
    //     rect: Rect,
    //     min: f32,
    //     max: f32,
    //     mode: GaugeColorMode,
    //     text: &str,
    // ) -> GaugeWidget {
    //     GaugeWidget {
    //         rect,
    //         colors: mode.colors(),
    //         label: Some(LabelWidget::bordered(rect, text, FontKind::MonoM).centering()),
    //         value: min,
    //         min,
    //         max,
    //     }
    // }

    pub fn set_params(&mut self, min: f32, max: f32, value: f32) {
        self.value = value;
        self.max = max;
        self.min = min;
    }

    pub fn set_value(&mut self, value: f32) {
        self.value = value;
    }
}

impl WidgetTrait for GaugeWidget {
    type Response = ();

    fn draw(&mut self, context: &mut Context) {
        let canvas = &mut context.canvas;
        canvas.set_draw_color(self.colors.bg);
        try_sdl!(canvas.fill_rect(self.rect));

        let value = if self.value >= self.min {
            self.value
        } else {
            self.min
        };
        let bar_width =
            ((self.rect.w - 4) as f32 * ((value - self.min) / (self.max - self.min))) as u32;
        let bar_rect = Rect::new(
            self.rect.x + 2,
            self.rect.y + 2,
            bar_width,
            self.rect.height() - 2,
        );

        canvas.set_draw_color(self.colors.bar);
        try_sdl!(canvas.fill_rect(bar_rect));

        for n in 0..2 {
            let r = Rect::new(
                self.rect.x + n,
                self.rect.y + n,
                (self.rect.w - 2 * n) as u32,
                (self.rect.h - 2 * n) as u32,
            );
            let c: Color = if n == 0 {
                self.colors.border_dark
            } else {
                self.colors.border_light
            };

            canvas.set_draw_color(c);
            try_sdl!(canvas.draw_rect(r));
        }

        if let Some(ref mut label) = self.label {
            label.draw(context);
        }
    }
}
