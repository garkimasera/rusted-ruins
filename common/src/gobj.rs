//! This module provides global state objholder

use crate::objholder::*;
use once_cell::sync::Lazy;
use std::path::PathBuf;
use std::sync::Mutex;

/// Initialize lazy static
pub fn init(pak_dirs: Vec<PathBuf>) {
    *PAK_DIRS.lock().unwrap() = Some(pak_dirs);
    Lazy::force(&OBJ_HOLDER);
}

static PAK_DIRS: Lazy<Mutex<Option<Vec<PathBuf>>>> = Lazy::new(|| Mutex::new(None));
static OBJ_HOLDER: Lazy<ObjectHolder> = Lazy::new(|| {
    let pak_dirs = PAK_DIRS.lock().unwrap();
    ObjectHolder::load(pak_dirs.as_ref().unwrap())
});
pub static OBJ_HOLDER_HASH: Lazy<u64> = Lazy::new(|| {
    use std::hash::{Hash, Hasher};
    let mut hasher = fnv::FnvHasher::default();
    get_objholder().hash(&mut hasher);
    hasher.finish()
});

pub fn get_objholder() -> &'static ObjectHolder {
    &OBJ_HOLDER
}

pub fn get_obj<T: ObjectIndex>(idx: T) -> &'static T::ObjectType {
    idx.get_obj_from_objholder(&OBJ_HOLDER)
}

pub fn id_to_idx<T: ObjectIndex + Default>(id: &str) -> T {
    T::search_idx(id, &OBJ_HOLDER).unwrap_or_default()
}

pub fn id_to_idx_checked<T: ObjectIndex>(id: &str) -> Option<T> {
    if let Some(idx) = T::search_idx(id, &OBJ_HOLDER) {
        Some(idx)
    } else {
        warn!("unknown id \"{}\" for {}", id, std::any::type_name::<T>());
        None
    }
}

pub fn idx_to_id<T: ObjectIndex>(idx: T) -> &'static str {
    idx.to_id(&OBJ_HOLDER)
}

pub fn get_by_id<T: FromId>(id: &str) -> &'static T {
    if let Some(s) = T::get_obj_from_objholder_by_id(id, &OBJ_HOLDER) {
        s
    } else {
        eprintln!("Object \"{id}\" is not found");
        panic!();
    }
}

pub fn get_by_id_checked<T: FromId>(id: &str) -> Option<&'static T> {
    if let Some(obj) = T::get_obj_from_objholder_by_id(id, &OBJ_HOLDER) {
        Some(obj)
    } else {
        warn!("unknown id \"{}\" for {}", id, std::any::type_name::<T>());
        None
    }
}

// serde_with implementaion
mod serde_with_impl {
    use super::*;
    use serde_with::{DeserializeAs, SerializeAs};

    /// serialize/deserialize object index as id string.
    /// gobj::init() must be called before using this.
    #[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
    pub struct ObjIdxAsId;

    impl<T> SerializeAs<T> for ObjIdxAsId
    where
        T: ObjectIndex + Copy,
    {
        fn serialize_as<S>(source: &T, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let id = idx_to_id(*source);
            serializer.serialize_str(id)
        }
    }

    impl<'de, T> DeserializeAs<'de, T> for ObjIdxAsId
    where
        T: ObjectIndex + Default + Sized,
    {
        fn deserialize_as<D>(deserializer: D) -> Result<T, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            use serde::Deserialize;
            let id = String::deserialize(deserializer)?;
            Ok(id_to_idx(&id))
        }
    }
}

pub use serde_with_impl::ObjIdxAsId;
