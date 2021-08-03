use super::img::build_img;
use crate::error::*;
use crate::input::*;
use anyhow::*;
use common::gamedata::*;

pub fn build_item_object(input: Input) -> Result<ItemObject, Error> {
    let img = get_optional_field!(input, image);
    let item = get_optional_field!(input, item);
    let flags = ItemFlags::empty();

    let kind = match item.item_kind.as_str() {
        "object" => ItemKind::Object,
        "potion" => ItemKind::Potion,
        "throwing" => ItemKind::Throwing,
        "food" => ItemKind::Food,
        "magic_device" => ItemKind::MagicDevice,
        "weapon" => ItemKind::Weapon(get_optional_field!(item, weapon_kind)),
        "armor" => ItemKind::Armor(get_optional_field!(item, armor_kind)),
        "tool" => ItemKind::Tool,
        "container" => ItemKind::Container,
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
        id: input.id,
        img: build_img(img)?.0,
        default_flags: flags,
        kind,
        group: item.group,
        basic_price: item.basic_price,
        w: item.w,
        quality_kind: item.quality_kind,
        gen_weight: item.gen_weight,
        shop_weight: item.shop_weight.unwrap_or(item.gen_weight),
        gen_level: item.gen_level,
        power: item.power.unwrap_or(0),
        power_var: item.power_var.unwrap_or(0),
        def: item.def.unwrap_or(ElementArray([0, 0, 0, 0, 0, 0])),
        attrs: item.attrs,
        material_group: item.material_group,
        material: item.material,
    })
}
