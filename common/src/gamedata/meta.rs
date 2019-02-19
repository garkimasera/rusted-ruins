/// Meta data
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct MetaData {
    /// Save directory name
    save_name: String,
}

impl MetaData {
    pub fn save_name(&self) -> &str {
        &self.save_name
    }

    pub fn set_save_name(&mut self, s: &str) {
        self.save_name = s.to_owned();
    }
}

impl Default for MetaData {
    fn default() -> MetaData {
        MetaData {
            save_name: "uninit".to_owned(),
        }
    }
}
