
use common::obj::TalkScriptObject;
use common::objholder::TalkScriptIdx;
use common::gamedata::chara::{CharaId, CharaTalk};
use common::gobj;
use game::Game;
use text;

/// Hold data for talk handling
pub struct TalkStatus {
    idx: TalkScriptIdx,
    /// The chara that player talks to
    cid: CharaId,
    chara_talk: CharaTalk,
    current_section: String,
}

impl TalkStatus {
    pub fn new(chara_talk: CharaTalk, cid: CharaId, game: &mut Game) -> Option<TalkStatus> {
        let idx = gobj::id_to_idx_checked(&chara_talk.id)?;
        let current_section = chara_talk.section.clone();
        Some(TalkStatus {
            idx, cid, chara_talk,
            current_section,
        })
    }

    pub fn get_text(&self) -> &'static str {
        let tso = gobj::get_obj(self.idx);
        let text = tso.get_section_text(&self.current_section).unwrap();
        text::talk_txt(&text)
    }
}

