use super::to_text::CharaTraitTextId;
use super::{misc_txt, ToText};
use common::gamedata::CharaTrait;
use rules::RULES;
use std::fmt::Write;

pub fn trait_description(chara_trait: &CharaTrait) -> String {
    let mut desc = misc_txt(&format!("trait-{}-desc", chara_trait.text_id()));

    let _ = writeln!(desc, "\n");

    if let CharaTrait::Id(id) = chara_trait {
        for modifier in &RULES.chara_traits.get(id).modifiers {
            let _ = writeln!(desc, "{}", &modifier.to_text());
        }
    }

    desc
}
