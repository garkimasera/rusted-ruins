use super::commonuse::*;
use crate::game::command::MouseButton;
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
        _pa: &mut DoPlayerAction<'_, '_>,
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
                if *button != MouseButton::Left {
                    return DialogResult::Command(None);
                }
                let i = (*x - self.rect.x) as u32 / cfg.icon_w;
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
            }
            _ => (),
        }

        DialogResult::Continue
    }

    fn mode(&self) -> InputMode {
        InputMode::Dialog
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

    fn mode(&self) -> InputMode {
        InputMode::Dialog
    }
}
