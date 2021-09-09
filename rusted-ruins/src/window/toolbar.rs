use super::build_obj_dialog::BuildObjDialog;
use super::choose_window::{ChooseWindow, DefaultBehavior};
use super::commonuse::*;
use super::item_info_window::ItemInfoWindow;
use crate::game::command::MouseButton;
use crate::text::ui_txt;
use common::basic::MAX_ACTION_SHORTCUTS;
use common::gamedata::*;
use common::gobj;
use common::objholder::UiImgIdx;
use once_cell::sync::Lazy;

pub struct Toolbar {
    rect: Rect,
    mouseover: Option<u32>,
}

const ITEM_MELEE: u32 = 0;
const ITEM_SHOOT: u32 = 1;
const ITEM_TOOL: u32 = 2;
const N_ITEM: u32 = 3;

static ICON_FRAME: Lazy<UiImgIdx> = Lazy::new(|| gobj::id_to_idx("!toolbar-icon-frame"));

impl Toolbar {
    pub fn new() -> Toolbar {
        Self {
            rect: SCREEN_CFG.toolbar.into(),
            mouseover: None,
        }
    }
}

impl Window for Toolbar {
    fn draw(
        &mut self,
        context: &mut Context<'_, '_, '_, '_>,
        game: &Game<'_>,
        _anim: Option<(&Animation, u32)>,
    ) {
        let cfg = &UI_CFG.toolbar;

        context.fill_rect(self.rect, UI_CFG.color.toolbar_bg);

        for i in 0..N_ITEM {
            let rect = Rect::new(
                self.rect.x + cfg.icon_w as i32 * i as i32,
                self.rect.y,
                cfg.icon_w,
                cfg.icon_h,
            );
            context.set_viewport(rect);
            let rect = Rect::new(0, 0, cfg.icon_w, cfg.icon_h);

            match i {
                ITEM_MELEE => {
                    let player = game.gd.chara.get(CharaId::Player);
                    if let Some(item) = player.equip.item(EquipSlotKind::MeleeWeapon, 0) {
                        context.render_tex_n_center(item.idx, rect, 0);
                    }
                }
                ITEM_SHOOT => {
                    let player = game.gd.chara.get(CharaId::Player);
                    if let Some(item) = player.equip.item(EquipSlotKind::RangedWeapon, 0) {
                        context.render_tex_n_center(item.idx, rect, 0);
                    }
                }
                ITEM_TOOL => {
                    let player = game.gd.chara.get(CharaId::Player);
                    if let Some(item) = player.equip.item(EquipSlotKind::Tool, 0) {
                        context.render_tex_n_center(item.idx, rect, 0);
                    }
                }
                _ => unreachable!(),
            }

            // Draw icon frame
            let mouseover = if let Some(mouseover) = self.mouseover.as_ref() {
                if *mouseover == i {
                    1
                } else {
                    0
                }
            } else {
                0
            };
            context.render_tex_n(*ICON_FRAME, rect, mouseover);
        }
    }
}

impl DialogWindow for Toolbar {
    fn process_command(
        &mut self,
        command: &Command,
        pa: &mut DoPlayerAction<'_, '_>,
    ) -> DialogResult {
        let cfg = &UI_CFG.toolbar;

        match command {
            Command::MouseState { x, y, .. } => {
                self.mouseover = None;
                if self.rect.contains_point((*x, *y)) {
                    let i = (*x - self.rect.x) as u32 / cfg.icon_w;
                    if i < N_ITEM {
                        self.mouseover = Some(i);
                    }
                }
            }
            Command::MouseButtonDown { x, y, .. } => {
                if !self.rect.contains_point((*x, *y)) {
                    return DialogResult::Continue;
                }
                return DialogResult::Command(None);
            }
            Command::MouseButtonUp { x, y, button, .. } => {
                if !self.rect.contains_point((*x, *y)) {
                    return DialogResult::Continue;
                }
                let i = (*x - self.rect.x) as u32 / cfg.icon_w;
                if *button == MouseButton::Left {
                    match i {
                        ITEM_MELEE => {
                            return DialogResult::Command(Some(Command::ChangeEquip {
                                kind: EquipSlotKind::MeleeWeapon,
                            }));
                        }
                        ITEM_SHOOT => {
                            return DialogResult::Command(Some(Command::ChangeEquip {
                                kind: EquipSlotKind::RangedWeapon,
                            }));
                        }
                        ITEM_TOOL => {
                            return DialogResult::Command(Some(Command::ChangeEquip {
                                kind: EquipSlotKind::Tool,
                            }));
                        }
                        _ => (),
                    }
                } else if *button == MouseButton::Right {
                    if let Some(menu) = ToolbarMenu::new(pa.gd(), i, (*x, *y)) {
                        return DialogResult::OpenChildDialog(Box::new(menu));
                    } else {
                        return DialogResult::Command(None);
                    }
                } else {
                    return DialogResult::Command(None);
                }
            }
            _ => (),
        }

        DialogResult::Continue
    }
}

pub struct ShortcutList {
    rect: Rect,
    mouseover: Option<u32>,
    availability: Vec<Option<(bool, Option<u32>)>>,
}

impl ShortcutList {
    pub fn new() -> ShortcutList {
        Self {
            rect: SCREEN_CFG.shortcut_list.into(),
            mouseover: None,
            availability: vec![None; MAX_ACTION_SHORTCUTS],
        }
    }

    pub fn update(&mut self, gd: &GameData) {
        for (i, a) in self.availability.iter_mut().enumerate() {
            *a = gd.shortcut_available(i);
        }
    }
}

