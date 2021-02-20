use crate::hashmap::HashMap;
use crate::script::Value;

/// Holds variables which are referenced in scripts
#[derive(Debug, Serialize, Deserialize)]
pub struct Variables {
    global: HashMap<String, Value>,
}

impl Default for Variables {
    fn default() -> Self {
        Variables {
            global: HashMap::default(),
        }
    }
}

impl Variables {
    /// Get globally named variable
    pub fn global_var(&self, name: &str) -> Option<&Value> {
        self.global.get(name)
    }

    /// Get globally named variable (mutable)
    pub fn global_mut(&mut self, name: &str) -> Option<&mut Value> {
        self.global.get_mut(name)
    }

    /// Get globally named variable
    pub fn set_global_var<S: ToString>(&mut self, name: S, v: Value) {
        self.global.insert(name.to_string(), v);
    }

    /// Set special named variable "$?".
    /// This special variable is used for the result of the last instruction.
    pub fn set_last_result(&mut self, v: Value) {
        self.global.insert("?".to_owned(), v);
    }
}
