use super::commonuse::*;
use super::widget::*;
use crate::config::SCREEN_CFG;
use crate::context::textrenderer::FontKind;
use crate::game::InfoGetter;
use crate::text::ToText;
use common::gamedata::*;
use common::gobj;
use common::obj::UiImgObject;
use geom::Coords;
use rules::RULES;

#[derive(Clone, Copy, Debug)]
pub enum BarIndicatorKind {
    Hp,
    Mp,
    Sp,
}

impl BarIndicatorKind {
    fn label_id(self) -> &'static str {
        match self {
            BarIndicatorKind::Hp => "!label-hp",
            BarIndicatorKind::Mp => "!label-mp",
            BarIndicatorKind::Sp => "!label-sp",
        }
    }

    fn color_mode(self) -> GaugeColorMode {
        match self {
            BarIndicatorKind::Hp => GaugeColorMode::Hp,
            BarIndicatorKind::Mp => GaugeColorMode::Mp,
            BarIndicatorKind::Sp => GaugeColorMode::Sp,
        }
    }

    fn rect(self) -> Rect {
        match self {
            BarIndicatorKind::Hp => SCREEN_CFG.hp_indicator.into(),
            BarIndicatorKind::Mp => SCREEN_CFG.mp_indicator.into(),
            BarIndicatorKind::Sp => SCREEN_CFG.sp_indicator.into(),
        }
    }
}

pub struct BarIndicator {
    rect: Rect,
    kind: BarIndicatorKind,
    guage: GaugeWidget,
    label: ImageWidget,
}

impl BarIndicator {
    pub fn new(kind: BarIndicatorKind) -> BarIndicator {
        let rect: Rect = kind.rect();

        // Label is drawed over the guage
        let label_img: &'static UiImgObject = gobj::get_by_id(kind.label_id());
        let (label_w, label_h) = (label_img.img.w, label_img.img.h);
        // Centering of the guage
        let label_rect = Rect::from_center((rect.w / 2, rect.h / 2), label_w, label_h);

        BarIndicator {
            rect,
            kind,
            guage: GaugeWidget::new(
                Rect::new(0, 0, rect.width(), rect.height()),
                0.0,
                1.0,
                kind.color_mode(),
            ),
            label: ImageWidget::ui_img(label_rect, kind.label_id()),
        }
    }
}

impl Window for BarIndicator {
    fn draw(
        &mut self,
        context: &mut Context<'_, '_, '_, '_>,
        game: &Game,
        _anim: Option<(&Animation, u32)>,
    ) {
        match self.kind {
            BarIndicatorKind::Hp => {
                let (max_hp, hp) = game.gd.player_hp();
                self.guage.set_params(0.0, max_hp as f32, hp as f32);
            }
            BarIndicatorKind::Mp => {
                let (max_mp, mp) = game.gd.player_mp();
                self.guage.set_params(0.0, max_mp as f32, mp as f32);
            }
            BarIndicatorKind::Sp => {
                let sp = game.gd.chara.get(CharaId::Player).sp;
                let r = &RULES.chara;
                self.guage.set_params(r.sp_starving, r.sp_max, sp);
            }
        }

        context.set_viewport(self.rect);
        self.guage.draw(context);
        self.label.draw(context);
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
        let label = LabelWidget::bordered(
            Rect::new(0, 0, rect.width(), rect.height()),
            "",
            FontKind::T,
        );
        FloorInfo {
            rect,
            label,
            mid: None,
        }
    }
}

impl Window for FloorInfo {
    fn draw(
        &mut self,
        context: &mut Context<'_, '_, '_, '_>,
        game: &Game,
        _anim: Option<(&Animation, u32)>,
    ) {
        let current_mid = game.gd.get_current_mapid();

        if self.mid != Some(current_mid) {
            self.mid = Some(current_mid);
            match current_mid {
                MapId::SiteMap { sid, floor } => {
                    let site = game.gd.region.get_site(sid);
                    self.label
                        .set_text(&format!("{} ({})", site.to_text(), floor + 1));
                }
                MapId::RegionMap { rid } => {
                    let region = game.gd.region.get(rid);
                    self.label.set_text(region.to_text())
                }
            }
        }

        context.set_viewport(self.rect);
        self.label.draw(context);
    }
}

pub struct TimeInfo {
    date_label: LabelWidget,
    time_label: LabelWidget,
    year: u32,
    month: u16,
    day: u16,
    hour: u16,
    minute: u16,
}

