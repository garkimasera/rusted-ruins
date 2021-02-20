use crate::game::Game;
use common::gamedata::*;
use geom::*;

#[derive(Clone, Copy, Debug)]
pub enum Target {
    None,
    Tile(Vec2d),
    Chara(CharaId),
}

impl From<Vec2d> for Target {
    fn from(pos: Vec2d) -> Target {
        Target::Tile(pos)
    }
}

impl From<CharaId> for Target {
    fn from(cid: CharaId) -> Target {
        Target::Chara(cid)
    }
}

// pub fn auto_target(game: &Game, cid: CharaId, effect: &Effect) -> Option<Target> {
//     if effect.target_mode == TargetMode::None {
//         return Target::None;
//     }
//     if cid == CharaId::Player {
//         match auto_target_for_player(game, effect) {
//             Ok(target) => { return target; }
//             Err(_) => (),
//         }
//     }
//     todo!();
// }

pub fn auto_target_for_player(game: &Game, effect: &Effect) -> Result<Option<Target>, ()> {
    match effect.target_mode {
        TargetMode::Ally => Err(()),
        TargetMode::Enemy => {
            if let Some(target) = game.target_chara() {
                Ok(Some(target.into()))
            } else {
                Err(())
            }
        }
        TargetMode::None => Ok(Some(Target::None)),
    }
}
