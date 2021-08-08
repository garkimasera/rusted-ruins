use super::item_info_window;
use super::item_window::*;
use super::msg_dialog;
use super::read_window;
use super::status_window;
use super::talk_window;
use super::DialogWindow;
use crate::game::{DialogOpenRequest, Game};
use common::gamedata::CharaId;
use script::TalkText;

pub fn create_dialog_from_request(
    req: DialogOpenRequest,
    game: &mut Game,
) -> Option<Box<dyn DialogWindow>> {
    Some(match req {
        DialogOpenRequest::YesNo { mut callback, msg } => {
            let msgdialog = msg_dialog::MsgDialog::with_yesno(&*msg, move |pa, n| {
                callback(pa, n == 0);
                super::DialogResult::Close
            });
            Box::new(msgdialog)
        }
        DialogOpenRequest::Talk { cid, talk_text } => create_talk_dialog(talk_text, cid, game)?,
        DialogOpenRequest::ItemInfo { il } => {
            Box::new(item_info_window::ItemInfoWindow::new(il, game))
        }
        DialogOpenRequest::CharaStatus { cid } => {
            Box::new(status_window::create_status_window_group(game, cid))
        }
        DialogOpenRequest::Read { title } => Box::new(read_window::ReadWindow::new(&title)),
        DialogOpenRequest::ShopBuy { cid } => {
            Box::new(ItemWindow::new(ItemWindowMode::ShopBuy { cid }, game))
        }
        DialogOpenRequest::ShopSell => Box::new(ItemWindow::new(ItemWindowMode::ShopSell, game)),
        DialogOpenRequest::RegisterAsShortcut { shortcut } => {
            Box::new(super::register_shortcut_dialog::RegisterShortcutDialog::new(shortcut))
        }
        DialogOpenRequest::PickUpItem => Box::new(ItemWindow::new(ItemWindowMode::PickUp, game)),
        DialogOpenRequest::Quest => Box::new(super::quest_window::QuestWindow::new(game)),
        DialogOpenRequest::GameOver => Box::new(super::exit_window::GameOverWindow::new()),
    })
}

pub fn create_talk_dialog(
    talk_text: TalkText,
    cid: Option<CharaId>,
    game: &mut Game,
) -> Option<Box<dyn DialogWindow>> {
    let chara_template_idx = cid.map(|cid| game.gd.chara.get(cid).idx);

    let talk_window = talk_window::TalkWindow::new(talk_text, chara_template_idx);
    Some(Box::new(talk_window))
}
