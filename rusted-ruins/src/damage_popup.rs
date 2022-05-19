use crate::config::UI_CFG;
use common::gamedata::*;
use geom::Coords;
use once_cell::sync::Lazy;
use std::collections::VecDeque;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

static DAMAGE_POPUP: Lazy<RwLock<DamagePopupList>> =
    Lazy::new(|| RwLock::new(DamagePopupList::new()));

pub struct DamagePopupList {
    pub popup_list: Vec<DamagePopup>,
}

pub enum PopupKind {
    Damage(i32),
    Heal(i32),
    Miss,
}

pub struct DamagePopup {
    pub cid: CharaId,
    pub pos: Coords,
    /// (passed_frame, popup)
    pub queue: VecDeque<(u32, PopupKind)>,
}

pub fn get() -> RwLockReadGuard<'static, DamagePopupList> {
    DAMAGE_POPUP.try_read().expect("failed lock COMBAT_LOG")
}

fn get_mut() -> RwLockWriteGuard<'static, DamagePopupList> {
    DAMAGE_POPUP.try_write().expect("failed lock COMBAT_LOG")
}

pub fn push(cid: CharaId, pos: Coords, popup: PopupKind) {
    get_mut().push(cid, pos, popup);
}

pub fn advance_frame() {
    get_mut().advance();
}

impl DamagePopupList {
    pub fn new() -> DamagePopupList {
        DamagePopupList {
            popup_list: Vec::new(),
        }
    }

    pub fn push(&mut self, cid: CharaId, pos: Coords, popup_kind: PopupKind) {
        if let Some(popup) = self
            .popup_list
            .iter_mut()
            .find(|damaged_chara| damaged_chara.cid == cid)
        {
            popup.pos = pos;
            popup.queue.push_front((0, popup_kind));
        } else {
            let mut queue = VecDeque::new();
            queue.push_front((0, popup_kind));

            self.popup_list.push(DamagePopup { cid, pos, queue });
        }
    }

    fn advance(&mut self) {
        for popup in &mut self.popup_list {
            for (passed_frame, _) in &mut popup.queue {
                *passed_frame += 1;
            }
            popup
                .queue
                .retain(|(passed_frame, _)| *passed_frame < UI_CFG.damage_popup.n_frame);
        }

        self.popup_list.retain(|popup| !popup.queue.is_empty());
    }
}
