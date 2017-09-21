
use error::*;
use tomlinput::*;
use common::obj::*;
use common::item::*;
use super::img::{build_img, build_icon};
//use super::

pub fn build_item_object(tomlinput: TomlInput) -> Result<ItemObject> {
    let img = get_optional_field!(tomlinput, image);
    let icon = get_optional_field!(tomlinput, icon);
    let item = get_optional_field!(tomlinput, item);

    let content = match item.item_kind.as_str() {
        "Thing" => {
            ItemContent::Thing
        },
        "Potion" => {
            ItemContent::Potion {
                kind: PotionKind::Heal,
                eff: get_optional_field!(item, eff)
            }
        }
        _ => {
            bail!(ErrorKind::UnexpectedValue("item_kind".to_owned(), item.item_kind.clone()));
        },
    };
    
    Ok(ItemObject {
        id: tomlinput.id,
        img: build_img(img)?,
        icon: build_icon(icon)?,
        basic_price: item.basic_price,
        content: content, 
    })
}

