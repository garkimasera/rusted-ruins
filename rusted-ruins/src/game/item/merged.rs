use common::gamedata::*;

#[derive(Clone)]
/// ItemList with the other list.
pub struct MergedItemList<'a> {
    pub first: (ItemListLocation, &'a ItemList),
    pub second: Option<(ItemListLocation, &'a ItemList)>,
    dummy_list: Vec<(Item, u32)>,
}

pub trait MergedItemListGet {
    fn get_merged_item_list(
        &self,
        first: ItemListLocation,
        second: Option<ItemListLocation>,
    ) -> MergedItemList;
}

impl MergedItemListGet for GameData {
    fn get_merged_item_list(
        &self,
        first: ItemListLocation,
        second: Option<ItemListLocation>,
    ) -> MergedItemList {
        assert_ne!(Some(first), second);

        MergedItemList {
            first: (first, self.get_item_list(first)),
            second: second.map(|il| (il, self.get_item_list(il))),
            dummy_list: vec![],
        }
    }
}

impl<'a> MergedItemList<'a> {
    // pub fn iter(&self) -> impl Iterator<Item = &(Item, u32)> {
    //     let first = self.first.1.iter();
    //     if let Some(second) = self.second.as_ref() {
    //         first.chain(second.1.iter())
    //     } else {
    //         first.chain(self.dummy_list.iter())
    //     }
    // }

    pub fn item_location(&self, i: usize) -> ItemLocation {
        if let Some(second) = self.second.as_ref() {
            let first_len = self.first.1.len();
            if i < first_len {
                (self.first.0, i as u32)
            } else {
                (second.0, (i - first_len) as u32)
            }
        } else {
            (self.first.0, i as u32)
        }
    }

    pub fn len(&self) -> usize {
        self.first.1.len() + self.second.map_or(0, |second| second.1.len())
    }

    pub fn get(&self, i: usize) -> &'a (Item, u32) {
        if let Some(second) = self.second.as_ref() {
            let first_len = self.first.1.len();
            if i < first_len {
                &self.first.1.items[i]
            } else {
                &second.1.items[i - first_len]
            }
        } else {
            &self.first.1.items[i]
        }
    }
}
