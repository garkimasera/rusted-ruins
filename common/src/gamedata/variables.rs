use crate::hashmap::HashMap;

/// Value is used to be stored in Variable.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub enum Value {
    None,
    Bool(bool),
    Int(i64),
}

impl Default for Value {
    fn default() -> Self {
        Value::None
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}

impl From<u32> for Value {
    fn from(value: u32) -> Self {
        Value::Int(value.into())
    }
}

/// Stores variables which are referenced in scripts
#[derive(Debug, Serialize, Deserialize)]
pub struct Variables {
    global: HashMap<String, Value>,
    local: HashMap<(String, String), Value>,
}

impl Default for Variables {
    fn default() -> Self {
        Variables {
            global: HashMap::default(),
            local: HashMap::default(),
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

    /// Get local named variable
    pub fn local_var(&self, script_id: &str, name: &str) -> Option<&Value> {
        self.local.get(&(script_id.to_owned(), name.to_owned()))
    }

    /// Get local named variable (mutable)
    pub fn local_mut(&mut self, script_id: &str, name: &str) -> Option<&mut Value> {
        self.local.get_mut(&(script_id.to_owned(), name.to_owned()))
    }

    /// Get local named variable
    pub fn set_local_var<S1: ToString, S2: ToString>(&mut self, script_id: S1, name: S2, v: Value) {
        self.local
            .insert((script_id.to_string(), name.to_string()), v);
    }
}
