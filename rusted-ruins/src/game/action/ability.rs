use super::Game;
use crate::game::effect::do_effect;
use crate::game::power::{calc_hit, calc_power};
use crate::game::{extrait::*, Target};
use crate::text::ToText;
use common::gamedata::*;
use rules::RULES;

/// Return true if success.
pub fn use_ability<T: Into<Target>>(
    game: &mut Game,
    ability_id: &AbilityId,
    cid: CharaId,
    target: T,
) -> bool {
    let ability = if let Some(ability) = RULES.abilities.get(ability_id) {
        ability
    } else {
        warn!("unknown ability \"{}\"", ability_id);
        return false;
    };

    if !super::ability::usable(&game.gd, cid, ability_id, false) {
        return false;
    }

    let chara = game.gd.chara.get(cid);
    if !chara.ability_available(ability) {
        return false;
    }

    let power = calc_power(&game.gd, cid, &ability.power_calc);
    let hit = calc_hit(&game.gd, cid, &ability.power_calc);

    let chara = game.gd.chara.get(cid);
    trace!(
        "{} uses active skill \"{}\", power = {}",
        chara.to_text(),
        ability_id,
        power,
    );

    let use_ability_text_id = format!("use-ability-{}", ability.category);
    game_log!(&use_ability_text_id; chara=chara, ability=ability_id);

    do_effect(game, &ability.effect, Some(cid), target, power, hit);

    let chara = game.gd.chara.get_mut(cid);
    chara.sp -= ability.cost_sp as f32;
    chara.mp -= ability.cost_mp as i32;

    true
}

pub fn usable(gd: &GameData, cid: CharaId, ability_id: &AbilityId, print_log: bool) -> bool {
    if RULES.abilities.get(ability_id).is_none() {
        warn!("unknown ability \"{}\"", ability_id);
        return false;
    };
    let chara = gd.chara.get(cid);
    let cost = cost(gd, cid, ability_id);

    if chara.sp < cost.sp {
        if print_log {
            game_log!("ability-not-enough-sp"; chara=chara);
        }
        return false;
    }

    if chara.mp < cost.mp {
        if print_log {
            game_log!("ability-not-enough-mp"; chara=chara);
        }
        return false;
    }
    true
}

#[derive(Clone, Copy, Default, Debug)]
pub struct AbilityCost {
    pub sp: f32,
    pub mp: i32,
}

pub fn cost(_gd: &GameData, _cid: CharaId, ability_id: &AbilityId) -> AbilityCost {
    let ability = if let Some(ability) = RULES.abilities.get(ability_id) {
        ability
    } else {
        warn!("unknown ability \"{}\"", ability_id);
        return AbilityCost::default();
    };
    AbilityCost {
        sp: ability.cost_sp as f32,
        mp: ability.cost_mp as i32,
    }
}
