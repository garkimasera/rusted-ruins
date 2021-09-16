use crate::gamedata::{CharaId, Value};

/// Stores data for script execution
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ScriptExec {
    pub current_script_id: Option<String>,
    pub target_chara: Option<CharaId>,
    pub response: Option<Value>,
    pub talking: bool,
}

impl ScriptExec {
    /// Clear current script execution data.
    pub fn clear(&mut self) {
        self.current_script_id = None;
        self.target_chara = None;
        self.response = None;
        self.talking = false;
    }
}
