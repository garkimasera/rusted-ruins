
use std::borrow::Cow;
use common::talkscript::*;
use common::objholder::TalkScriptIdx;
use common::gamedata::chara::{CharaId, CharaTalk};
use common::gobj;
use game::{Game, DoPlayerAction};

/// Hold data for talk handling
pub struct TalkStatus {
    idx: TalkScriptIdx,
    /// The chara that player talks to
    cid: CharaId,
    current_section: String,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TalkResult {
    Continue, NoChange, End,
}                    

impl TalkStatus {
    pub fn new(chara_talk: CharaTalk, cid: CharaId, game: &mut Game) -> Option<TalkStatus> {
        let idx = gobj::id_to_idx_checked(&chara_talk.id)?;
        let current_section = chara_talk.section.clone();

        let mut talk_status = TalkStatus {
            idx, cid, current_section,
        };
        talk_status.proceed_loop(game);
        Some(talk_status)
    }

    pub fn get_text(&self) -> Cow<'static, str> {
        let tso = gobj::get_obj(self.idx);
        if let Some(text) = tso.get_section_text(&self.current_section) {
            text
        } else {
            unreachable!()
        }
    }

    /// Return answers of the current section
    pub fn get_answers(&self) -> Option<&'static [String]> {
        let section = self.get_current_section().expect("Tried to get answers of finished talk");
        match *section {
            TalkSection::Normal { ref answer_texts, .. } => Some(answer_texts),
            _ => None,
        }
    }

    /// Proceed to next section
    pub fn proceed(&mut self, pa: &mut DoPlayerAction) -> TalkResult {
        let game = &mut pa.0;
        let section = if let Some(section) = self.get_current_section() {
            section
        } else {
            return TalkResult::End;
        };
        // Set next section
        self.current_section = match *section {
            TalkSection::Normal { ref default_dest_section, .. } => {
                if let Some(ref default_dest_section) = *default_dest_section {
                    default_dest_section.clone()
                } else {
                    return TalkResult::NoChange; // No need to update talk displaying
                }
            }
            TalkSection::Reaction { ref next_section, .. } => next_section.clone(),
            TalkSection::Special { ref next_section, .. } => next_section.clone(),
        };
        self.proceed_loop(game)
    }

    /// Proceed until reaching a section that wait for player's input
    fn proceed_loop(&mut self, _game: &mut Game) -> TalkResult {
        loop {
            // Empty section id means to finish the talk
            if self.current_section == "" {
                return TalkResult::End
            }
            let section = if let Some(section) = self.get_current_section() {
                section
            } else {
                return TalkResult::End;
            };
            match *section {
                TalkSection::Normal {  .. } => {
                    return TalkResult::Continue;
                }
                TalkSection::Reaction { ref next_section, .. } => {
                    // process reaction here
                    self.current_section = next_section.clone();
                    continue;
                }
                TalkSection::Special { ref next_section, .. } => {
                    self.current_section = next_section.clone();
                    unimplemented!();
                }
            }
        }
    }

    fn get_current_section(&self) -> Option<&'static TalkSection> {
        let tso = gobj::get_obj(self.idx);
        let section = tso.sections.get(&self.current_section);
        if section.is_none() {
            warn!("TalkSection \"{}\" is not found in TalkScript \"{}\"",
                  self.current_section, gobj::idx_to_id(self.idx));
        }
        section
    }
}

