
use common::objholder::*;

use config::PAK_DIRS;

/// Initialize lazy static
pub fn init() {
    ::lazy_static::initialize(&OBJ_HOLDER);
}

lazy_static! {
    static ref OBJ_HOLDER: ObjectHolder = {
        ObjectHolder::load(&PAK_DIRS)
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

pub fn idx_to_id<T: ObjectIndex>(idx: T) -> &'static str {
    idx.to_id(&OBJ_HOLDER)
}

pub fn get_by_id<T: FromId>(id: &str) -> &'static T {
    T::get_obj_from_objholder_by_id(id, &OBJ_HOLDER).unwrap()
}


