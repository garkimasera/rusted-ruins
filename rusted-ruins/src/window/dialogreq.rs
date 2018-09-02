
use common::gamedata::{CharaId, CharaTalk};
use game::{Game, DialogOpenRequest, TalkManager, DoPlayerAction};
use super::DialogWindow;
use super::talk_window;
use super::item_window::*;
use super::msg_dialog;

pub fn create_dialog_from_request(req: DialogOpenRequest, game: &mut Game) -> Option<Box<DialogWindow>> {
    Some(match req {
        DialogOpenRequest::YesNo { mut callback, msg } => {
            let msgdialog = msg_dialog::MsgDialog::with_yesno(
                &*msg,
                move |pa, n| {
                    callback(pa, n == 0);
                    super::DialogResult::Close
                }
            );
            Box::new(msgdialog)
        }
        DialogOpenRequest::Talk { chara_talk, cid } => {
            create_talk_dialog(chara_talk, cid, game)?
        }
        DialogOpenRequest::ShopBuy { cid } => {
            return None;
        }
        DialogOpenRequest::ShopSell => {
            let mut pa = DoPlayerAction::new(game);
            Box::new(ItemWindow::new(ItemWindowMode::ShopSell, &mut pa))
        }
        DialogOpenRequest::GameOver => {
            Box::new(super::exit_window::GameOverWindow::new())
        }
    })
}

pub fn create_talk_dialog(
    chara_talk: CharaTalk, cid: CharaId, game: &mut Game) -> Option<Box<DialogWindow>> {
        
    let talk_status = TalkManager::new(chara_talk, cid, game)?;
    let chara_template_idx = game.gd.chara.get(cid).template;
    
    let talk_window = talk_window::TalkWindow::new(talk_status, chara_template_idx);
    Some(Box::new(talk_window))
}

