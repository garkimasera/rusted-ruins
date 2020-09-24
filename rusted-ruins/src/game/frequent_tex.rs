use common::gobj;
use common::objholder::*;

/// Holds frequent used texture's ids
pub struct FrequentTextures {
    effect_idx: Vec<EffectImgIdx>,
}

impl FrequentTextures {
    pub fn new() -> FrequentTextures {
        // Set Effect Object indices
        let mut effect_idx = Vec::new();
        effect_idx.push(gobj::id_to_idx("overlay-fog")); // Fog
        effect_idx.push(gobj::id_to_idx("overlay-fog-dark")); // Fog (dark)
        effect_idx.push(gobj::id_to_idx("overlay-night")); // Night
        effect_idx.push(gobj::id_to_idx("overlay-twilight0")); // Twilight (0 is darkest)
        effect_idx.push(gobj::id_to_idx("overlay-twilight1"));
        effect_idx.push(gobj::id_to_idx("overlay-twilight2"));

        FrequentTextures { effect_idx }
    }

    pub fn overlay_idx(&self, o: Overlay) -> EffectImgIdx {
        self.effect_idx[o as usize]
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Overlay {
    Fog = 0,
    _FogDark,
    Night,
    Twilight0, // Darkest
    Twilight1,
    Twilight2,
}
