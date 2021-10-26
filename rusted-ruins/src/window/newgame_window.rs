use std::cell::{Cell, RefCell};
use std::rc::Rc;

use super::commonuse::*;
use super::group_window::GroupWindow;
use super::list_desc_window::ListWithDescWindow;
use super::text_input_dialog::TextInputDialog;
use super::text_window::{ScrollingTextWindow, TextWindow};
use super::widget::*;
use super::SpecialDialogResult;
use crate::config::{SCREEN_CFG, UI_CFG};
use crate::game::newgame::NewGameBuilder;
use crate::text::{self, misc_txt};
use crate::window::status_window::create_status_window_group;
use common::basic::{TAB_ICON_H, TAB_TEXT_H};
use common::gamedata::{CharaId, GameData};
use common::gobj;
use rules::RULES;

pub struct NewGameWindow {
    back_image: ImageWidget,
}

impl NewGameWindow {
    pub fn new() -> NewGameWindow {
        let rect = Rect::new(0, 0, SCREEN_CFG.screen_w, SCREEN_CFG.screen_h);

        NewGameWindow {
            back_image: ImageWidget::ui_img(rect, "!title-screen"),
        }
    }

    pub fn draw(
        &mut self,
        context: &mut Context<'_, '_, '_, '_>,
        _game: &Game<'_>,
        _anim: Option<(&Animation, u32)>,
    ) {
        self.back_image.draw(context);
    }
}

pub struct DummyNewGameDialog {
    builder: Rc<RefCell<NewGameBuilder>>,
    page: NewGameBuildPage,
    next_button: ButtonWidget,
    back_button: ButtonWidget,
    explanation_text: TextWindow,
    choose_class_dialog: ChooseClassDialog,
    name_input_dialog: TextInputDialog,
    player_info_window: Option<GroupWindow>,
    opening_text: Option<ScrollingTextWindow>,
    gd: Option<Box<GameData>>,
}

impl DummyNewGameDialog {
    pub fn new() -> DummyNewGameDialog {
        let builder = Rc::new(RefCell::new(NewGameBuilder::default()));
        let (next_button_rect, back_button_rect) = Self::button_rect(0);
        let next_button = ButtonWidget::new(next_button_rect, "Next", FontKind::M);
        let back_button = ButtonWidget::new(back_button_rect, "Back", FontKind::M);
        let explanation_text =
            TextWindow::new(UI_CFG.newgame_dialog.explanation_text_rect.into(), "");

        let mut name_input_dialog = TextInputDialog::new();
        let b = builder.clone();
        name_input_dialog.set_cb_text_changed(move |_, text| {
            b.borrow_mut().set_player_name(text);
        });

        let mut dialog = DummyNewGameDialog {
            builder: builder.clone(),
            explanation_text,
            next_button,
            back_button,
            page: NewGameBuildPage::default(),
            choose_class_dialog: ChooseClassDialog::new(builder),
            name_input_dialog,
            player_info_window: None,
            opening_text: None,
            gd: None,
        };
        dialog.update();
        dialog
    }

    fn button_rect(y: i32) -> (Rect, Rect) {
        let ui_cfg = &UI_CFG.newgame_dialog;
        let back_button_x = SCREEN_CFG.screen_w / 2 - ui_cfg.button_space / 2 - ui_cfg.button_w;
        let next_button_x = SCREEN_CFG.screen_w / 2 + ui_cfg.button_space / 2;
        (
            Rect::new(next_button_x as _, y, ui_cfg.button_w, ui_cfg.button_h),
            Rect::new(back_button_x as _, y, ui_cfg.button_w, ui_cfg.button_h),
        )
    }

    fn update(&mut self) {
        let (top, bottom) = self.page.top_bottom();

        self.explanation_text
            .set_text(&text::ui_txt(self.page.explanation_text()));
        self.explanation_text
            .move_to(None, Some(top - UI_CFG.newgame_dialog.top_margin));

        let (next_button_rect, back_button_rect) =
            Self::button_rect(bottom + UI_CFG.newgame_dialog.bottom_margin);
        self.next_button
            .move_to(next_button_rect.x, next_button_rect.y);
        self.back_button
            .move_to(back_button_rect.x, back_button_rect.y);
    }
}

