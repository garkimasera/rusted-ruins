use super::defs::*;
use super::effect::Effect;
use super::item_attr::*;
use super::skill::WeaponKind;
use super::time::{Duration, Time};
use super::KindParseError;
use crate::basic::ArrayStringId;
use crate::objholder::ItemIdx;
use bitflags::bitflags;
use geom::Coords;
use std::cmp::{Ord, Ordering, PartialOrd};
use std::str::FromStr;

/// Game item
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Item {
    pub idx: ItemIdx,
    pub kind: ItemKind,
    pub flags: ItemFlags,
    pub quality: ItemQuality,
    pub attrs: Vec<ItemAttr>,
    pub time: Option<ItemTime>,
}

/// Time management data for item.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct ItemTime {
    pub remaining: Duration,
    pub last_updated: Time,
}

/// ItemObject has detail data for one item
#[derive(Serialize, Deserialize)]
pub struct ItemObject {
    pub id: String,
    pub img: crate::obj::Img,
    pub kind: ItemKind,
    /// The name of item group. It is used for sorting.
    pub group: String,
    /// Defalut item flags
    /// They are set at making object based on object setting files
    pub default_flags: ItemFlags,
    pub basic_price: u32,
    /// Item weight (gram)
    pub w: u32,
    /// Quality kind
    pub quality_kind: QualityKind,
    /// The frequency of item generation in random map
    pub gen_weight: f32,
    /// The frequency of item generation in shops
    pub shop_weight: f32,
    /// Generation level
    /// If it is higher, and the item will be generated on deeper floors.
    /// This parameter will be used for shops also.
    pub gen_level: u32,
    pub attrs: Vec<ItemObjAttr>,
    /// Possible material for this item
    pub material_group: String,
    /// Material name used for creation
    pub material: MaterialName,
}

impl Ord for Item {
    fn cmp(&self, other: &Item) -> Ordering {
        let order = self.cmp_except_time(other);
        if order != Ordering::Equal {
            return order;
        }
        self.time.cmp(&other.time)
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Item) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Item {
    pub fn cmp_except_time(&self, other: &Item) -> Ordering {
        let order = self.flags.cmp(&other.flags);
        if order != Ordering::Equal {
            return order.reverse();
        }

        let order = self.kind.cmp(&other.kind);
        if order != Ordering::Equal {
            return order;
        }
        let order = self.group_order(other);
        if order != Ordering::Equal {
            return order;
        }
        let order = self.idx.cmp(&other.idx);
        if order != Ordering::Equal {
            return order;
        }
        let order = self.quality.cmp(&other.quality);
        if order != Ordering::Equal {
            return order;
        }
        self.attrs.cmp(&other.attrs)
    }

    #[cfg(feature = "global_state_obj")]
    pub fn group_order(&self, other: &Item) -> Ordering {
        self.obj().group.cmp(&other.obj().group)
    }

    #[cfg(not(feature = "global_state_obj"))]
    pub fn group_order(&self, _order: &Item) -> Ordering {
        Ordering::Equal
    }

    #[cfg(feature = "global_state_obj")]
    pub fn obj(&self) -> &'static ItemObject {
        crate::gobj::get_obj(self.idx)
    }
}

