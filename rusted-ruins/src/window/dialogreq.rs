use super::build_obj_dialog;
use super::item_info_window;
use super::item_window::*;
use super::msg_dialog;
use super::read_window;
use super::status_window;
use super::talk_window;
use super::DialogWindow;
use crate::game::{DialogOpenRequest, Game};
use common::gamedata::{CharaId, ModuleSlotKind};
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
        DialogOpenRequest::BuildObj { il } => Box::new(build_obj_dialog::BuildObjDialog::new(il)),
        DialogOpenRequest::ItemInfo { il } => {
            Box::new(item_info_window::ItemInfoWindow::new(il, game))
        }
        DialogOpenRequest::CharaStatus { cid } => {
            Box::new(status_window::create_status_window_group(game, cid, false))
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
        DialogOpenRequest::QuestOffer => {
            Box::new(super::quest_window::QuestWindow::new_offer(&game.gd))
        }
        DialogOpenRequest::QuestReport => {
            Box::new(super::quest_window::QuestWindow::new_report(&game.gd))
        }
        DialogOpenRequest::InstallAbilitySlot => Box::new(
            super::slot_window::SlotInstallWindow::new(&game.gd, ModuleSlotKind::Ability),
        ),
        DialogOpenRequest::InstallExtendSlot => Box::new(
            super::slot_window::SlotInstallWindow::new(&game.gd, ModuleSlotKind::Extend),
        ),
        DialogOpenRequest::InsertModule => todo!(),
        DialogOpenRequest::GameOver => Box::new(super::exit_window::GameOverWindow::new()),
    })
}

pub fn create_talk_dialog(
    talk_text: TalkText,
    cid: Option<CharaId>,
    game: &mut Game,
) -> Option<Box<dyn DialogWindow>> {
    let talk_window = talk_window::TalkWindow::new(&game.gd, talk_text, cid);
    Some(Box::new(talk_window))
}
