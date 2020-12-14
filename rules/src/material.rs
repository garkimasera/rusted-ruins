use common::gamedata::*;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Materials(HashMap<MaterialName, Material>);

impl Materials {
    pub fn get(&self, material_name: &MaterialName) -> &Material {
        if let Some(material) = self.0.get(material_name) {
            material
        } else {
            panic!("unknown material {}", material_name);
            // lazy_static! {
            //     static ref MATERIAL: Material = Matedial::default();
            // }
            // &*MATERIAL
        }
    }

    pub fn get_by_group(&self, group: &str, level: Option<u32>) -> Vec<(MaterialName, &Material)> {
        self.0
            .iter()
            .filter_map(|(k, v)| {
                if v.group != group {
                    return None;
                }
                if level.is_none() || level.unwrap() >= v.level {
                    Some((*k, v))
                } else {
                    None
                }
            })
            .collect()
    }
}

/// Rules for character generation
#[derive(Serialize, Deserialize, Default)]
pub struct Material {
    /// Group of this material
    pub group: String,
    /// Generation weight
    pub gen_weight: f32,
    /// Minimum level for generation
    pub level: u32,
    /// Weight factor
    pub w: f32,
    /// Dice factor
    pub dice: f32,
}
