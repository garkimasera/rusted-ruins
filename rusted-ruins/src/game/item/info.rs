use crate::game::extrait::*;
use crate::text::{misc_txt, ToText};
use common::gamedata::*;
use common::objholder::ItemIdx;
use ordered_float::NotNan;

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
                let (power, hit) = if let Some(ItemObjAttr::Weapon { base_power, hit }) =
                    find_attr!(obj, ItemObjAttr::Weapon)
                {
                    (base_power.calc_without_var(item.power_factor()), *hit)
                } else {
                    (0.0, NotNan::default())
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
                let t = misc_txt_format!(
                    "item_info_text-hit"; hit=hit);
                desc_text.push((UI_IMG_ID_ITEM_INFO, t));
            }
            ItemKind::Armor(_) => {
                if item.defence(Element::Physical) != 0 {
                    desc_text.push((
                        "!icon-defence-physical",
                        format!("{}", item.defence(Element::Physical)),
                    ));
                }
                if item.defence(Element::Heat) != 0 {
                    desc_text.push((
                        "!icon-defence-heat",
                        format!("{}", item.defence(Element::Heat)),
                    ));
                }
                if item.defence(Element::Cold) != 0 {
                    desc_text.push((
                        "!icon-defence-cold",
                        format!("{}", item.defence(Element::Cold)),
                    ));
                }
                if item.defence(Element::Shock) != 0 {
                    desc_text.push((
                        "!icon-defence-shock",
                        format!("{}", item.defence(Element::Shock)),
                    ));
                }
                if item.defence(Element::Poison) != 0 {
                    desc_text.push((
                        "!icon-defence-poison",
                        format!("{}", item.defence(Element::Poison)),
                    ));
                }
                if item.defence(Element::Spirit) != 0 {
                    desc_text.push((
                        "!icon-defence-spirit",
                        format!("{}", item.defence(Element::Spirit)),
                    ));
                }
            }
            ItemKind::Tool => {}
            ItemKind::Container => {}
            ItemKind::Special => {}
            ItemKind::Readable => {}
            ItemKind::Material => {}
            ItemKind::Module => {}
            ItemKind::Object => {}
        }

        for obj_attr in &obj.attrs {
            match obj_attr {
                ItemObjAttr::CharaModifier(m) => {
                    desc_text.push((UI_IMG_ID_ITEM_INFO, m.to_text().into()));
                }
                ItemObjAttr::Facility { ty, quality } => {
                    let ty = misc_txt(&format!("facility-{}", ty));
                    let t = misc_txt_format!(
                        "item_info_text-facility"; ty=ty, quality=quality);
                    desc_text.push((UI_IMG_ID_ITEM_INFO, t));
                }
                _ => (),
            }
        }

        for attr in &item.attrs {
            match attr {
                ItemAttr::Material(material) => {
                    let material_name = crate::text::prefix::material(*material);
                    let t = misc_txt_format!(
                        "item_info_text-material"; material=material_name);
                    desc_text.push((UI_IMG_ID_ITEM_INFO, t));
                }
                ItemAttr::ModuleSlot { kind, content } => {
                    desc_text.push((slot_icon(*kind), slot_text(content)));
                }
                ItemAttr::Module(effect) => {
                    desc_text.push((slot_icon(effect.kind()), effect.to_text().into()));
                }
                _ => (),
            }
        }

        ItemInfoText {
            item_name,
            item_kind,
            desc_text,
        }
    }
}

pub fn slot_icon(kind: ModuleSlotKind) -> &'static str {
    match kind {
        ModuleSlotKind::Ability => "!icon-module_slot-ability",
        ModuleSlotKind::Core => "!icon-module_slot-core",
        ModuleSlotKind::Extend => "!icon-module_slot-extend",
    }
}

pub fn slot_text(content: &Option<(ItemIdx, ModuleEffect)>) -> String {
    if let Some((_, effect)) = content {
        effect.to_text().into()
    } else {
        misc_txt("item_info_text-empty_slot")
    }
}
