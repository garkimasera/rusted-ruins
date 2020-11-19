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
                    match *s {
                        // Update left sleeping turn
                        CharaStatus::Asleep { ref mut turn_left } => {
                            if turn_left_new > *turn_left {
                                *turn_left = turn_left_new;
                            }
                            return;
                        }
                        _ => (),
                    }
                }
            }
            CharaStatus::Poisoned => {
                for s in self.status.iter_mut() {
                    match *s {
                        CharaStatus::Poisoned => {
                            return;
                        }
                        _ => (),
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

pub trait CharaStatusEx {
    fn about_sp(&self) -> bool;
    fn about_encumbrance(&self) -> bool;
    fn advance_turn(&mut self, n: u16);
    /// If this status is expired, returns true.
    /// Expired status will be removed from character.
    fn is_expired(&self) -> bool;
    fn expire(self, gd: &mut GameData, cid: CharaId);
}

macro_rules! impl_chara_status_ex {
    ($($e:ident),*) => {
        fn advance_turn(&mut self, n: u16) {
            match self {
                $(CharaStatus::$e { ref mut turn_left, .. } => {
                    if *turn_left > n {
                        *turn_left -= n;
                    } else {
                        *turn_left = 0;
                    }
                })*
                _ => (),
            }
        }

        fn is_expired(&self) -> bool {
            match *self {
                $(CharaStatus::$e { turn_left, .. } if turn_left == 0 => true,)*
                _ => false,
            }
        }
    }
}

impl CharaStatusEx for CharaStatus {
    fn about_sp(&self) -> bool {
        match *self {
            CharaStatus::Hungry | CharaStatus::Weak | CharaStatus::Starving => true,
            _ => false,
        }
    }

    fn about_encumbrance(&self) -> bool {
        match *self {
            CharaStatus::Burdened
            | CharaStatus::Strained
            | CharaStatus::Stressed
            | CharaStatus::Overloaded => true,
            _ => false,
        }
    }

    fn expire(self, gd: &mut GameData, cid: CharaId) {
        match self {
            CharaStatus::Work { work, .. } => match work {
                Work::Creation {
                    kind,
                    recipe,
                    ingredients,
                } => {
                    assert!(cid == CharaId::Player);
                    crate::game::creation::finish_creation(gd, kind, &recipe, ingredients);
                }
                Work::Harvest { item_idx, il } => {
                    crate::game::action::harvest::finish_harvest(gd, cid, item_idx, il);
                }
            },
            _ => (),
        }
    }

    impl_chara_status_ex!(Asleep, Work);
}