/// This is mainly used for item list sorting
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub enum ItemKind {
    /// Drinkable and have some medical effects.
    Potion,
    /// Eatable and have nutrition.
    Food,
    /// Throwing weapons.
    Throwing,
    /// Can release the own magical power.
    MagicDevice,
    /// Equipment to attack enemy.
    Weapon(WeaponKind),
    /// Equipment to protect character from attacks.
    Armor(ArmorKind),
    /// Equipment for specific work.
    Tool,
    /// Contains some items.
    Container,
    /// Other items that have some effects.
    Special,
    /// Readable books or other items.
    Readable,
    /// Usable to create other items.
    Material,
    /// Module items for weapon/armor items.
    Module,
    /// Other items that might not have effects, but have some price.
    Object,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
pub enum ItemKindRough {
    Potion,
    Food,
    Throwing,
    MagicDevice,
    Weapon,
    Armor,
    Tool,
    Container,
    Special,
    Readable,
    Material,
    Module,
    Object,
}

impl ItemKind {
    pub fn rough(&self) -> ItemKindRough {
        match self {
            ItemKind::Potion => ItemKindRough::Potion,
            ItemKind::Food => ItemKindRough::Food,
            ItemKind::Throwing => ItemKindRough::Throwing,
            ItemKind::MagicDevice => ItemKindRough::MagicDevice,
            ItemKind::Weapon(_) => ItemKindRough::Weapon,
            ItemKind::Armor(_) => ItemKindRough::Armor,
            ItemKind::Tool => ItemKindRough::Tool,
            ItemKind::Container => ItemKindRough::Container,
            ItemKind::Special => ItemKindRough::Special,
            ItemKind::Readable => ItemKindRough::Readable,
            ItemKind::Material => ItemKindRough::Material,
            ItemKind::Module => ItemKindRough::Module,
            ItemKind::Object => ItemKindRough::Object,
        }
    }
}

impl FromStr for ItemKind {
    type Err = KindParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(weapon) = WeaponKind::from_str(s) {
            return Ok(ItemKind::Weapon(weapon));
        }
        if let Ok(armor) = ArmorKind::from_str(s) {
            return Ok(ItemKind::Armor(armor));
        }

        Ok(match s {
            "potion" => ItemKind::Potion,
            "food" => ItemKind::Food,
            "throwing" => ItemKind::Throwing,
            "magic_device" => ItemKind::MagicDevice,
            "tool" => ItemKind::Tool,
            "container" => ItemKind::Container,
            "special" => ItemKind::Special,
            "readable" => ItemKind::Readable,
            "material" => ItemKind::Material,
            "module" => ItemKind::Module,
            "object" => ItemKind::Object,
            _ => {
                return Err(KindParseError(s.into()));
            }
        })
    }
}

impl std::fmt::Display for ItemKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ItemKind::Potion => "potion",
            ItemKind::Food => "food",
            ItemKind::Throwing => "throwing",
            ItemKind::MagicDevice => "magic_device",
            ItemKind::Tool => "tool",
            ItemKind::Container => "container",
            ItemKind::Special => "special",
            ItemKind::Readable => "readable",
            ItemKind::Material => "material",
            ItemKind::Module => "module",
            ItemKind::Object => "object",
            ItemKind::Weapon(weapon_kind) => {
                return write!(f, "{}", weapon_kind);
            }
            ItemKind::Armor(armor_kind) => {
                return write!(f, "{}", armor_kind);
            }
        };
        write!(f, "{}", s)
    }
}

bitflags! {
    #[derive(Serialize, Deserialize)]
    #[serde(transparent)]
    pub struct ItemFlags: u64 {
        /// This item is fixed and never moved or destroyed.
        const FIXED = 1 << 0;
        /// This item is owned by the other faction. Need to steal to get it.
        const OWNED = 1 << 1;
        /// This item cannot be moved, but can be harvested.
        const PLANT = 1 << 2;
    }
}

/// Item quality is used to calculate the effects.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct ItemQuality {
    /// Base quality of the item
    pub base: i32,
    /// Additional quality by magic, smithing, etc.
    pub enchant: u16,
    /// If the item is damaged, this value will decrease
    pub damage: u16,
}

impl Default for ItemQuality {
    fn default() -> ItemQuality {
        ItemQuality {
            base: 0,
            enchant: 0,
            damage: 0,
        }
    }
}

impl ItemQuality {
    /// Return the summation of quality values
    pub fn as_int(&self) -> i32 {
        self.base + self.enchant as i32 + self.damage as i32
    }
}

/// Quality kind for item.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub enum QualityKind {
    /// Item's quality is always zero including enchant and damage.
    None,
    /// Mutable quality and additional base quality will be added when generated.
    Mutable,
}

impl Default for QualityKind {
    fn default() -> Self {
        QualityKind::None
    }
}

