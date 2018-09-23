
use config::{SCREEN_CFG, UI_CFG};
use game::InfoGetter;
use text::ToText;
use super::commonuse::*;
use super::widget::*;
use common::gobj;
use common::obj::UIImgObject;
use common::gamedata::*;
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
        let label = LabelWidget::bordered(Rect::new(0, 0, rect.width(), rect.height()), "", FontKind::S);
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
                    self.label.set_text(&format!("{} ({})", site.to_text(), floor + 1));
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

pub struct TimeInfo {
    date_label: LabelWidget,
    time_label: LabelWidget,
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
}

impl TimeInfo {
    pub fn new() -> TimeInfo {
        let rect: Rect = SCREEN_CFG.date_info.into();
        let date_label = LabelWidget::bordered(rect, "", FontKind::S);
        let rect: Rect = SCREEN_CFG.time_info.into();
        let time_label = LabelWidget::bordered(rect, "", FontKind::M);
        TimeInfo {
            date_label,
            time_label,
            year: 0, month: 0, day: 0, hour: 0, minute: 0,
        }
    }
}

impl Window for TimeInfo {
    fn draw(
        &mut self, canvas: &mut WindowCanvas, game: &Game, sv: &mut SdlValues,
        _anim: Option<(&Animation, u32)>) {

        let time = &game.gd.time;
        let mut date_changed = false;
        if self.year != time.year() {
            self.year = time.year();
            date_changed = true;
        }
        if self.month != time.month() {
            self.month = time.month();
            date_changed = true;
        }
        if self.day != time.day() {
            self.day = time.day();
            date_changed = true;
        }
        let mut time_changed = false;
        if self.hour != time.hour() {
            self.hour = time.hour();
            time_changed = true;
        }
        if self.minute != time.minute() {
            self.minute = time.minute();
            time_changed = true;
        }
        if date_changed {
            self.date_label.set_text(&format!("{}/{:02}/{:02}", self.year, self.month, self.day))
        }
        if time_changed {
            self.time_label.set_text(&format!("{:02}:{:02}", self.hour, self.minute))
        }
        self.date_label.draw(canvas, sv);
        self.time_label.draw(canvas, sv);
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
                let label = LabelWidget::bordered(
                    Rect::new(rect.x + rect.h * i as i32, rect.y, 1, 1),
                    ::text::to_txt(status),
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

