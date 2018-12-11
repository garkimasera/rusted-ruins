
//! This module provides global state objholder

use std::path::PathBuf;
use std::sync::Mutex;
use lazy_static::lazy_static;
use crate::objholder::*;

/// Initialize lazy static
pub fn init(pak_dirs: Vec<PathBuf>) {
    *PAK_DIRS.lock().unwrap() = Some(pak_dirs);
    ::lazy_static::initialize(&OBJ_HOLDER);
}

lazy_static! {
    static ref PAK_DIRS: Mutex<Option<Vec<PathBuf>>> = Mutex::new(None);
    static ref OBJ_HOLDER: ObjectHolder = {
        let pak_dirs = PAK_DIRS.lock().unwrap();
        ObjectHolder::load(pak_dirs.as_ref().unwrap())
    };
}

pub fn get_objholder() -> &'static ObjectHolder {
    &*OBJ_HOLDER
}

pub fn get_obj<T: ObjectIndex>(idx: T) -> &'static T::ObjectType {
    idx.get_obj_from_objholder(&OBJ_HOLDER)
}

pub fn id_to_idx<T: ObjectIndex + Default>(id: &str) -> T {
    T::search_idx(id, &OBJ_HOLDER).unwrap_or_default()
}

pub fn id_to_idx_checked<T: ObjectIndex>(id: &str) -> Option<T> {
    T::search_idx(id, &OBJ_HOLDER)
}

pub fn idx_to_id<T: ObjectIndex>(idx: T) -> &'static str {
    idx.to_id(&OBJ_HOLDER)
}

pub fn get_by_id<T: FromId>(id: &str) -> &'static T {
    if let Some(s) = T::get_obj_from_objholder_by_id(id, &OBJ_HOLDER) {
        s
    } else {
        eprintln!("Object \"{}\" is not found", id);
        panic!();
    }
}

pub fn get_by_id_checked<T: FromId>(id: &str) -> Option<&'static T> {
    T::get_obj_from_objholder_by_id(id, &OBJ_HOLDER)
}

