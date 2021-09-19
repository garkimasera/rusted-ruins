//! Functions for character status operation

use common::gamedata::*;

pub trait CharaStatusOperation {
    fn add_status(&mut self, new_status: CharaStatus);
    fn remove_sp_status(&mut self);
    fn remove_encumbrance_status(&mut self);
}

impl CharaStatusOperation for Chara {
    fn add_status(&mut self, new_status: CharaStatus) {
        match new_status {
            CharaStatus::Hungry | CharaStatus::Weak | CharaStatus::Starving => {
                if let Some((i, _)) = self.status.iter().enumerate().find(|(_, s)| s.about_sp()) {
                    self.status[i] = new_status;
                    return;
                }
            }
            CharaStatus::Asleep {
                turn_left: turn_left_new,
            } => {
                for s in self.status.iter_mut() {
                    if let CharaStatus::Asleep { ref mut turn_left } = *s {
                        if turn_left_new > *turn_left {
                            *turn_left = turn_left_new;
                        }
                        return;
                    }
                }
            }
            CharaStatus::Poisoned => {
                for s in self.status.iter_mut() {
                    if *s == CharaStatus::Poisoned {
                        return;
                    }
                }
            }
            CharaStatus::Burdened
            | CharaStatus::Strained
            | CharaStatus::Stressed
            | CharaStatus::Overloaded => {
                if let Some((i, _)) = self
                    .status
                    .iter()
                    .enumerate()
                    .find(|(_, s)| s.about_encumbrance())
                {
                    self.status[i] = new_status;
                    return;
                }
            }
            CharaStatus::Scanned => {
                if self.status.iter().all(|s| *s != CharaStatus::Scanned) {
                    self.status.push(new_status);
                }
                return;
            }
            _ => (),
        }
        self.status.push(new_status);
    }

    // Remove sp status
    fn remove_sp_status(&mut self) {
        self.status.retain(|s| !s.about_sp());
    }

    // Remove encumbrance status
    fn remove_encumbrance_status(&mut self) {
        self.status.retain(|s| !s.about_encumbrance());
    }
}

pub trait CharaStatusExt {
    fn about_sp(&self) -> bool;
    fn about_encumbrance(&self) -> bool;
    fn advance_turn(&mut self, n: u16);
    fn is_expired(&self) -> bool;
    fn expire(self, gd: &mut GameData, cid: CharaId);
}

impl CharaStatusExt for CharaStatus {
    fn about_sp(&self) -> bool {
        matches!(
            self,
            CharaStatus::Hungry | CharaStatus::Weak | CharaStatus::Starving
        )
    }

    fn about_encumbrance(&self) -> bool {
        matches!(
            *self,
            CharaStatus::Burdened
                | CharaStatus::Strained
                | CharaStatus::Stressed
                | CharaStatus::Overloaded
        )
    }

    fn advance_turn(&mut self, n: u16) {
        if let Some(turn_left) = self.turn_left_mut() {
            if *turn_left > n {
                *turn_left -= n;
            } else {
                *turn_left = 0;
            }
        }
    }

    /// If this status is expired, returns true.
    /// Expired status will be removed from character.
    fn is_expired(&self) -> bool {
        if let Some(turn_left) = self.turn_left() {
            turn_left == 0
        } else {
            false
        }
    }

    fn expire(self, gd: &mut GameData, cid: CharaId) {
        if let CharaStatus::Work { work, .. } = self {
            match work {
                Work::Creation {
                    kind,
                    recipe,
                    ingredients,
                    material,
                } => {
                    assert_eq!(cid, CharaId::Player);
                    crate::game::creation::finish_creation(
                        gd,
                        kind,
                        &recipe,
                        ingredients,
                        material,
                    );
                }
                Work::Harvest { item_idx, il } => {
                    crate::game::action::harvest::finish_harvest(gd, cid, item_idx, il);
                }
            }
        }
    }
}