impl Window for ShortcutList {
    fn draw(
        &mut self,
        context: &mut Context<'_, '_, '_, '_>,
        game: &Game<'_>,
        _anim: Option<(&Animation, u32)>,
    ) {
        self.update(&game.gd);
        let cfg = &UI_CFG.toolbar;

        context.fill_rect(self.rect, UI_CFG.color.toolbar_bg);

        for i in 0..cfg.n_shortcut {
            let rect = Rect::new(
                self.rect.x + cfg.icon_w as i32 * i as i32,
                self.rect.y,
                cfg.icon_w,
                cfg.icon_h,
            );
            context.set_viewport(rect);
            let rect = Rect::new(0, 0, cfg.icon_w, cfg.icon_h);

            if let Some(action_shortcut) = game.gd.settings.action_shortcuts[i as usize] {
                match action_shortcut {
                    ActionShortcut::Throw(idx)
                    | ActionShortcut::Drink(idx)
                    | ActionShortcut::Eat(idx)
                    | ActionShortcut::Use(idx)
                    | ActionShortcut::Release(idx)
                    | ActionShortcut::Read(idx) => {
                        context.render_tex_n_center(idx, rect, 0);
                    }
                }
            }

            let mut icon_frame = 0;
            if let Some((available, _remaining)) = self.availability[i as usize] {
                if !available {
                    icon_frame += 2;
                }
            }

            // Draw icon frame
            if let Some(mouseover) = self.mouseover.as_ref() {
                if *mouseover == i {
                    icon_frame += 1;
                }
            }
            context.render_tex_n(*ICON_FRAME, rect, icon_frame);
        }
    }
}

impl DialogWindow for ShortcutList {
    fn process_command(
        &mut self,
        command: &Command,
        _pa: &mut DoPlayerAction<'_, '_>,
    ) -> DialogResult {
        let cfg = &UI_CFG.toolbar;

        match command {
            Command::MouseState { x, y, .. } => {
                self.mouseover = None;
                if self.rect.contains_point((*x, *y)) {
                    let i = (*x - self.rect.x) as u32 / cfg.icon_w;

                    if i < cfg.n_shortcut {
                        self.mouseover = Some(i);
                    }
                }
            }
            Command::MouseButtonDown { x, y, .. } => {
                if !self.rect.contains_point((*x, *y)) {
                    return DialogResult::Continue;
                }
                return DialogResult::Command(None);
            }
            Command::MouseButtonUp { x, y, button, .. } => {
                if !self.rect.contains_point((*x, *y)) {
                    return DialogResult::Continue;
                }
                if *button != MouseButton::Left {
                    return DialogResult::Command(None);
                }
                let i = (*x - self.rect.x) as u32 / cfg.icon_w;
                return DialogResult::Command(Some(Command::ActionShortcut(i as usize)));
            }
            _ => (),
        }

        DialogResult::Continue
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum ToolbarMenuItem {
    Information,
    SelectBuilding,
}

struct ToolbarMenu {
    choose_window: ChooseWindow,
    menu_items: Vec<ToolbarMenuItem>,
    esk: EquipSlotKind,
}

impl ToolbarMenu {
    pub fn new(gd: &GameData, i: u32, pos: (i32, i32)) -> Option<Self> {
        let winpos = WindowPos::from_left_top(pos.0, pos.1);

        let mut choices = Vec::new();
        let mut menu_items = Vec::new();

        let esk = match i {
            ITEM_MELEE => EquipSlotKind::MeleeWeapon,
            ITEM_SHOOT => EquipSlotKind::RangedWeapon,
            ITEM_TOOL => EquipSlotKind::Tool,
            _ => unreachable!(),
        };

        let item = gd.get_equip_list(CharaId::Player).item(esk, 0)?;
        let item_obj = item.obj();

        if let Some(use_effect) = find_attr!(item_obj, ItemObjAttr::Use(use_effect)) {
            if *use_effect == UseEffect::SelectBuilding {
                choices.push(ui_txt("item_menu-select-building"));
                menu_items.push(ToolbarMenuItem::SelectBuilding);
            }
        }

        // Item information.
        choices.push(ui_txt("item_menu-information"));
        menu_items.push(ToolbarMenuItem::Information);

        let choose_window = ChooseWindow::new(winpos, choices, DefaultBehavior::Close);

        Some(ToolbarMenu {
            choose_window,
            menu_items,
            esk,
        })
    }
}

impl Window for ToolbarMenu {
    fn draw(
        &mut self,
        context: &mut Context<'_, '_, '_, '_>,
        game: &Game<'_>,
        anim: Option<(&Animation, u32)>,
    ) {
        self.choose_window.draw(context, game, anim);
    }
}

impl DialogWindow for ToolbarMenu {
    fn process_command(
        &mut self,
        command: &Command,
        pa: &mut DoPlayerAction<'_, '_>,
    ) -> DialogResult {
        match self.choose_window.process_command(command, pa) {
            DialogResult::CloseWithValue(v) => {
                if let DialogCloseValue::Index(n) = v {
                    let item = self.menu_items[n as usize];
                    let gd = pa.gd();
                    let il = gd
                        .equipment_item_location(CharaId::Player, self.esk, 0)
                        .unwrap();

                    match item {
                        ToolbarMenuItem::Information => {
                            let info_win = ItemInfoWindow::new(il, pa.game());
                            DialogResult::CloseAndOpen(Box::new(info_win))
                        }
                        ToolbarMenuItem::SelectBuilding => {
                            let win = BuildObjDialog::new(il);
                            DialogResult::CloseAndOpen(Box::new(win))
                        }
                    }
                } else {
                    unreachable!()
                }
            }
            result => result,
        }
    }
}