impl Window for DummyNewGameDialog {
    fn draw(
        &mut self,
        context: &mut Context<'_, '_, '_, '_>,
        game: &Game<'_>,
        anim: Option<(&Animation, u32)>,
    ) {
        if let Some(opening_text) = self.opening_text.as_mut() {
            opening_text.draw(context, game, anim);
            return;
        }

        self.update_player_info(game);
        if self.next_ok() {
            self.next_button.draw(context);
        }
        self.back_button.draw(context);
        self.explanation_text.draw(context, game, anim);

        match self.page {
            NewGameBuildPage::ChooseClass => self.choose_class_dialog.draw(context, game, anim),
            NewGameBuildPage::PlayerNameInput => self.name_input_dialog.draw(context, game, anim),
            NewGameBuildPage::PlayerInfo => self
                .player_info_window
                .as_mut()
                .unwrap()
                .draw(context, game, anim),
        }
    }
}

impl DialogWindow for DummyNewGameDialog {
    fn process_command(
        &mut self,
        command: &Command,
        pa: &mut DoPlayerAction<'_, '_>,
    ) -> DialogResult {
        if let Some(opening_text) = self.opening_text.as_mut() {
            if (matches!(command, Command::Enter) && opening_text.is_finished())
                || matches!(command, Command::Cancel)
            {
                return DialogResult::Special(SpecialDialogResult::NewGameStart(
                    self.gd.take().unwrap(),
                ));
            } else {
                return DialogResult::Continue;
            }
        }

        self.update_player_info(pa.game());
        let mut page_dialog_result = None;

        if *command == Command::Cancel
            || self.back_button.process_command(command) == Some(true)
            || {
                page_dialog_result = Some(self.process_command_page(command, pa));
                self.page != NewGameBuildPage::PlayerNameInput
                    && matches!(page_dialog_result, Some(DialogResult::Close))
            }
        {
            if let Some(back_page) = self.page.back() {
                if self.page == NewGameBuildPage::PlayerInfo {
                    self.player_info_window = None;
                }
                self.page = back_page;
                self.update();
                return DialogResult::Continue;
            } else {
                return DialogResult::Special(SpecialDialogResult::ReturnToStartScreen);
            }
        }

        if (self.next_ok() && self.next_button.process_command(command) == Some(true))
            || (self.page == NewGameBuildPage::PlayerNameInput
                && matches!(page_dialog_result, Some(DialogResult::Close)))
        {
            if let Some(next_page) = self.page.next() {
                self.page = next_page;
                self.update();

                if next_page == NewGameBuildPage::PlayerInfo {
                    let gd = self.builder.borrow().build_with_player();
                    return DialogResult::Special(SpecialDialogResult::TempGameData(Box::new(gd)));
                }
            } else {
                let gd = self.builder.borrow().build_with_player();
                let gd = self.builder.borrow().build(gd);
                self.opening_text = opening_text_window();
                if self.opening_text.is_none() {
                    return DialogResult::Special(SpecialDialogResult::NewGameStart(Box::new(gd)));
                }
                self.gd = Some(Box::new(gd));
            }
        }

        DialogResult::Continue
    }

    fn mode(&self) -> InputMode {
        match self.page {
            NewGameBuildPage::PlayerNameInput { .. } => InputMode::TextInput,
            _ => InputMode::Dialog,
        }
    }
}

impl DummyNewGameDialog {
    fn next_ok(&self) -> bool {
        let builder = self.builder.borrow();

        match self.page {
            NewGameBuildPage::ChooseClass => {
                self.choose_class_dialog.current_choice.get().is_some()
            }
            NewGameBuildPage::PlayerNameInput => builder
                .player_name
                .as_ref()
                .map_or(false, |name| !name.trim().is_empty()),
            NewGameBuildPage::PlayerInfo => true,
        }
    }

    fn process_command_page(
        &mut self,
        command: &Command,
        pa: &mut DoPlayerAction<'_, '_>,
    ) -> DialogResult {
        match self.page {
            NewGameBuildPage::ChooseClass => self.choose_class_dialog.process_command(command, pa),
            NewGameBuildPage::PlayerNameInput => {
                if let DialogResult::Close = self.name_input_dialog.process_command(command, pa) {
                    let player_name = self.name_input_dialog.get_text();
                    if !player_name.is_empty() {
                        // If input text is valid for character name
                        self.builder.borrow_mut().set_player_name(player_name);
                        return DialogResult::Close;
                    }
                    self.name_input_dialog.restart();
                }
                DialogResult::Continue
            }
            NewGameBuildPage::PlayerInfo => self
                .player_info_window
                .as_mut()
                .unwrap()
                .process_command(command, pa),
        }
    }

