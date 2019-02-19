use super::{FileBox, WithId};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

impl<T: WithId> Serialize for FileBox<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.id.serialize(serializer)
    }
}

impl<'de, T: WithId> Deserialize<'de> for FileBox<T> {
    fn deserialize<D>(deserializer: D) -> Result<FileBox<T>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let id = u64::deserialize(deserializer)?;
        Ok(FileBox::empty(id))
    }
}
