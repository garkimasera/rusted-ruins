use super::{DialogOpenRequest, Game};
use common::gamedata::*;
use script::{ScriptResult, TalkText};

pub enum AdvanceScriptResult {
    Continue,
    UpdateTalkText(TalkText),
    Quit,
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct ScriptState {
    talking: bool,
    dialog: bool,
    /// Target character of talking
    target_cid: Option<CharaId>,
}

impl ScriptState {
    fn clear(&mut self) {
        *self = Self::default();
    }
}

impl Game {
    /// Start script. Give cid if talk.
    pub fn start_script(&mut self, id: &str, cid: Option<CharaId>, scene: Option<String>) {
        info!("Start script {}", id);
        self.script_state.target_cid = cid;
        self.se.start_script(id, scene);

        self.advance_script(None);
    }

    /// Advance current script.
    /// `ui_response` is needed if the previous result is ui request.
    pub fn advance_script(&mut self, ui_response: Option<Value>) -> AdvanceScriptResult {
        if let Some(ui_response) = ui_response {
            self.script_state.dialog = false;
            self.se.ui_response(ui_response);
        } else if self.script_state.dialog {
            self.script_state.dialog = false;
            self.se.ui_response(Value::None);
        }

        let result = match self.se.next(&mut self.gd) {
            ScriptResult::Finish => AdvanceScriptResult::Quit,
            ScriptResult::UiRequest(script::UiRequest::Talk { talk }) => {
                if self.script_state.talking {
                    AdvanceScriptResult::UpdateTalkText(talk)
                } else {
                    self.script_state.talking = true;
                    self.request_dialog_open(DialogOpenRequest::Talk {
                        cid: self.script_state.target_cid,
                        talk_text: talk,
                    });
                    AdvanceScriptResult::Continue
                }
            }
            ScriptResult::UiRequest(script::UiRequest::ShopBuy) => {
                if let Some(cid) = self.script_state.target_cid {
                    self.request_dialog_open(DialogOpenRequest::ShopBuy { cid });
                    self.script_state.dialog = true;
                    AdvanceScriptResult::Continue
                } else {
                    AdvanceScriptResult::Quit
                }
            }
            ScriptResult::UiRequest(script::UiRequest::ShopSell) => {
                if let Some(_cid) = self.script_state.target_cid {
                    self.request_dialog_open(DialogOpenRequest::ShopSell);
                    self.script_state.dialog = true;
                    AdvanceScriptResult::Continue
                } else {
                    AdvanceScriptResult::Quit
                }
            }
            ScriptResult::UiRequest(script::UiRequest::QuestOffer) => {
                crate::game::quest::update_town_quest(&mut self.gd);
                self.request_dialog_open(DialogOpenRequest::QuestOffer);
                self.script_state.dialog = true;
                AdvanceScriptResult::Continue
            }
            ScriptResult::UiRequest(script::UiRequest::QuestReport) => {
                crate::game::quest::update_quest_status(&mut self.gd);
                self.request_dialog_open(DialogOpenRequest::QuestReport);
                self.script_state.dialog = true;
                AdvanceScriptResult::Continue
            }
        };
        if matches!(result, AdvanceScriptResult::Quit) {
            self.script_state.clear();
        }
        result
    }
}
