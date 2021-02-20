use crate::game::Command;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct InputConfig {
    pub wasd_mode: bool,
    pub normal: HashMap<String, Command>,
    pub dialog: HashMap<String, Command>,
}

impl InputConfig {
    pub fn find_key(&self, command: &Command) -> String {
        let mut s = String::new();

        for (k, c) in &self.normal {
            if *c != *command {
                continue;
            }

            if !s.is_empty() {
                s.push(',');
            }
            s.push_str(k);
        }
        s
    }
}