pub type MaterialName = ArrayStringId;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
pub enum ArmorKind {
    Shield,
    Head,
    Skin,
    Body,
    Arms,
    Legs,
    ArmorPlate,
    Accessory,
}

impl std::fmt::Display for ArmorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ArmorKind::Shield => "shield",
            ArmorKind::Head => "head",
            ArmorKind::Skin => "skin",
            ArmorKind::Body => "body",
            ArmorKind::Arms => "arms",
            ArmorKind::Legs => "legs",
            ArmorKind::ArmorPlate => "armor_plate",
            ArmorKind::Accessory => "accessory",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for ArmorKind {
    type Err = KindParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "shield" => ArmorKind::Shield,
            "head" => ArmorKind::Head,
            "skin" => ArmorKind::Skin,
            "body" => ArmorKind::Body,
            "arms" => ArmorKind::Arms,
            "legs" => ArmorKind::Legs,
            "armor_plate" => ArmorKind::ArmorPlate,
            "accessory" => ArmorKind::Accessory,
            _ => {
                return Err(KindParseError(s.into()));
            }
        })
    }
}

/// Data to generate an item.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct ItemGen {
    pub id: String,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum ItemListLocation {
    OnMap {
        mid: super::map::MapId,
        pos: Coords,
    },
    Chara {
        cid: super::chara::CharaId,
    },
    Equip {
        cid: super::chara::CharaId,
    },
    Shop {
        cid: super::CharaId,
    },
    Container {
        ill: ItemListLocationExceptContainer,
        i: u32,
    },
}

impl ItemListLocation {
    pub fn in_container(il: ItemLocation) -> Self {
        let ill = match il.0 {
            ItemListLocation::OnMap { mid, pos } => {
                ItemListLocationExceptContainer::OnMap { mid, pos }
            }
            ItemListLocation::Chara { cid } => ItemListLocationExceptContainer::Chara { cid },
            ItemListLocation::Equip { cid } => ItemListLocationExceptContainer::Equip { cid },
            ItemListLocation::Shop { cid } => ItemListLocationExceptContainer::Shop { cid },
            ItemListLocation::Container { .. } => {
                panic!("tried to get item list in an item in container")
            }
        };
        ItemListLocation::Container { ill, i: il.1 }
    }

    pub fn container_item_location(self) -> Option<ItemLocation> {
        match self {
            ItemListLocation::Container { ill, i } => Some((ill.into(), i)),
            _ => None,
        }
    }

    pub const PLAYER: ItemListLocation = ItemListLocation::Chara {
        cid: super::chara::CharaId::Player,
    };
}

