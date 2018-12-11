
use crate::script::Value;
use crate::hashmap::HashMap;

/// Holds variables which are referenced in scripts
#[derive(Debug, Serialize, Deserialize)]
pub struct Variables {
    global: HashMap<String, Value>,
}

impl Variables {
    pub fn new() -> Variables {
        Variables {
            global: HashMap::default(),
        }
    }

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
}

