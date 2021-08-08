use super::commonuse::*;
use super::widget::{GaugeColorMode, GaugeWidget};
use crate::config::UI_CFG;
use common::gamedata::*;

pub struct ProgressBar {
    gauge: GaugeWidget,
}

impl ProgressBar {
    pub fn new() -> ProgressBar {
        let rect: Rect = UI_CFG.progress_bar.rect.into();
        let gauge = GaugeWidget::new(rect, 0.0, 1.0, GaugeColorMode::Work);

        ProgressBar { gauge }
    }
}

impl Window for ProgressBar {
    fn draw(
        &mut self,
        context: &mut Context<'_, '_, '_, '_>,
        game: &Game<'_>,
        anim: Option<(&Animation, u32)>,
    ) {
        let player = game.gd.chara.get(CharaId::Player);
        let mut in_work = false;
        for status in &player.status {
            if let CharaStatus::Work { .. } = status {
                in_work = true;
                break;
            }
        }

        if !in_work {
            return;
        }

        if let Some((Animation::Work { ratio, .. }, _)) = anim {
            self.gauge.set_value(1.0 - *ratio);
        }

        context.set_viewport(None);
        self.gauge.draw(context);
    }
}
