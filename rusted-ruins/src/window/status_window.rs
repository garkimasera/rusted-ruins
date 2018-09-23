
use super::commonuse::*;
use super::widget::*;
use sdlvalues::FontKind;
use config::UI_CFG;
use common::gamedata::*;
use text::ToText;
use super::group_window::GroupWindow;
use super::choose_window::PagedChooseWindow;

const STATUS_WINDOW_GROUP_SIZE: usize = 2;

pub fn create_status_window_group(game: &Game) -> GroupWindow {
    GroupWindow::new(STATUS_WINDOW_GROUP_SIZE, 0, game, create_members)
}

fn create_members(game: &Game, i: usize) -> Box<DialogWindow> {
    match i {
        0 => Box::new(StatusWindow::new(&game.gd)),
        1 => Box::new(SkillWindow::new(&game.gd)),
        _ => unreachable!(),
    }
}

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
        let name_label = LabelWidget::new(cfg.name_label_rect, &chara.to_text(), FontKind::M);
        let hp_label = LabelWidget::new(
            cfg.hp_label_rect, &format!("HP  {} / {}", chara.hp, chara.params.max_hp), FontKind::MonoM);
        let str_label = LabelWidget::new(
            cfg.str_label_rect, &format!("STR  {}", chara.params.str), FontKind::MonoM);
        let vit_label = LabelWidget::new(
            cfg.vit_label_rect, &format!("VIT  {}", chara.params.vit), FontKind::MonoM);
        let dex_label = LabelWidget::new(
            cfg.dex_label_rect, &format!("DEX  {}", chara.params.dex), FontKind::MonoM);
        let int_label = LabelWidget::new(
            cfg.int_label_rect, &format!("INT  {}", chara.params.int), FontKind::MonoM);
        let wil_label = LabelWidget::new(
            cfg.wil_label_rect, &format!("WIL  {}", chara.params.wil), FontKind::MonoM);
        let cha_label = LabelWidget::new(
            cfg.cha_label_rect, &format!("CHA  {}", chara.params.cha), FontKind::MonoM);
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

/// Character skill viewer
pub struct SkillWindow {
    choose_window: PagedChooseWindow,
}

impl SkillWindow {
    pub fn new(gd: &GameData) -> SkillWindow {
        let rect: Rect = UI_CFG.skill_window.rect.into();
        let chara = gd.chara.get(::common::gamedata::chara::CharaId::Player);
        let mut choices: Vec<ListRow> = Vec::new();
        for (skill_kind, level) in &chara.skills.skills {
            let e = if let Some(exp) = chara.skills.exp.as_ref() {
                if let Some(e) = exp.get(skill_kind) {
                    *e
                } else {
                    0
                }
            } else {
                0
            };
            let s = format!("{} {}  {}%", skill_kind.to_text(), level, e / 100);
            choices.push(ListRow::Str(s));
        }
        let choose_window = PagedChooseWindow::new(
            rect, choices, UI_CFG.skill_window.n_row, None);
        
        SkillWindow { choose_window }
    }
}

impl Window for SkillWindow {
    fn draw(
        &mut self, canvas: &mut WindowCanvas, game: &Game, sv: &mut SdlValues,
        anim: Option<(&Animation, u32)>) {

        self.choose_window.draw(canvas, game, sv, anim);
    }
}

impl DialogWindow for SkillWindow {
    fn process_command(&mut self, command: &Command, pa: &mut DoPlayerAction) -> DialogResult {
        match self.choose_window.process_command(&command, pa) {
            DialogResult::Close => DialogResult::Close,
            _ => DialogResult::Continue
        }
    }

    fn mode(&self) -> InputMode {
        InputMode::Dialog
    }
}
