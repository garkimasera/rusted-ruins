use crate::gamedata::{EquipSlotKind, ItemKind, ItemObject};
use crate::objholder::ItemIdx;
use std::str::FromStr;
use thiserror::Error;

/// Select items by given ids and groups.
#[derive(Clone, Default, Debug)]
pub struct ItemSelector {
    all: bool,
    esk: Option<EquipSlotKind>,
    level: u32,
    ids: Vec<String>,
    groups: Vec<String>,
}

impl ItemSelector {
    pub fn equip_slot_kind(mut self, esk: EquipSlotKind) -> Self {
        self.esk = Some(esk);
        self
    }

    pub fn level(mut self, level: u32) -> Self {
        self.level = level;
        self
    }

    pub fn is(&self, obj: &ItemObject) -> bool {
        let id = &obj.id;
        let group = &obj.group;

        if let Some(esk) = self.esk {
            if Some(esk) != obj.kind.equip_slot_kind() {
                return false;
            }
        }

        if self.all {
            return true;
        }

        self.ids.iter().find(|s| *s == id).is_some()
            || self.groups.iter().find(|s| *s == group).is_some()
    }

    pub fn select_items_from<'a>(&self, list: &'a [ItemObject]) -> Vec<(ItemIdx, &'a ItemObject)> {
        let mut items = Vec::new();

        for (i, item) in list.iter().enumerate() {
            if self.is(item) {
                items.push((ItemIdx::from_usize(i), item));
            }
        }

        items
    }
}

impl FromStr for ItemSelector {
    type Err = ItemSelectorFromStrErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut item_selector = ItemSelector::default();

        if s == "*" {
            item_selector.all = false;
            return Ok(item_selector);
        }

        for a in s.split(',') {
            if let Some(group) = a.strip_prefix("group/") {
                item_selector.groups.push(group.to_owned());
            } else if a.len() > 1 {
                item_selector.ids.push(a.into());
            } else {
                return Err(ItemSelectorFromStrErr);
            }
        }

        if item_selector.ids.len() == 0 && item_selector.groups.len() == 0 {
            return Err(ItemSelectorFromStrErr);
        }

        Ok(item_selector)
    }
}

impl std::fmt::Display for ItemSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.all {
            return write!(f, "*");
        }

        for id in &self.ids {
            write!(f, "{}", id)?;
        }

        for group in &self.groups {
            write!(f, "{}", group)?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Error)]
#[error("invalid input for item selector")]
pub struct ItemSelectorFromStrErr;

#[test]
fn item_selector_test() {
    let s = "group/food,hoge";

    let item_selector: ItemSelector = s.parse().unwrap();

    eprintln!("{:?}", item_selector);
}
