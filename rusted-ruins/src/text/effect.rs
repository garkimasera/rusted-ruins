use common::gamedata::{Effect, EffectKind};

use super::{misc_txt, ToText};

pub const UI_IMG_ID_ITEM_INFO: &str = "!icon-item-info";

#[derive(Debug)]
pub struct EffectText {
    power_factor: f32,
}

impl Default for EffectText {
    fn default() -> Self {
        Self::new()
    }
}

impl EffectText {
    pub fn new() -> Self {
        EffectText { power_factor: 1.0 }
    }

    pub fn effect_kind(
        &self,
        effect_kind: &EffectKind,
        power: f32,
        hit: f32,
    ) -> (&'static str, String) {
        let power = format!("{:.0}", power * self.power_factor);
        let hit = format!("{hit:.0}");

        match effect_kind {
            EffectKind::None => ("!", misc_txt("effect_kind-none")),
            EffectKind::RestoreHp => ("!", misc_txt_format!("effect_kind-restore_hp"; power=power)),
            EffectKind::RestoreSp => ("!", misc_txt_format!("effect_kind-restore_sp"; power=power)),
            EffectKind::RestoreMp => ("!", misc_txt_format!("effect_kind-restore_mp"; power=power)),
            EffectKind::Melee { .. } => ("!", misc_txt_format!("effect_kind-melee"; power=power)),
            EffectKind::Ranged { .. } => ("!", misc_txt_format!("effect_kind-ranged"; power=power)),
            EffectKind::Explosion { .. } => {
                ("!", misc_txt_format!("effect_kind-explosion"; power=power))
            }
            EffectKind::Direct { .. } => ("!", misc_txt_format!("effect_kind-direct"; power=power)),
            EffectKind::Status { status } => (
                "!",
                misc_txt_format!("effect_kind-status"; status=status, hit=hit),
            ),
            EffectKind::WallDamage => ("!", misc_txt("effect_kind-wall_damage")),
            EffectKind::CharaScan => ("!", misc_txt("effect_kind-chara_scan")),
            EffectKind::SkillLearning { .. } => ("!", misc_txt("effect_kind-skill_learning")),
            EffectKind::PlaceTile { .. } => ("!", misc_txt("effect_kind-place_tile")),
            EffectKind::GenItem { .. } => ("!", misc_txt("effect_kind-gen_item")),
        }
    }

    pub fn effect(&self, effect: &Effect) -> Vec<(&'static str, String)> {
        let power: f32 = effect.base_power.0.into();
        let hit: f32 = effect.hit.into();
        effect
            .kind
            .iter()
            .map(|effect_kind| self.effect_kind(effect_kind, power, hit))
            .collect()
    }
}
