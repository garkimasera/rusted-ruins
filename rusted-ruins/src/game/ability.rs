use super::effect::do_effect;
use super::extrait::*;
use super::{Game, InfoGetter};
use crate::text::ToText;
use common::gamedata::*;
use ordered_float::NotNan;
use rules::RULES;

/// Return true if success.
pub fn use_ability(game: &mut Game, ability_id: &AbilityId, cid: CharaId, target: CharaId) -> bool {
    if !game.gd.target_visible(cid, target) {
        return false;
    }

    let ability = if let Some(ability) = RULES.abilities.get(ability_id) {
        ability
    } else {
        warn!("unknown ability \"{}\"", ability_id);
        return false;
    };

    let chara = game.gd.chara.get(cid);
    if !chara.ability_available(ability) {
        return false;
    }

    let power = super::power::calc_power(&game.gd, cid, &ability.power_calc);
    let hit = super::power::calc_hit(&game.gd, cid, &ability.power_calc);

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
    true
}

#[extend::ext(pub)]
impl BasePower {
    fn calc(&self, factor: f32) -> f32 {
        let factor = NotNan::new(factor).unwrap();
        let base_power = self.0 * factor;
        let power_var = self.1 * factor;
        let power_min = std::cmp::max(base_power - power_var, NotNan::new(0.0).unwrap());
        let power_max = base_power + power_var;
        let power = if power_max > power_min {
            rng::gen_range(power_min..power_max)
        } else {
            power_min
        };
        power.into_inner()
    }

    fn calc_without_var(&self, factor: f32) -> f32 {
        self.0.into_inner() * factor
    }
}
