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

    let power = crate::game::ability::calc_power(&game.gd, ability, cid) * ability.power;
    let hit_power = ability.hit_power;

    let chara = game.gd.chara.get(cid);
    trace!(
        "{} uses active skill \"{}\", power = {}, hit_power = {}",
        chara.to_text(),
        ability_id,
        power,
        hit_power,
    );

    match ability.group {
        AbilityGroup::Magic => {
            game_log!("use-ability-magic"; chara=chara, ability=ability_id);
        }
        AbilityGroup::Special => {
            game_log!("use-ability-special"; chara=chara, ability=ability_id);
        }
    }

    do_effect(game, &ability.effect, Some(cid), target, power, hit_power);
    true
}

pub fn calc_power(gd: &GameData, ability: &'static Ability, cid: CharaId) -> f32 {
    match ability.power_calc {
        PowerCalcMethod::Num(n) => n,
        PowerCalcMethod::Magic => {
            let chara = gd.chara.get(cid);
            let skill_level = chara.skill_level(SkillKind::MagicDevice) as f32;
            let int = chara.attr.int as f32;
            skill_level * int
        }
        _ => todo!(),
    }
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
