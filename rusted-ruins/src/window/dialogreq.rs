
use common::gamedata::chara::CharaTalk;
use game::DialogOpenRequest;
use super::{Window, DialogWindow, DialogResult};
use super::talkwindow;

pub fn create_dialog_from_request(req: DialogOpenRequest) -> Box<DialogWindow> {
    match req {
        DialogOpenRequest::Talk(chara_talk) => {
            create_talk_dialog(chara_talk)
        }
    }
}

pub fn create_talk_dialog(chara_talk: CharaTalk) -> Box<DialogWindow> {
    let talk_window = talkwindow::TalkWindow::new(chara_talk);
    Box::new(talk_window)
}

