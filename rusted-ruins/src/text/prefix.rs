use super::misc_txt;
use common::gamedata::MaterialName;

pub fn material(name: MaterialName) -> String {
    misc_txt(&format!("material-{}", name))
}

pub fn material_group(group: &str) -> String {
    misc_txt(&format!("material_group-{}", group))
}
