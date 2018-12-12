
use common::gamedata::CharaId;
use crate::game::{Game, DialogOpenRequest, DoPlayerAction, TalkText};
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
        DialogOpenRequest::Talk { cid, talk_text, } => {
            create_talk_dialog(talk_text, cid, game)?
        }
        DialogOpenRequest::ShopBuy { cid } => {
            let mut pa = DoPlayerAction::new(game);
            Box::new(ItemWindow::new(ItemWindowMode::ShopBuy { cid }, &mut pa))
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

pub fn create_talk_dialog(talk_text: TalkText, cid: CharaId, game: &mut Game) -> Option<Box<DialogWindow>> {
    let chara_template_idx = game.gd.chara.get(cid).template;
    
    let talk_window = talk_window::TalkWindow::new(talk_text, chara_template_idx);
    Some(Box::new(talk_window))
}

