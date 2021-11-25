use crate::basic::ArrayStringId;
use crate::gamedata::faction::FactionId;
use crate::gamedata::map::SiteSymbolKind;
use crate::gamedata::site::SiteKind;
use crate::gamedata::Reward;
use crate::item_selector::ItemSelector;
use geom::Vec2d;

/// Hold data for site generation
#[derive(Clone, Serialize, Deserialize)]
pub struct SiteGenObject {
    pub id: String,
    pub kind: SiteKind,
    pub site_symbol: SiteSymbolKind,
    pub default_faction_id: FactionId,
    pub map_template_id: Vec<String>,
    pub npcs: Vec<NpcGenData>,
    /// pub random_npcs: Vec<>,
    pub shops: Vec<ShopGenData>,
    pub quests: Vec<QuestGenData>,
}

/// Data to generate a unique citizen
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct NpcGenData {
    /// Unique id in this site
    pub id: NpcGenId,
    pub pos: Vec2d,
    pub floor: u32,
    #[serde(default)]
    pub name: String,
    pub chara_template_id: String,
    #[serde(default)]
    pub talk_script_id: String,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum NpcGenId {
    Site(u32),
    Unique(ArrayStringId),
}

/// Data to generate a shop on the site
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct ShopGenData {
    /// Shopkeeper's id
    pub chara_id: u32,
    #[serde(default)]
    pub shop_kind: String,
    #[serde(default)]
    pub selector: String,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum QuestGenData {
    ItemDelivering {
        #[serde(default = "quest_gen_data_default_weight")]
        weight: f32,
        text_id: String,
        deadline: u32,
        reward: Reward,
        item: ItemSelector,
        n: u32,
    },
}

impl QuestGenData {
    pub fn weight(&self) -> f32 {
        match self {
            Self::ItemDelivering { weight, .. } => *weight,
        }
    }
}

fn quest_gen_data_default_weight() -> f32 {
    1.0
}
