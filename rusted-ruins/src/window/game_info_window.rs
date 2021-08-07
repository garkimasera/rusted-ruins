use super::commonuse::*;
use super::widget::*;
use super::group_window::*;
use crate::config::UI_CFG;
use crate::context::textrenderer::FontKind;
use crate::text::ui_txt;
use common::gobj;

/// Game play information viewer
pub struct GameInfoWindow {
    rect: Rect,
    money_label: LabelWidget,
    play_time_label: LabelWidget,
    escape_click: bool,
}

const GAME_INFO_WINDOW_GROUP_SIZE: u32 = 1;

pub fn create_game_info_window_group(game: &Game) -> GroupWindow {
    let mem_info: Vec<(MemberInfo, ChildWinCreator)> = vec![
        (
            MemberInfo {
                idx: gobj::id_to_idx("!tab-icon-game-info"),
                text_id: "tab_text-game_info",
            },
            Box::new(move |game| Box::new(GameInfoWindow::new(game))),
        ),
    ];
    let rect: Rect = UI_CFG.info_window.rect.into();
    GroupWindow::new(
        "game_info",
        GAME_INFO_WINDOW_GROUP_SIZE,
        None,
        game,
        mem_info,
        (rect.x, rect.y),
    )
}

impl GameInfoWindow {
    pub fn new(game: &Game) -> GameInfoWindow {
        let cfg = &UI_CFG.game_info_window;
        let rect: Rect = UI_CFG.info_window.rect.into();

        let money_label = LabelWidget::new(
            cfg.money_label_rect,
            &format!("{} Gold", game.gd.player.money()),
            FontKind::M,
        );

        let play_time_label = LabelWidget::new(cfg.play_time_label_rect, "", FontKind::M);

        let mut game_info_win = GameInfoWindow {
            rect,
            money_label,
            play_time_label,
            escape_click: false,
        };

        game_info_win.update();
        game_info_win
    }

    fn update(&mut self) {
        let s = crate::game::play_time::play_time_as_secs();
        let text = format!(
            "{}  {:02}:{:02}:{:02}",
            ui_txt("label_text-play_time"),
            s / 3600,
            (s / 60) % 60,
            s % 60
        );
        self.play_time_label.set_text(text);
    }
}

impl Window for GameInfoWindow {
    fn draw(&mut self, context: &mut Context, _game: &Game, _anim: Option<(&Animation, u32)>) {
        self.update();

        draw_window_border(context, self.rect);
        self.money_label.draw(context);
        self.play_time_label.draw(context);
    }
}

impl DialogWindow for GameInfoWindow {
    fn process_command(&mut self, command: &Command, _pa: &mut DoPlayerAction) -> DialogResult {
        check_escape_click!(self, command);

        match *command {
            Command::Cancel => DialogResult::Close,
            _ => DialogResult::Continue,
        }
    }

    fn mode(&self) -> InputMode {
        InputMode::Dialog
    }
}
