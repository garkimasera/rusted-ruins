//! Functions for character status operation

use common::gamedata::chara::*;

pub trait CharaStatusOperation {
    fn add_status(&mut self, new_status: CharaStatus);
}

impl CharaStatusOperation for Chara {
    fn add_status(&mut self, new_status: CharaStatus) {
        let status = &mut self.status;

        match new_status {
            CharaStatus::Hungry => {
                status.retain(|s| !s.about_nutrition()); // Remove nutrition status
                status.push(new_status);
            }
            CharaStatus::Asleep { turn_left: turn_left_new }=> {
                for s in status.iter_mut() {
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
                for s in status.iter_mut() {
                    match *s {
                        CharaStatus::Poisoned => {
                            return;
                        }
                        _ => (),
                    }
                }
            }
        }
        status.push(new_status);
    }
}

pub trait CharaStatusEx {
    fn about_nutrition(&self) -> bool;
    fn advance_turn(&mut self, n: u16);
    fn is_expired(&self) -> bool;
}

impl CharaStatusEx for CharaStatus {
    fn about_nutrition(&self) -> bool {
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

