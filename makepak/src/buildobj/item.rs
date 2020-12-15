use super::effect::convert_effect_input;
use super::img::build_img;
use crate::error::*;
use crate::input::*;
use anyhow::*;
use common::gamedata::defs::ElementArray;
use common::gamedata::item::*;

pub fn build_item_object(tomlinput: Input) -> Result<ItemObject, Error> {
    let img = get_optional_field!(tomlinput, image);
    let item = get_optional_field!(tomlinput, item);
    let flags = ItemFlags::empty();

    let kind = match item.item_kind.as_str() {
        "object" => ItemKind::Object,
        "potion" => ItemKind::Potion,
        "food" => ItemKind::Food,
        "magic_device" => ItemKind::MagicDevice,
        "weapon" => ItemKind::Weapon(get_optional_field!(item, weapon_kind)),
        "armor" => ItemKind::Armor(get_optional_field!(item, armor_kind)),
        "tool" => ItemKind::Tool,
        "readable" => ItemKind::Readable,
        "material" => ItemKind::Material,
        "special" => ItemKind::Special,
        _ => {
            bail!(PakCompileError::UnexpectedValue {
                field_name: "item_kind".to_owned(),
                value: item.item_kind.clone()
            });
        }
    };

    Ok(ItemObject {
        id: tomlinput.id,
        img: build_img(img)?.0,
        default_flags: flags,
        kind,
        group: item.group,
        basic_price: item.basic_price,
        w: item.w,
        gen_weight: item.gen_weight,
        shop_weight: item.shop_weight.unwrap_or(item.gen_weight),
        gen_level: item.gen_level,
        dice_n: item.dice_n.unwrap_or(0),
        dice_x: item.dice_x.unwrap_or(0),
        def: item.def.unwrap_or(ElementArray([0, 0, 0, 0, 0, 0])),
        eff: item.eff.unwrap_or(0),
        magical_effect: convert_effect_input(item.magical_effect)?,
        medical_effect: convert_effect_input(item.medical_effect)?,
        use_effect: convert_effect_input(item.use_effect)?,
        tool_effect: item.tool_effect,
        nutrition: item.nutrition.unwrap_or(0),
        charge: item.charge,
        harvest: item.harvest,
        facility: item.facility,
        material_group: item.material_group,
        material: item.material,
        titles: item.titles,
    })
}
