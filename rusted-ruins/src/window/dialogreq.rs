
use common::gamedata::chara::{CharaId, CharaTalk};
use game::{Game, DialogOpenRequest, TalkStatus};
use super::DialogWindow;
use super::talkwindow;

pub fn create_dialog_from_request(req: DialogOpenRequest, game: &mut Game) -> Option<Box<DialogWindow>> {
    Some(match req {
        DialogOpenRequest::Talk { chara_talk, cid } => {
            create_talk_dialog(chara_talk, cid, game)?
        }
    })
}

pub fn create_talk_dialog(
    chara_talk: CharaTalk, cid: CharaId, game: &mut Game) -> Option<Box<DialogWindow>> {
        
    let talk_status = TalkStatus::new(chara_talk, cid, game)?;
    
    let talk_window = talkwindow::TalkWindow::new(talk_status);
    Some(Box::new(talk_window))
}

