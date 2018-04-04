
use config::{SCREEN_CFG, UI_CFG};
use game::InfoGetter;
use game::site::SiteEx;
use super::commonuse::*;
use super::widget::*;
use common::gobj;
use common::obj::UIImgObject;
use common::gamedata::map::MapId;
use common::gamedata::chara::{CharaId, CharaStatus};
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
    fn draw(
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
        let label = LabelWidget::new(Rect::new(0, 0, rect.width(), rect.height()), "", FontKind::S);
        FloorInfo { rect, label, mid: None, }
    }
}

impl Window for FloorInfo {
    fn draw(
        &mut self, canvas: &mut WindowCanvas, game: &Game, sv: &mut SdlValues,
        _anim: Option<(&Animation, u32)>) {

        let current_mid = game.gd.get_current_mapid();

        if self.mid != Some(current_mid) {
            self.mid = Some(current_mid);
            match current_mid {
                MapId::SiteMap { sid, floor }=> {
                    let site = game.gd.region.get_site(sid);
                    self.label.set_text(&format!("{} ({})", site.get_name(), floor + 1));
                }
                MapId::RegionMap { rid } => {
                    let region = game.gd.region.get(rid);
                    self.label.set_text(&format!("{}", region.name))
                }
            }
        }
        
        canvas.set_viewport(self.rect);
        self.label.draw(canvas, sv);
    }
}

pub struct StatusInfo {
    labels: Vec<LabelWidget>,
    status: Vec<CharaStatus>,
}

impl StatusInfo {
    pub fn new() -> StatusInfo {
        StatusInfo {
            labels: Vec::new(),
            status: Vec::new(),
        }
    }

    fn update(&mut self, game: &Game) {
        let player_chara = game.gd.chara.get(CharaId::Player);
        let rect: Rect = SCREEN_CFG.status_info.into();
        
        if self.status != player_chara.status {
            self.status.clone_from(&player_chara.status);

            self.labels.clear();
            for (i, status) in self.status.iter().enumerate() {
                let label = LabelWidget::new(
                    Rect::new(rect.x + rect.h * i as i32, rect.y, 1, 1),
                    &format!("{:?}", status),
                    FontKind::S);
                self.labels.push(label);
            }
        }
    }
}

impl Window for StatusInfo {
    fn draw(
        &mut self, canvas: &mut WindowCanvas, game: &Game, sv: &mut SdlValues,
        _anim: Option<(&Animation, u32)>) {
        
        self.update(game);
        
        canvas.set_viewport(None);
        for label in self.labels.iter_mut() {
            label.draw(canvas, sv);
        }
    }
}