impl From<ItemListLocationExceptContainer> for ItemListLocation {
    fn from(ill: ItemListLocationExceptContainer) -> Self {
        match ill {
            ItemListLocationExceptContainer::OnMap { mid, pos } => {
                ItemListLocation::OnMap { mid, pos }
            }
            ItemListLocationExceptContainer::Chara { cid } => ItemListLocation::Chara { cid },
            ItemListLocationExceptContainer::Equip { cid } => ItemListLocation::Equip { cid },
            ItemListLocationExceptContainer::Shop { cid } => ItemListLocation::Shop { cid },
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum ItemListLocationExceptContainer {
    OnMap { mid: super::map::MapId, pos: Coords },
    Chara { cid: super::chara::CharaId },
    Equip { cid: super::chara::CharaId },
    Shop { cid: super::CharaId },
}

pub type ItemLocation = (ItemListLocation, u32);

/// Item list that records all items owned by one character or one tile
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItemList {
    pub items: Vec<(Item, u32)>,
}

impl Default for ItemList {
    fn default() -> Self {
        ItemList { items: Vec::new() }
    }
}

impl ItemList {
    /// Get the number of item
    pub fn get_number(&self, i: u32) -> u32 {
        self.items[i as usize].1
    }

    /// This list is empty or not
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Append item simply
    pub fn append_simple(&mut self, item: Item, n: u32) {
        if self.items.is_empty() {
            self.items.push((item, n));
            return;
        }

        for i in 0..self.items.len() {
            match item.cmp(&self.items[i].0) {
                Ordering::Equal => {
                    // If this list has the same item, increases the number
                    self.items[i].1 += n;
                    return;
                }
                Ordering::Less => {
                    self.items.insert(i, (item, n));
                    return;
                }
                Ordering::Greater => {
                    continue;
                }
            }
        }
        self.items.push((item, n));
    }

    /// Remove an item from list
    pub fn remove<T: Into<ItemMoveNum>>(&mut self, i: u32, n: T) {
        let i = i as usize;
        let n = n.into().to_u32(self.items[i].1);
        assert!(self.items[i].1 >= n && n != 0);
        if n == 0 {
            return;
        }

        self.items[i].1 -= n;
        if self.items[i].1 == 0 {
            self.items.remove(i);
        }
    }

    /// Remove an item from list and get its clone or moved value
    pub fn remove_and_get<T: Into<ItemMoveNum>>(&mut self, i: u32, n: T) -> Item {
        let i = i as usize;
        let n = n.into().to_u32(self.items[i].1);
        assert!(self.items[i].1 >= n && n != 0);

        self.items[i].1 -= n;
        if self.items[i].1 == 0 {
            self.items.remove(i).0
        } else {
            self.items[i].0.clone()
        }
    }

    /// Clear all item in list
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Return item iterator
    pub fn iter(&self) -> std::slice::Iter<'_, (Item, u32)> {
        self.items.iter()
    }

    /// Return list size
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Count specified item
    pub fn count(&self, idx: ItemIdx) -> u32 {
        self.iter()
            .filter_map(|(item, n)| if item.idx == idx { Some(n) } else { None })
            .sum::<u32>()
    }

    /// Retains item by given number
    pub fn retain<F: FnMut(&Item, u32) -> u32>(&mut self, mut f: F, reverse_order: bool) {
        let iter = std::mem::take(&mut self.items).into_iter();
        let mut items = Vec::new();
        if !reverse_order {
            for (item, n) in iter {
                let n = f(&item, n);
                if n > 0 {
                    items.push((item, n));
                }
            }
        } else {
            for (item, n) in iter.rev() {
                let n = f(&item, n);
                if n > 0 {
                    items.push((item, n));
                }
            }
            items.reverse();
        }
        self.items = items;
    }

    /// Consume specified item
    pub fn consume<F: FnMut(&Item, u32)>(
        &mut self,
        idx: ItemIdx,
        consume: u32,
        mut f: F,
        prior_high_quality: bool,
    ) {
        let mut need_consumed = consume;
        self.retain(
            |item, n| {
                if item.idx != idx {
                    return n;
                }
                let consumed = if need_consumed == 0 {
                    0
                } else if need_consumed < n {
                    let consumed = need_consumed;
                    need_consumed = 0;
                    consumed
                } else {
                    need_consumed -= n;
                    n
                };

                f(item, consumed);
                n - consumed
            },
            prior_high_quality,
        );
        assert_eq!(need_consumed, 0);
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ItemMoveNum {
    All,
    Partial(u32),
}

impl ItemMoveNum {
    pub fn to_u32(self, all: u32) -> u32 {
        match self {
            ItemMoveNum::All => all,
            ItemMoveNum::Partial(n) => n,
        }
    }
}

impl From<u32> for ItemMoveNum {
    fn from(n: u32) -> ItemMoveNum {
        ItemMoveNum::Partial(n)
    }
}

//
// Equipment handling types and routines
//

/// equipment slots
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
pub enum EquipSlotKind {
    MeleeWeapon,
    RangedWeapon,
    Tool,
    Shield,
    Head,
    Skin,
    Body,
    Arms,
    Legs,
    ArmorPlate,
    Accessory,
}

impl ItemKind {
    pub fn equip_slot_kind(self) -> Option<EquipSlotKind> {
        match self {
            ItemKind::Weapon(weapon_kind) => Some(weapon_kind.equip_slot_kind()),
            ItemKind::Armor(armor_kind) => Some(armor_kind.equip_slot_kind()),
            ItemKind::Tool => Some(EquipSlotKind::Tool),
            _ => None,
        }
    }
}

impl WeaponKind {
    pub fn equip_slot_kind(self) -> EquipSlotKind {
        if self < WeaponKind::_DummyWeapon {
            EquipSlotKind::MeleeWeapon
        } else {
            EquipSlotKind::RangedWeapon
        }
    }
}

impl ArmorKind {
    pub fn equip_slot_kind(self) -> EquipSlotKind {
        match self {
            ArmorKind::Shield => EquipSlotKind::Shield,
            ArmorKind::Head => EquipSlotKind::Head,
            ArmorKind::Skin => EquipSlotKind::Skin,
            ArmorKind::Body => EquipSlotKind::Body,
            ArmorKind::Arms => EquipSlotKind::Arms,
            ArmorKind::Legs => EquipSlotKind::Legs,
            ArmorKind::ArmorPlate => EquipSlotKind::ArmorPlate,
            ArmorKind::Accessory => EquipSlotKind::Accessory,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EquipItemList {
    /// Slot information
    slots: Vec<SlotInfo>,
    item_list: ItemList,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct SlotInfo {
    /// The kind of equipment
    esk: EquipSlotKind,
    /// The index in this ItemKind
    n: u8,
    /// The Index at list
    list_idx: Option<u8>,
}

impl SlotInfo {
    fn new(esk: EquipSlotKind, n: u8) -> SlotInfo {
        SlotInfo {
            esk,
            n,
            list_idx: None,
        }
    }
}

pub const MAX_SLOT_NUM_PER_KIND: usize = crate::basic::MAX_EQUIP_SLOT as usize;

impl EquipItemList {
    pub fn new(slots: &[EquipSlotKind]) -> EquipItemList {
        let mut slots = slots.to_vec();
        slots.sort();
        let mut new_slots = Vec::new();
        let mut last_esk = None;
        let mut i = 0;
        for esk in slots.iter() {
            if last_esk == Some(esk) {
                i += 1;
            }
            new_slots.push(SlotInfo::new(*esk, i));
            last_esk = Some(esk);
        }

        EquipItemList {
            slots: new_slots,
            item_list: ItemList::default(),
        }
    }

    /// Number of slots for specified ItemKind
    pub fn slot_num(&self, esk: EquipSlotKind) -> usize {
        self.slots.iter().filter(|slot| slot.esk == esk).count()
    }

    /// Specified slot is empty or not
    /// If specified slot doesn't exist, return false.
    pub fn is_slot_empty(&self, esk: EquipSlotKind, n: usize) -> bool {
        assert!(n < MAX_SLOT_NUM_PER_KIND);
        if let Some(a) = self.slots.iter().filter(|slot| slot.esk == esk).nth(n) {
            a.list_idx.is_none()
        } else {
            false
        }
    }

    /// Get specified equipped item
    pub fn item(&self, esk: EquipSlotKind, n: usize) -> Option<&Item> {
        assert!(n < MAX_SLOT_NUM_PER_KIND);
        self.list_idx(esk, n).map(|a| &self.item_list.items[a].0)
    }

    /// Equip an item to specified slot (the nth slot of given EquipSlotKind), and returns removed item
    pub fn equip(&mut self, esk: EquipSlotKind, n: usize, item: Item) -> Option<Item> {
        assert!(self.slot_num(esk) > n);
        if let Some(i) = self.list_idx(esk, n) {
            // Replace existing item
            return Some(std::mem::replace(&mut self.item_list.items[i].0, item));
        }

        if self.item_list.items.is_empty() {
            // If any item is not equipped.
            self.item_list.items.push((item, 1));
            self.set_list_idx(esk, n, 0);
            return None;
        }

        // Calculate new index for insert
        let mut new_idx = 0;
        let mut processed_slot = 0;
        for i_slot in 0..self.slots.len() {
            if self.slots[i_slot].esk == esk && self.slots[i_slot].n as usize == n {
                self.set_list_idx(esk, n, new_idx);
                self.item_list.items.insert(new_idx, (item, 1));
                processed_slot = i_slot;
                break;
            } else if self.slots[i_slot].list_idx.is_some() {
                new_idx += 1;
            }
        }

        for i_slot in (processed_slot + 1)..self.slots.len() {
            if let Some(list_idx) = self.slots[i_slot].list_idx {
                self.slots[i_slot].list_idx = Some(list_idx + 1);
            }
        }

        None
    }

    pub fn remove(&mut self, esk: EquipSlotKind, n: usize) -> Option<Item> {
        assert!(self.slot_num(esk) > n);

        let i_remove = self.list_idx(esk, n)?;
        let item = self.item_list.items.remove(i_remove);

        for slot_info in &mut self.slots {
            if slot_info.esk == esk && slot_info.n as usize == n {
                assert_eq!(slot_info.list_idx, Some(i_remove as u8));
                slot_info.list_idx = None;
            } else if let Some(list_idx) = slot_info.list_idx.as_mut() {
                if *list_idx > i_remove as u8 {
                    *list_idx -= 1;
                }
            }
        }
        Some(item.0)
    }

    pub(crate) fn list_idx(&self, esk: EquipSlotKind, n: usize) -> Option<usize> {
        if let Some(slot) = self
            .slots
            .iter()
            .find(|slot| slot.esk == esk && slot.n as usize == n)
        {
            slot.list_idx.map(|list_idx| list_idx as usize)
        } else {
            None
        }
    }

    fn set_list_idx(&mut self, esk: EquipSlotKind, n: usize, idx: usize) {
        if let Some(slot) = self
            .slots
            .iter_mut()
            .find(|slot| slot.esk == esk && slot.n as usize == n)
        {
            slot.list_idx = Some(idx as u8);
        } else {
            panic!("set_list_idx for invalid slot");
        }
    }

    pub fn slot_iter(&self) -> EquipSlotIter<'_> {
        EquipSlotIter {
            equip_item_list: self,
            n: 0,
        }
    }

    pub fn item_iter(&self) -> EquipItemIter<'_> {
        EquipItemIter {
            equip_item_list: self,
            n: 0,
        }
    }

    pub fn list(&self) -> &ItemList {
        &self.item_list
    }

    pub(crate) fn list_mut(&mut self) -> &mut ItemList {
        &mut self.item_list
    }

    pub fn n_slots(&self) -> u32 {
        self.slots.len() as u32
    }
}

pub struct EquipSlotIter<'a> {
    equip_item_list: &'a EquipItemList,
    n: usize,
}

impl<'a> Iterator for EquipSlotIter<'a> {
    type Item = (EquipSlotKind, u8, Option<&'a Item>);
    fn next(&mut self) -> Option<Self::Item> {
        if self.n >= self.equip_item_list.slots.len() {
            return None;
        }
        let slot = &self.equip_item_list.slots[self.n];
        let result = if let Some(i) = slot.list_idx {
            (
                slot.esk,
                slot.n,
                Some(&self.equip_item_list.item_list.items[i as usize].0),
            )
        } else {
            (slot.esk, slot.n, None)
        };
        self.n += 1;
        Some(result)
    }
}

pub struct EquipItemIter<'a> {
    equip_item_list: &'a EquipItemList,
    n: usize,
}

impl<'a> Iterator for EquipItemIter<'a> {
    type Item = (EquipSlotKind, u8, &'a Item);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.n >= self.equip_item_list.slots.len() {
                return None;
            }
            let slot = &self.equip_item_list.slots[self.n];
            if let Some(i) = slot.list_idx {
                let result = (
                    slot.esk,
                    slot.n,
                    &self.equip_item_list.item_list.items[i as usize].0,
                );
                self.n += 1;
                return Some(result);
            }
            self.n += 1;
        }
    }
}
