//! Functions for character status operation

use common::gamedata::chara::*;

pub trait CharaStatusOperation {
    fn add_status(&mut self, new_status: CharaStatus);
    fn remove_sp_status(&mut self);
}

impl CharaStatusOperation for Chara {
    fn add_status(&mut self, new_status: CharaStatus) {

        match new_status {
            CharaStatus::Hungry => {
                self.remove_sp_status();
            }
            CharaStatus::Asleep { turn_left: turn_left_new }=> {
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
        }
        self.status.push(new_status);
    }

    // Remove sp status
    fn remove_sp_status(&mut self) {
        self.status.retain(|s| !s.about_sp());
    }
}

pub trait CharaStatusEx {
    fn about_sp(&self) -> bool;
    fn advance_turn(&mut self, n: u16);
    fn is_expired(&self) -> bool;
}

impl CharaStatusEx for CharaStatus {
    fn about_sp(&self) -> bool {
        match *self {
            CharaStatus::Hungry => true,
            _ => false,
        }
    }

    fn advance_turn(&mut self, n: u16) {
        match *self {
            CharaStatus::Asleep { ref mut turn_left } => {
                if *turn_left > n {
                    *turn_left -= n;
                } else {
                    *turn_left = 0;
                }
            }
            _ => (),
        }
    }

    /// If this status is expired, returns true.
    /// Expired status will be removed from character.
    fn is_expired(&self) -> bool {
        match *self {
            CharaStatus::Asleep { turn_left } if turn_left == 0 => true,
            _ => false,
        }
    }
}