    fn update_player_info(&mut self, game: &Game<'_>) {
        if self.page == NewGameBuildPage::PlayerInfo && self.player_info_window.is_none() {
            self.player_info_window =
                Some(create_status_window_group(game, CharaId::Player, false));
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum NewGameBuildPage {
    ChooseClass,
    PlayerNameInput,
    PlayerInfo,
}

impl Default for NewGameBuildPage {
    fn default() -> Self {
        Self::ChooseClass
    }
}

impl NewGameBuildPage {
    fn next(&self) -> Option<Self> {
        match self {
            Self::ChooseClass => Some(Self::PlayerNameInput),
            Self::PlayerNameInput => Some(Self::PlayerInfo),
            Self::PlayerInfo => None,
        }
    }

    fn back(&self) -> Option<Self> {
        match self {
            Self::ChooseClass => None,
            Self::PlayerNameInput => Some(Self::ChooseClass),
            Self::PlayerInfo => Some(Self::PlayerNameInput),
        }
    }

    fn explanation_text(&self) -> &'static str {
        match self {
            Self::ChooseClass => "newgame-chooseclass",
            Self::PlayerNameInput => "newgame-inputplayername",
            Self::PlayerInfo => "newgame-complete",
        }
    }

    fn top_bottom(&self) -> (i32, i32) {
        let rect: Rect = match self {
            Self::ChooseClass => UI_CFG.info_window.rect,
            Self::PlayerNameInput => UI_CFG.text_input_dialog.rect,
            Self::PlayerInfo => {
                let rect: Rect = UI_CFG.info_window.rect.into();
                return (rect.top() - (TAB_ICON_H + TAB_TEXT_H) as i32, rect.bottom());
            }
        }
        .into();
        (rect.top(), rect.bottom())
    }
}

pub struct ChooseClassDialog {
    window: ListWithDescWindow<(IconIdx, TextCache)>,
    current_choice: Rc<Cell<Option<u32>>>,
}

impl ChooseClassDialog {
    pub fn new(builder: Rc<RefCell<NewGameBuilder>>) -> ChooseClassDialog {
        let classes: Vec<_> = RULES
            .newgame
            .class_choices
            .iter()
            .map(|c| {
                (
                    icon_idx_empty(),
                    TextCache::new(
                        text::misc_txt(c.as_str()),
                        FontKind::M,
                        UI_CFG.color.normal_font,
                    ),
                )
            })
            .collect();

        let mut window = ListWithDescWindow::new(
            UI_CFG.info_window.rect.into(),
            UI_CFG.newgame_dialog.column_pos.clone(),
            classes,
        );

        window.set_cb_selection_changed(Box::new(|i, desc| {
            let desc_text_id = format!(
                "class-{}-desc",
                RULES.newgame.class_choices[i as usize].as_str()
            );
            desc.set_text(misc_txt(&desc_text_id));
        }));
        let current_choice = Rc::new(Cell::new(None));
        let c = current_choice.clone();
        window.set_cb_selected(Box::new(move |i, list| {
            if c.get() != Some(i) {
                list.get_item_mut(i).unwrap().0 = icon_idx_checked();
                if let Some(c) = c.get() {
                    list.get_item_mut(c).unwrap().0 = icon_idx_empty();
                }
                c.set(Some(i));
                builder
                    .borrow_mut()
                    .set_chara_class(RULES.newgame.class_choices[i as usize]);
            }
        }));

        ChooseClassDialog {
            window,
            current_choice,
        }
    }
}

impl Window for ChooseClassDialog {
    fn draw(
        &mut self,
        context: &mut Context<'_, '_, '_, '_>,
        game: &Game<'_>,
        anim: Option<(&Animation, u32)>,
    ) {
        self.window.draw(context, game, anim);
    }
}

impl DialogWindow for ChooseClassDialog {
    fn process_command(
        &mut self,
        command: &Command,
        pa: &mut DoPlayerAction<'_, '_>,
    ) -> DialogResult {
        self.window.process_command(command, pa)
    }
}

fn icon_idx_empty() -> IconIdx {
    IconIdx::UiImg {
        idx: gobj::id_to_idx("!"),
        i_pattern: 0,
    }
}

fn icon_idx_checked() -> IconIdx {
    IconIdx::UiImg {
        idx: gobj::id_to_idx("!icon-ok"),
        i_pattern: 0,
    }
}

/// Create scrolling text window that displays opening text
fn opening_text_window() -> Option<ScrollingTextWindow> {
    text::misc_txt_checked("op-scroll", None)
        .map(|t| ScrollingTextWindow::new(SCREEN_CFG.main_window.into(), &t))
}
