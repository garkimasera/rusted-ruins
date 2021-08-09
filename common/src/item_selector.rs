use crate::gamedata::{EquipSlotKind, ItemKind, ItemObject, KindParseError, WeaponKind};
use crate::objholder::ItemIdx;
use serde::de::{self, Deserialize, Visitor};
use serde::Serialize;
use std::str::FromStr;
use thiserror::Error;

/// Select items by given ids and groups.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Default, Debug)]
pub struct ItemSelector {
    all: bool,
    esk: Option<EquipSlotKind>,
    level: u32,
    ids: Vec<String>,
    kinds: Vec<ItemKind>,
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
        let kind = obj.kind;

        if let Some(esk) = self.esk {
            if Some(esk) != obj.kind.equip_slot_kind() {
                return false;
            }
        }

        if self.all {
            return true;
        }

        self.ids.iter().any(|s| s == id)
            || self.kinds.iter().any(|s| *s == kind)
            || self.groups.iter().any(|s| s == group)
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
            item_selector.all = true;
            return Ok(item_selector);
        }

        for a in s.split(',') {
            if let Some(group) = a.strip_prefix("group/") {
                item_selector.groups.push(group.to_owned());
            } else if let Some(kind) = a.strip_prefix("kind/") {
                if let Ok(kind) = kind.parse() {
                    item_selector.kinds.push(kind);
                } else {
                    return Err(ItemSelectorFromStrErr(s.into()));
                }
            } else if a.len() > 1 {
                item_selector.ids.push(a.into());
            } else {
                return Err(ItemSelectorFromStrErr(s.into()));
            }
        }

        if item_selector.ids.is_empty()
            && item_selector.kinds.is_empty()
            && item_selector.groups.is_empty()
        {
            return Err(ItemSelectorFromStrErr(s.into()));
        }

        Ok(item_selector)
    }
}

impl std::fmt::Display for ItemSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.all {
            return write!(f, "*");
        }

        let mut need_comma = false;

        for kind in &self.kinds {
            if need_comma {
                write!(f, ",")?;
            } else {
                need_comma = true;
            }
            write!(f, "kind/{}", kind)?;
        }

        for group in &self.groups {
            if need_comma {
                write!(f, ",")?;
            } else {
                need_comma = true;
            }
            write!(f, "group/{}", group)?;
        }

        for id in &self.ids {
            if need_comma {
                write!(f, ",")?;
            } else {
                need_comma = true;
            }
            write!(f, "{}", id)?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Error)]
#[error("invalid input for item selector \"{0}\"")]
pub struct ItemSelectorFromStrErr(String);

impl Serialize for ItemSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

struct ItemSelectorVisitor;

impl<'de> Visitor<'de> for ItemSelectorVisitor {
    type Value = ItemSelector;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("item selector string")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        s.parse().map_err(de::Error::custom)
    }
}

impl<'de> Deserialize<'de> for ItemSelector {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_any(ItemSelectorVisitor)
    }
}

#[test]
fn item_selector_test() {
    let s = "kind/food,group/food,hoge";

    let item_selector: ItemSelector = s.parse().unwrap();

    eprintln!("{:?}", item_selector);
}
