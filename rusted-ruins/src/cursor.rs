use std::cell::RefCell;

use anyhow::{anyhow, Error};
use common::hashmap::HashMap;
use common::{gobj, obj::UiImgObject};
use sdl2::image::ImageRWops;
use sdl2::mouse::Cursor as SdlCursor;
use sdl2::rwops::RWops;

thread_local!(
    static CURSORS: RefCell<Option<HashMap<Cursor, SdlCursor>>> = RefCell::new(None);
);

pub fn load() {
    let mut cursors = HashMap::default();

    for cursor in Cursor::ALL {
        match cursor.load() {
            Ok(loaded_cursor) => {
                cursors.insert(*cursor, loaded_cursor);
            }
            Err(e) => {
                warn!("Loading cursor \"{}\" failed\n{}", cursor.img_id(), e);
            }
        }
    }

    CURSORS.with(|c| {
        *c.borrow_mut() = Some(cursors);
    });

    set(Cursor::Normal);
}

pub fn set(cursor: Cursor) {
    CURSORS.with(|c| {
        if let Some(c) = c.borrow().as_ref().and_then(|c| c.get(&cursor)) {
            c.set();
        }
    });
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Cursor {
    Normal,
}

impl Cursor {
    const ALL: &'static [Cursor] = &[Cursor::Normal];

    fn img_id(&self) -> &'static str {
        match self {
            Cursor::Normal => "!cursor-normal",
        }
    }

    fn load(&self) -> Result<SdlCursor, Error> {
        let to_err = |s: String| anyhow!("{}", s);

        let obj: &UiImgObject =
            gobj::get_by_id_checked(self.img_id()).ok_or_else(|| anyhow!("object not found"))?;

        let rwops = RWops::from_bytes(&obj.img.data).map_err(to_err)?;
        let surface = rwops.load().map_err(to_err)?;

        SdlCursor::from_surface(surface, obj.hot.0.into(), obj.hot.1.into()).map_err(to_err)
    }
}
