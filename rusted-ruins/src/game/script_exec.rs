use super::{DialogOpenRequest, Game};
use common::gamedata::*;
use common::gobj;
use common::obj::ScriptObject;
use script::{ScriptYield, TalkText};

pub enum AdvanceScriptResult {
    Continue,
    UpdateTalkText(TalkText),
    Quit,
}

impl<'s> Game<'s> {
    /// Start script. Give cid if talk.
    pub fn start_script(&mut self, id: &str, cid: Option<CharaId>, scene: Option<String>) {
        self.gd.script_exec.current_script_id = Some(id.into());
        self.gd.script_exec.target_chara = cid;
        self.gd.script_exec.scene = scene;

        let script_obj: &ScriptObject = if let Some(script_obj) = gobj::get_by_id_checked(id) {
            script_obj
        } else {
            warn!("script id \"{}\" not found", id);
            self.gd.script_exec.clear();
            return;
        };
        if let Err(e) = self.se.start(script_obj, id) {
            warn!("script \"{}\" starting failed:\n{}", id, e);
            self.gd.script_exec.clear();
            return;
        }
        self.advance_script(None);
    }

    /// Advance current script.
    /// When called by advance_talk, give player's choice.
    pub fn advance_script(&mut self, choice: Option<u32>) -> AdvanceScriptResult {
        self.gd.script_exec.response = choice.map(|n| Value::Int(n.into()));
        let result = match self.se.next(&mut self.gd) {
            Ok(result) => result,
            Err(e) => {
                warn!("script execution failed:\n{}", e);
                self.gd.script_exec.clear();
                return AdvanceScriptResult::Quit;
            }
        };
        let result = if let Some(result) = result {
            result
        } else {
            self.gd.script_exec.clear();
            return AdvanceScriptResult::Quit;
        };

        match result {
            ScriptYield::Talk { talk } => {
                if !self.gd.script_exec.talking {
                    self.gd.script_exec.talking = true;
                    self.request_dialog_open(DialogOpenRequest::Talk {
                        cid: self.gd.script_exec.target_chara,
                        talk_text: talk,
                    });
                    AdvanceScriptResult::Continue
                } else {
                    AdvanceScriptResult::UpdateTalkText(talk)
                }
            }
            ScriptYield::ShopBuy => {
                let cid = if let Some(cid) = self.gd.script_exec.target_chara {
                    cid
                } else {
                    return AdvanceScriptResult::Quit;
                };
                self.request_dialog_open(DialogOpenRequest::ShopBuy { cid });
                AdvanceScriptResult::Continue
            }
            ScriptYield::ShopSell => {
                self.request_dialog_open(DialogOpenRequest::ShopSell);
                AdvanceScriptResult::Continue
            }
            ScriptYield::Quest => {
                crate::game::quest::update_town_quest(&mut self.gd);
                self.request_dialog_open(DialogOpenRequest::Quest);
                AdvanceScriptResult::Continue
            }
        }
    }
}
