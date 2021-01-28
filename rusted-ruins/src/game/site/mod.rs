pub mod gen;
pub mod temp;

use common::gamedata::*;

/// Additional Site method
pub trait SiteEx {}

impl SiteEx for Site {}
