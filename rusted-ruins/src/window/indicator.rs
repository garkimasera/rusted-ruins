
use config::{SCREEN_CFG, UI_CFG};
use game::InfoGetter;
use super::commonuse::*;
use super::widget::*;
use common::gobj;
use common::obj::UIImgObject;
use common::gamedata::map::MapId;
use sdlvalues::FontKind;

pub struct HPIndicator {
    rect: Rect,
    guage: GuageWidget,
    label: ImageWidget,
}

impl HPIndicator {
    pub fn new() -> HPIndicator {
        let rect: Rect = SCREEN_CFG.hp_indicator.into();
        let color = UI_CFG.color.guage_hp;

        // Label is drawed over the guage
        let label_id = "!label-hp";
        let label_img: &'static UIImgObject = gobj::get_by_id(label_id);
        let (label_w, label_h) = (label_img.img.w, label_img.img.h);
        let label_rect = Rect::from_center(
            (rect.w / 2, rect.h/ 2), label_w, label_h); // Centering of the guage
        
        HPIndicator {
            rect,
            guage: GuageWidget::new(Rect::new(0, 0, rect.width(), rect.height()), 0.0, 1.0, color.into()),
            label: ImageWidget::ui_img(label_rect, label_id),
        }
    }
}

impl Window for HPIndicator {
    fn redraw(
        &mut self, canvas: &mut WindowCanvas, game: &Game, sv: &mut SdlValues,
        _anim: Option<(&Animation, u32)>) {

        let (max_hp, hp) = game.gd.player_hp();
        self.guage.set_params(0.0, max_hp as f32, hp as f32);

        canvas.set_viewport(self.rect);
        self.guage.draw(canvas, sv);
        self.label.draw(canvas, sv);        
    }
}

pub struct FloorInfo {
    rect: Rect,
    label: LabelWidget,
    mid: Option<MapId>,
}

impl FloorInfo {
    pub fn new() -> FloorInfo {
        let rect: Rect = SCREEN_CFG.floor_info.into();
        let label = LabelWidget::new(Rect::new(0, 0, rect.width(), rect.height()), "AA", FontKind::M);
        FloorInfo { rect, label, mid: None, }
    }
}

impl Window for FloorInfo {
    fn redraw(
        &mut self, canvas: &mut WindowCanvas, game: &Game, sv: &mut SdlValues,
        _anim: Option<(&Animation, u32)>) {

        let current_mid = game.gd.get_current_mapid();

        if self.mid != Some(current_mid) {
            self.mid = Some(current_mid);
            let site = game.gd.site.get(current_mid.sid);
            let s = format!("{} ({})", site.name, current_mid.floor + 1);
            self.label.set_text(&s);
        }
        
        canvas.set_viewport(self.rect);
        self.label.draw(canvas, sv);
    }
}

