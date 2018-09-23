
pub mod gen;

use std::borrow::Cow;
use common::gamedata::*;
use text;

/// Additional Site method
pub trait SiteEx {
    fn get_name(&self) -> Cow<str>;
}

impl SiteEx for Site {
    fn get_name(&self) -> Cow<str> {
        if let Some(ref name) = self.name {
            let name: &str = &*name;
            return name.into();
        }
        
        match self.content {
            SiteContent::AutoGenDungeon { dungeon_kind } => {
                text::to_txt(&dungeon_kind).into()
            }
            SiteContent::Town { ref town } => {
                text::obj_txt(town.id()).into()
            }
            SiteContent::Other => {
                warn!("Unnamed other kind site");
                "".into()
            }
        }
    }
}
