
use error::*;
use tomlinput::*;
use common::obj::*;
use common::gamedata::item::*;
use super::img::build_img;

pub fn build_item_object(tomlinput: TomlInput) -> Result<ItemObject> {
    let img = get_optional_field!(tomlinput, image);
    let item = get_optional_field!(tomlinput, item);
    let mut flags = ItemFlags::empty();

    let kind = match item.item_kind.as_str() {
        "Object" => ItemKind::Object,
        "Potion" => {
            flags |= ItemFlags::DRINKABLE;
            ItemKind::Potion
        }
        "Weapon" => {
            ItemKind::Weapon(get_optional_field!(item, weapon_kind))
        }
        "Armor" => {
            ItemKind::Armor(get_optional_field!(item, armor_kind))
        }
        _ => {
            bail!(ErrorKind::UnexpectedValue("item_kind".to_owned(), item.item_kind.clone()));
        },
    };

    Ok(ItemObject {
        id: tomlinput.id,
        img: build_img(img)?.0,
        default_flags: flags,
        kind: kind,
        basic_price: item.basic_price,
        gen_weight: item.gen_weight,
        gen_level: item.gen_level,
        dice_n: item.dice_n.unwrap_or(0),
        dice_x: item.dice_x.unwrap_or(0),
        def: item.def.unwrap_or(0),
        mdf: item.mdf.unwrap_or(0),
        eff: item.eff.unwrap_or(0),
        medical_effect: item.medical_effect,
    })
}

