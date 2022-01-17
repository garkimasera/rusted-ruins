use crate::game::extrait::*;
use crate::text::ToText;
use common::gamedata::*;

const UI_IMG_ID_ITEM_INFO: &str = "!icon-item-info";

#[derive(Default, Debug)]
pub struct ItemInfoText {
    pub item_name: String,
    pub item_kind: String,
    pub desc_text: Vec<(&'static str, String)>,
}

impl ItemInfoText {
    pub fn new(item: &Item) -> ItemInfoText {
        let obj = item.obj();

        let item_name = item.to_text().into_owned();
        let item_kind = misc_txt_format!("item_info_text-item_kind"; item_kind=obj.kind);
        let mut desc_text = Vec::new();

        match obj.kind {
            ItemKind::Potion | ItemKind::Food => {
                if let Some(nutrition) = find_attr!(obj, ItemObjAttr::Nutrition(nutrition)) {
                    let t = misc_txt_format!("item_info_text-nutrition"; nutrition=nutrition);
                    desc_text.push((UI_IMG_ID_ITEM_INFO, t));
                }

                if let Some(ItemObjAttr::Medical { .. }) = find_attr!(obj, ItemObjAttr::Medical) {
                    // let t = // TODO: Add text by its medical effect
                    // desc_text.push((UI_IMG_ID_ITEM_INFO, t));
                }
            }
            ItemKind::Throwing => {}
            ItemKind::MagicDevice => {}
            ItemKind::Weapon(weapon_kind) => {
                let power = if let Some(power) = find_attr!(obj, ItemObjAttr::WeaponPower(power)) {
                    power.calc_without_var(item.power_factor())
                } else {
                    0.0
                };
                if weapon_kind.is_melee() {
                    let t = misc_txt_format!(
                        "item_info_text-melee_weapon"; power=power);
                    desc_text.push((UI_IMG_ID_ITEM_INFO, t));
                } else {
                    let t = misc_txt_format!(
                        "item_info_text-ranged_weapon"; power=power);
                    desc_text.push((UI_IMG_ID_ITEM_INFO, t));
                }
            }
            ItemKind::Armor(_) => {
                let d0 = format!("{:+}", item.defence(Element::Physical));
                let d1 = format!("{:+}", item.defence(Element::Fire));
                let d2 = format!("{:+}", item.defence(Element::Cold));
                let d3 = format!("{:+}", item.defence(Element::Shock));
                let d4 = format!("{:+}", item.defence(Element::Poison));
                let d5 = format!("{:+}", item.defence(Element::Spirit));
                let t = misc_txt_format!(
                    "item_info_text-defence";
                    physical=d0, fire=d1, cold=d2, shock=d3, poison=d4, spirit=d5);
                desc_text.push((UI_IMG_ID_ITEM_INFO, t));
            }
            ItemKind::Tool => {}
            ItemKind::Container => {}
            ItemKind::Special => {}
            ItemKind::Readable => {}
            ItemKind::Material => {}
            ItemKind::Module => {}
            ItemKind::Object => {}
        }

        for attr in &item.attrs {
            if let ItemAttr::Material(material) = attr {
                let material_name = crate::text::prefix::material(*material);
                let t = misc_txt_format!(
                        "item_info_text-material"; material=material_name);
                desc_text.push((UI_IMG_ID_ITEM_INFO, t));
            }
        }

        ItemInfoText {
            item_name,
            item_kind,
            desc_text,
        }
    }
}
