//! Script engine implementation

use common::gobj;
use common::gamedata::*;
use common::script::*;

pub struct ScriptEngine {
    script: &'static Script,
    pos: ScriptPos,
    cid: Option<CharaId>,
    talking: bool,
}

#[derive(PartialEq, Eq, Debug)]
pub enum ExecResult {
    Talk(CharaId, TalkText, bool),
    Finish,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct TalkText {
    pub text_id: &'static str,
    pub choices: Option<&'static [(String, String)]>,
}

impl ScriptEngine {
    pub fn new(id: &str, cid: Option<CharaId>) -> ScriptEngine {
        let script_obj: &ScriptObject = gobj::get_by_id(id);
        ScriptEngine {
            script: &script_obj.script,
            pos: ScriptPos {
                section: "start".to_owned(),
                i: 0
            },
            cid,
            talking: false,
        }
    }

    pub fn exec(&mut self, gd: &mut GameData) -> ExecResult {
        let result = loop {
            let instruction = if let Some(instruction) = self.script.get(&self.pos) {
                instruction
            } else {
                break ExecResult::Finish;
            };

            match instruction {
                Instruction::Talk(text_id, choices) => {
                    let cid = if let Some(cid) = self.cid {
                        cid
                    } else {
                        warn!("script error: CharaId is not specified");
                        return ExecResult::Finish;
                    };

                    let need_open_talk_dialog = if self.talking {
                        false
                    } else {
                        self.talking = true;
                        true
                    };
                    
                    let choices = if choices.is_empty() { None } else { Some(choices.as_ref()) };
                    return ExecResult::Talk(
                        cid, TalkText { text_id, choices }, need_open_talk_dialog );
                }
                _ => unimplemented!(),
            }
            self.pos.advance();
        };
        
        self.pos.advance();
        result
    }

    pub fn continue_talk(&mut self, gd: &mut GameData, choice: Option<u32>) -> ExecResult {
        match self.script.get(&self.pos).expect("instruction not found") {
            Instruction::Talk(text_id, choices) => {
                if let Some(c) = choice {
                    self.pos.jump(&choices[c as usize].1);
                } else {
                    assert!(choices.is_empty());
                    self.pos.advance();
                }
                self.exec(gd)
            }
            _ => unreachable!(),
        }
    }
}

