use super::defs::Element;
use super::skill::SkillKind;
use geom::ShapeKind;

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct Effect {
    pub kind: Vec<EffectKind>,
    pub target_mode: TargetMode,
    #[serde(default)]
    pub power_adjust: Vec<f32>,
    pub range: u32,
    pub shape: ShapeKind,
    pub size: u32,
    #[serde(default)]
    pub anim_kind: EffectAnimKind,
    #[serde(default)]
    pub anim_img: String,
    #[serde(default)]
    pub anim_img_shot: String,
    #[serde(default)]
    pub sound: String,
}

impl Default for Effect {
    fn default() -> Effect {
        Effect {
            kind: Vec::new(),
            target_mode: TargetMode::default(),
            power_adjust: Vec::new(),
            range: 1,
            shape: ShapeKind::OneTile,
            size: 1,
            anim_kind: EffectAnimKind::None,
            anim_img: "".into(),
            anim_img_shot: "".into(),
            sound: "".into(),
        }
    }
}

/// Effect defines the game effect of items, magics, or other active skills.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub enum EffectKind {
    None,
    RecoverHp,
    RecoverSp,
    RecoverMp,
    Melee { element: Element },
    Ranged { element: Element },
    Explosion { element: Element },
    Direct { element: Element },
    Status { status: StatusEffect },
    WallDamage,
    CharaScan,
    SkillLearning { skills: Vec<SkillKind> },
}

impl Default for EffectKind {
    fn default() -> Self {
        EffectKind::None
    }
}

/// Default kind for target selection, used by NPC AI.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub enum TargetMode {
    None,
    Enemy,
    Ally,
}

impl Default for TargetMode {
    fn default() -> Self {
        TargetMode::Enemy
    }
}

/// Effect defines the game effect of items, magics, or other active skills.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub enum StatusEffect {
    Asleep,
    Poison,
    Scanned,
}

/// Animation kind for this effect.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub enum EffectAnimKind {
    None,
    Tile,
    Chara,
    Shot,
}

impl Default for EffectAnimKind {
    fn default() -> Self {
        EffectAnimKind::None
    }
}
