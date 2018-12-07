
use std::hash::Hash;
use super::{HashNamedFileBox, WithId};
use serde::{Serialize, Deserialize, Serializer, Deserializer};

impl<T: WithId> Serialize for HashNamedFileBox<T> where T::ID: Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        self.id.serialize(serializer)
    }
}

impl<'de, T: WithId> Deserialize<'de> for HashNamedFileBox<T> where T::ID: for<'a> Deserialize<'a> + Hash {
    fn deserialize<D>(deserializer: D) -> Result<HashNamedFileBox<T>, D::Error>
    where D: Deserializer<'de> {
        
        let id = T::ID::deserialize(deserializer)?;
        Ok(HashNamedFileBox::empty(id))
    }
}

