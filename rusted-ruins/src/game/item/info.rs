use crate::text::{misc_txt, ToText};
use common::gamedata::*;
use common::gobj;

#[derive(Default, Debug)]
pub struct ItemInfoText {
    pub item_name: String,
    pub item_kind: String,
    pub basic_desc: String,
}

impl ItemInfoText {
    pub fn new(item: &Item) -> ItemInfoText {
        let idx = item.idx;
        let obj = gobj::get_obj(idx);
        let mut info = ItemInfoText::default();

        info.item_name = item.to_text().into_owned();
        info.item_kind = replace_str!(misc_txt("!item_info_text.item_kind"); item_kind=obj.kind);

        match obj.kind {
            ItemKind::Potion | ItemKind::Herb | ItemKind::Food => {
                info.basic_desc = replace_str!(
                    misc_txt("!item_info_text.basic_desc.food_and_drink");
                    medical_effect=obj.medical_effect);
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
