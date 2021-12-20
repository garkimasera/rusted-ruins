use std::collections::hash_map::DefaultHasher;
use std::hash::BuildHasherDefault;

/// This hashmap does not have random state.
/// Used to fix the order of items in cbor maps.
pub type HashMap<K, V> = std::collections::HashMap<K, V, BuildHasherDefault<DefaultHasher>>;
pub type HashSet<T> = std::collections::HashSet<T, BuildHasherDefault<DefaultHasher>>;
