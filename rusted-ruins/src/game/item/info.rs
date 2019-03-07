use crate::text::{misc_txt, ToText};
use common::gamedata::*;
use common::gobj;
use common::objholder::UIImgIdx;

const UI_IMG_ID_ITEM_INFO: &str = "!icon-item-info";

#[derive(Default, Debug)]
pub struct ItemInfoText {
    pub item_name: String,
    pub item_kind: String,
    pub desc_text: Vec<(&'static str, String)>,
}

impl ItemInfoText {
    pub fn new(item: &Item) -> ItemInfoText {
        let idx = item.idx;
        let obj = gobj::get_obj(idx);
        let mut info = ItemInfoText::default();

        let item_name = item.to_text().into_owned();
        let item_kind = replace_str!(misc_txt("!item_info_text.item_kind"); item_kind=obj.kind);
        let mut desc_text = Vec::new();

        match obj.kind {
            ItemKind::Potion | ItemKind::Herb | ItemKind::Food => {
                desc_text.push((UI_IMG_ID_ITEM_INFO, "Very".to_owned()));
                let t = replace_str!(
                    misc_txt("!item_info_text.basic_desc.food_and_drink");
                    medical_effect=obj.medical_effect);
                desc_text.push((UI_IMG_ID_ITEM_INFO, t));
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

        ItemInfoText {
            item_name,
            item_kind,
            desc_text,
        }
    }
}

