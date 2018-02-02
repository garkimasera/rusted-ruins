
use error::*;
use tomlinput::*;
use common::obj::*;
use common::gamedata::item::*;
use super::img::{build_img, build_icon};

pub fn build_item_object(tomlinput: TomlInput) -> Result<ItemObject> {
    let img = get_optional_field!(tomlinput, image);
    let icon = get_optional_field!(tomlinput, icon);
    let item = get_optional_field!(tomlinput, item);

    let content = match item.item_kind.as_str() {
        "Object" => {
            ItemContent::Object
        },
        "Potion" => {
            ItemContent::Potion {
                kind: get_optional_field!(item, potion_kind),
                eff: get_optional_field!(item, eff)
            }
        }
        "Weapon" => {
            ItemContent::Weapon {
                kind: get_optional_field!(item, weapon_kind),
                eff: get_optional_field!(item, eff),
            }
        }
        "BodyArmor" => {
            ItemContent::BodyArmor {
                def: get_optional_field!(item, def),
                mdf: get_optional_field!(item, mdf),
            }
        }
        _ => {
            bail!(ErrorKind::UnexpectedValue("item_kind".to_owned(), item.item_kind.clone()));
        },
    };
    
    Ok(ItemObject {
        id: tomlinput.id,
        img: build_img(img)?.0,
        icon: build_icon(icon)?,
        gen_weight: item.gen_weight,
        gen_level: item.gen_level,
        basic_price: item.basic_price,
        content: content, 
    })
}

