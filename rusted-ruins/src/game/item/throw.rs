use crate::game::extrait::*;
use common::gamedata::*;

pub fn item_to_throw_effect(gd: &GameData, il: ItemLocation, cid: CharaId) -> Effect {
    let item = gd.get_item(il).0;
    let item_obj = item.obj();
    let range = item.throw_range(gd.chara.get(cid).attr.str);
    let mut effect = if let Some(effect) = item_obj
        .attrs
        .iter()
        .filter_map(|attr| match attr {
            ItemObjAttr::Throw { effect, .. } => Some(effect),
            _ => None,
        })
        .next()
    {
        effect.clone()
    } else {
        Effect {
            kind: vec![EffectKind::Ranged {
                element: Element::Physical,
            }],
            anim_kind: EffectAnimKind::Tile,
            anim_img: "!damage-blunt".into(),
            ..Effect::default()
        }
    };
    effect.range = range;
    effect
}
