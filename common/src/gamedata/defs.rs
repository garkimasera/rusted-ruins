//! Miscellaneous type definitions

use std::ops::{Index, IndexMut};

/// Elements of damage/attack
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(rename_all="snake_case")]
pub enum Element {
    None = -1,
    Physical = 0,
    Fire = 1,
    Cold = 2,
    Shock = 3,
    Poison = 4,
    Spirit = 5,
}

/// This array has the same size as element types.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct ElementArray<T>(pub [T; Element::Spirit as usize + 1]);

impl<T> Index<Element> for ElementArray<T> {
    type Output = T;
    fn index(&self, e: Element) -> &T {
        assert_ne!(e, Element::None);
        &self.0[e as usize]
    }
}

impl<T> IndexMut<Element> for ElementArray<T> {
    fn index_mut(&mut self, e: Element) -> &mut T {
        assert_ne!(e, Element::None);
        &mut self.0[e as usize]
    }
}

