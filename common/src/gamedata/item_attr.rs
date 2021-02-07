use super::item::*;

/// Items can have zero or more attributes.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub enum ItemAttr {
    /// Image variation
    ImageVariation(u32),
    /// Number of charges
    Charge { n: u32 },
    /// Data to generate the contents.
    /// Used to fix generated contents when this item is opened.
    ContentGen { level: u32, seed: u32 },
    /// Material of this item.
    Material(MaterialName),
    /// For skill learning items.
    SkillLearning(super::skill::SkillKind),
    /// Title for readable item.
    Title(String),
}