impl TimeInfo {
    pub fn new() -> TimeInfo {
        let rect: Rect = UI_CFG.time_info.date_label.into();
        let date_label = LabelWidget::bordered(rect, "", FontKind::T);
        let rect: Rect = UI_CFG.time_info.time_label.into();
        let time_label = LabelWidget::bordered(rect, "", FontKind::M);
        TimeInfo {
            date_label,
            time_label,
            year: 0,
            month: 0,
            day: 0,
            hour: 0,
            minute: 0,
        }
    }
}

impl Window for TimeInfo {
    fn draw(
        &mut self,
        context: &mut Context<'_, '_, '_, '_>,
        game: &Game,
        _anim: Option<(&Animation, u32)>,
    ) {
        draw_window_border(context, SCREEN_CFG.time_info);

        let date = game.gd.time.current_date();
        let mut date_changed = false;
        if self.year != date.year {
            self.year = date.year;
            date_changed = true;
        }
        if self.month != date.month {
            self.month = date.month;
            date_changed = true;
        }
        if self.day != date.day {
            self.day = date.day;
            date_changed = true;
        }
        let mut time_changed = false;
        if self.hour != date.hour {
            self.hour = date.hour;
            time_changed = true;
        }
        let minute10 = date.minute - date.minute % 10;
        if self.minute != minute10 {
            self.minute = minute10;
            time_changed = true;
        }
        if date_changed {
            self.date_label
                .set_text(&format!("{}/{:02}/{:02}", self.year, self.month, self.day))
        }
        if time_changed {
            self.time_label
                .set_text(&format!("{:02}:{:02}", self.hour, minute10))
        }
        self.date_label.draw(context);
        self.time_label.draw(context);
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
                    Rect::new(rect.x, rect.y - rect.h * i as i32, 1, 1),
                    &crate::text::to_txt(status),
                    FontKind::T,
                );
                self.labels.push(label);
            }
        }
    }
}

impl Window for StatusInfo {
    fn draw(
        &mut self,
        context: &mut Context<'_, '_, '_, '_>,
        game: &Game,
        _anim: Option<(&Animation, u32)>,
    ) {
        self.update(game);

        context.set_viewport(None);
        for label in self.labels.iter_mut() {
            label.draw(context);
        }
    }
}

#[derive(Default)]
pub struct SupplementInfo {
    hover_tile: Option<Coords>,
    tile_obj: Option<TextCache>,
    chara: Option<TextCache>,
}

impl SupplementInfo {
    pub fn update_hover_tile(&mut self, game: &Game, pos: Coords) {
        if self.hover_tile != Some(pos) {
            self.hover_tile = Some(pos);
            self.update(game);
        }
    }

    pub fn update(&mut self, game: &Game) {
        let pos = if let Some(pos) = self.hover_tile {
            pos
        } else {
            return;
        };

        if !game.view_map.get_tile_visible(pos) {
            *self = Self::default();
            return;
        }

        // About tile object
        let map = game.gd.get_current_map();
        let text: Option<String> = if let SpecialTileKind::SiteSymbol { .. } = map.tile[pos].special
        {
            let mid = game.gd.get_current_mapid();
            if mid.is_region_map() {
                let region = game.gd.region.get(mid.rid());
                if let Some(sid) = region.get_id_by_pos(pos) {
                    let site = game.gd.region.get_site(sid);
                    Some(site.to_text().into())
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };
        self.tile_obj =
            text.map(|text| TextCache::bordered(text, FontKind::T, UI_CFG.color.normal_font));

        // About character
        let text: Option<String> = if let Some(cid) = game.gd.get_current_map().tile[pos].chara {
            let chara = game.gd.chara.get(cid);
            Some(chara.to_text().into())
        } else {
            None
        };
        self.chara =
            text.map(|text| TextCache::bordered(text, FontKind::T, UI_CFG.color.normal_font));
    }
}

impl Window for SupplementInfo {
    fn draw(
        &mut self,
        context: &mut Context<'_, '_, '_, '_>,
        _game: &Game,
        _anim: Option<(&Animation, u32)>,
    ) {
        context.set_viewport(None);

        let bottom_right = Rect::from(SCREEN_CFG.main_window).bottom_right();
        let margin = UI_CFG.supplement_info.margin;
        let h = UI_CFG.supplement_info.h;
        let x = bottom_right.x - margin;
        let mut y = bottom_right.y - margin;

        if let Some(tile_obj) = &mut self.tile_obj {
            tile_obj.render_at(context, x, y, false, false);
            y -= h;
        }

        if let Some(chara) = &mut self.chara {
            chara.render_at(context, x, y, false, false);
        }
    }
}
