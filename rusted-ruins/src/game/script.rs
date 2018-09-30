//! Script engine implementation

use common::gobj;
use common::script::*;
use super::Game;

pub struct ScriptEngine {
    script: &'static Script,
    pos: ScriptPos,
}

#[derive(Debug)]
pub enum ExecResult {
    Talk(&'static str, &'static [(String, String)]),
    Finish,
}

impl ScriptEngine {
    pub fn new(id: &str) -> ScriptEngine {
        let script_obj: &ScriptObject = gobj::get_by_id(id);
        ScriptEngine {
            script: &script_obj.script,
            pos: ScriptPos {
                section: "start".to_owned(),
                i: 0
            },
        }
    }

    pub fn exec(&mut self, game: &mut Game) -> ExecResult {
        let result = loop {
            let instruction = if let Some(instruction) = self.script.get(&self.pos) {
                instruction
            } else {
                break ExecResult::Finish;
            };

            match instruction {
                Instruction::Talk(text_id, choices) => {
                    break ExecResult::Talk(&text_id, &choices);
                }
                _ => unimplemented!()
            }
        };
        
        self.pos.advance();
        result
    }
}

