
use common::gamedata::*;
use common::gobj;
use crate::text::{ToText, misc_txt};

#[derive(Default, Debug)]
pub struct ItemInfoText {
    pub item_name: String,
    pub item_kind: String,
}

impl ItemInfoText {
    pub fn new(item: &Item) -> ItemInfoText {
        let idx = item.idx;
        let obj = gobj::get_obj(idx);
        let mut info = ItemInfoText::default();

        info.item_name = item.to_text().into_owned();
        let t = obj.kind.to_text();
        info.item_kind = replace_str!(misc_txt("!item_info_text.item_kind"); item_kind=&t);

        match obj.kind {
            ItemKind::Potion => {
            }
            ItemKind::Herb => {
            }
            ItemKind::Food => {
            }
            ItemKind::Weapon(_) => {
            }
            ItemKind::Armor(_) => {
            }
            ItemKind::Material => {
            }
            ItemKind::Special => {
            }
            ItemKind::Object => {
            }
        }

        info
    }
}

