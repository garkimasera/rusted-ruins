use crate::config::UI_CFG;
use common::gamedata::*;
use geom::Vec2d;
use once_cell::sync::Lazy;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

static CHARA_LOG: Lazy<RwLock<CharaLog>> = Lazy::new(|| RwLock::new(CharaLog::new()));

pub struct CharaLog {
    pub damage_list: Vec<CharaLogDamage>,
}

pub struct CharaLogDamage {
    pub cid: CharaId,
    pub pos: Vec2d,
    pub damage: i32,
    pub passed_frame: u32,
}

pub fn advance_frame() {
    get_log_mut().advance();
}

pub fn get_log() -> RwLockReadGuard<'static, CharaLog> {
    CHARA_LOG.try_read().expect("failed lock COMBAT_LOG")
}

pub fn get_log_mut() -> RwLockWriteGuard<'static, CharaLog> {
    CHARA_LOG.try_write().expect("failed lock COMBAT_LOG")
}

impl CharaLog {
    pub fn new() -> CharaLog {
        CharaLog {
            damage_list: Vec::new(),
        }
    }

    pub fn push_damage(&mut self, cid: CharaId, pos: Vec2d, damage: i32) {
        if let Some(damaged_chara) = self
            .damage_list
            .iter_mut()
            .find(|damaged_chara| damaged_chara.cid == cid)
        {
            damaged_chara.damage += damage;
            damaged_chara.pos = pos;
            damaged_chara.passed_frame = 0;
        } else {
            self.damage_list.push(CharaLogDamage {
                cid,
                pos,
                damage,
                passed_frame: 0,
            });
        }
    }

    fn advance(&mut self) {
        for damaged_chara in &mut self.damage_list {
            damaged_chara.passed_frame += 1;
        }

        self.damage_list
            .retain(|damaged_chara| damaged_chara.passed_frame < UI_CFG.damage.n_frame);
    }
}
