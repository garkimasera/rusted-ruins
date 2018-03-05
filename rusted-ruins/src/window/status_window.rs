
use super::commonuse::*;
use super::widget::*;
use sdlvalues::FontKind;
use config::UI_CFG;
use common::gamedata::GameData;
use common::gamedata::chara::*;
use game::chara::CharaEx;
use text;

/// Character status viewer
pub struct StatusWindow {
    rect: Rect,
    image: ImageWidget,
    name_label: LabelWidget,
    hp_label: LabelWidget,
    str_label: LabelWidget,
    vit_label: LabelWidget,
    dex_label: LabelWidget,
    int_label: LabelWidget,
    wil_label: LabelWidget,
    cha_label: LabelWidget,
}

impl StatusWindow {
    pub fn new(gd: &GameData) -> StatusWindow {
        let cfg = &UI_CFG.status_window;
        let rect: Rect = cfg.rect.into();
        let chara = gd.chara.get(CharaId::Player);
        let image = ImageWidget::chara(cfg.image_rect, chara.template);
        let name_label = LabelWidget::new(cfg.name_label_rect, chara.get_name(), FontKind::M);
        let hp_label = LabelWidget::new(
            cfg.hp_label_rect, &format!("HP  {} / {}", chara.hp, chara.params.max_hp), FontKind::M);
        let str_label = LabelWidget::new(
            cfg.str_label_rect, &format!("STR  {}", chara.params.str), FontKind::M);
        let vit_label = LabelWidget::new(
            cfg.vit_label_rect, &format!("VIT  {}", chara.params.vit), FontKind::M);
        let dex_label = LabelWidget::new(
            cfg.dex_label_rect, &format!("DEX  {}", chara.params.dex), FontKind::M);
        let int_label = LabelWidget::new(
            cfg.int_label_rect, &format!("INT  {}", chara.params.int), FontKind::M);
        let wil_label = LabelWidget::new(
            cfg.wil_label_rect, &format!("WIL  {}", chara.params.wil), FontKind::M);
        let cha_label = LabelWidget::new(
            cfg.cha_label_rect, &format!("CHA  {}", chara.params.cha), FontKind::M);
        StatusWindow {
            rect,
            image, name_label, hp_label,
            str_label, vit_label, dex_label, int_label, wil_label, cha_label,
        }
    }
}

impl Window for StatusWindow {
    fn draw(
        &mut self, canvas: &mut WindowCanvas, _game: &Game, sv: &mut SdlValues,
        _anim: Option<(&Animation, u32)>) {

        draw_rect_border(canvas, self.rect);
        self.image.draw(canvas, sv);
        self.name_label.draw(canvas, sv);
        self.hp_label.draw(canvas, sv);
        self.str_label.draw(canvas, sv);
        self.vit_label.draw(canvas, sv);
        self.dex_label.draw(canvas, sv);
        self.int_label.draw(canvas, sv);
        self.wil_label.draw(canvas, sv);
        self.cha_label.draw(canvas, sv);
    }
}

impl DialogWindow for StatusWindow {
    fn process_command(&mut self, command: &Command, _pa: &mut DoPlayerAction) -> DialogResult {
        match *command {
            Command::Cancel => DialogResult::Close,
            _ => DialogResult::Continue,
        }
    }

    fn mode(&self) -> InputMode {
        InputMode::Dialog
    }
}

