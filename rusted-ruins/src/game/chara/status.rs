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
                for s in status {
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
            CharaStatus::Poisoning => {
                for s in status {
                    match *s {
                        CharaStatus::Poisoning => {
                            return;
                        }
                        _ => (),
                    }
                }
            }
        }
    }
}

pub trait CharaStatusEx {
    fn about_nutrition(&self) -> bool;
}

impl CharaStatusEx for CharaStatus {
    fn about_nutrition(&self) -> bool {
        match *self {
            CharaStatus::Hungry => true,
            _ => false,
        }
    }
}

