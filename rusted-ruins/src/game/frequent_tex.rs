
use common::objholder::*;
use common::gobj;

/// Holds frequent used texture's ids
pub struct FrequentTextures {
    effect_idx: Vec<EffectIdx>,
}

impl FrequentTextures {
    pub fn new() -> FrequentTextures {
        // Set Effect Object indices
        let mut effect_idx = Vec::new();
        effect_idx.push(gobj::id_to_idx("overlay-fog")); // Fog
        effect_idx.push(gobj::id_to_idx("overlay-fog-dark")); // Fog (dark)
        effect_idx.push(gobj::id_to_idx("overlay-night")); // Night

        FrequentTextures {
            effect_idx
        }
    }

    pub fn overlay_idx(&self, o: Overlay) -> EffectIdx {
        self.effect_idx[o as usize]
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Overlay {
    Fog = 0,
    FogDark,
    Night,
}

