use crate::basic::ArrayStringId;
use crate::gamedata::faction::FactionId;
use crate::gamedata::map::SiteSymbolKind;
use crate::gamedata::site::SiteKind;
use crate::gamedata::Reward;
use crate::hashmap::HashMap;
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
    // pub random_npcs: Vec<>,
    pub shops: HashMap<NpcGenId, ShopGenData>,
    pub quests: Vec<QuestGenData>,
    /// Delivery chest potision and object id for town sites
    pub delivery_chest: Option<(u32, Vec2d, String)>,
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
    pub talk_script: String,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum NpcGenId {
    Site(u32),
    Unique(ArrayStringId),
}

/// Data to generate a shop on the site
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct ShopGenData {
    #[serde(default)]
    pub shop_kind: String,
    #[serde(default)]
    pub item_selector: ItemSelector,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum QuestGenData {
    ItemDelivering {
        #[serde(default = "quest_gen_data_default_weight")]
        weight: f32,
        text_id: String,
        deadline: u32,
        reward: Reward,
        items: Vec<(ItemSelector, u32)>,
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
