
use common::basic::*;
use common::objholder::*;
use crate::config::SCREEN_CFG;
use super::commonuse::*;
use super::widget::WidgetTrait;
use super::widget::LabelWidget;
use crate::context::textrenderer::FontKind;

#[derive(Clone, Copy)]
pub struct MemberInfo {
    pub creator: fn(&Game) -> Box<DialogWindow>,
    pub idx: UIImgIdx,
    pub text_id: &'static str,
}

/// GroupWindow manages multiple windows.
/// Player can switches displaying windows.
pub struct GroupWindow {
    size: usize,
    current_window: usize,
    members: Vec<Option<Box<DialogWindow>>>,
    mem_info: Vec<MemberInfo>,
    tab_navigator: TabsNavigator,
}

impl GroupWindow {
    pub fn new(size: usize, init_win: usize, game: &Game, mem_info: Vec<MemberInfo>) -> GroupWindow {
        assert!(init_win < size);
        let members: Vec<Option<Box<DialogWindow>>> = (0..size).into_iter().map(|_| None).collect();
        let tab_navigator = TabsNavigator::new(mem_info.clone(), init_win);
        
        let mut group_window = GroupWindow {
            size,
            current_window: init_win,
            members,
            mem_info,
            tab_navigator,
        };
        group_window.switch(init_win, game);
        group_window
    }

    pub fn switch(&mut self, i_win: usize, game: &Game) {
        assert!(i_win < self.size);
        self.current_window = i_win;
        if self.members[i_win].is_none() {
            self.members[i_win] = Some((self.mem_info[i_win].creator)(game));
        }
        self.tab_navigator.set_current_tab(i_win);
    }

    pub fn rotate_right(&mut self, game: &Game) {
        let result = if self.current_window + 1 < self.size {
            self.current_window + 1
        } else {
            0
        };
        self.switch(result, game);
    }

    pub fn rotate_left(&mut self, game: &Game) {
        let result = if self.current_window > 0 {
            self.current_window - 1
        } else {
            self.size - 1
        };
        self.switch(result, game);
    }
}

impl Window for GroupWindow {
    fn draw(&mut self, context: &mut Context, game: &Game, anim: Option<(&Animation, u32)>) {

        self.tab_navigator.draw(context, game, anim);

        if let Some(ref mut member) = self.members[self.current_window] {
            member.draw(context, game, anim);
        }
    }
}

impl DialogWindow for GroupWindow {
    fn process_command(&mut self, command: &Command, pa: &mut DoPlayerAction) -> DialogResult {
        match command {
            &Command::RotateWindowRight => {
                self.rotate_right(pa.game());
                return DialogResult::Continue;
            }
            &Command::RotateWindowLeft => {
                self.rotate_left(pa.game());
                return DialogResult::Continue;
            }
            _ => (),
        }
        if let Some(ref mut member) = self.members[self.current_window] {
            match member.process_command(command, pa) {
                DialogResult::Close => DialogResult::Close,
                _ => DialogResult::Continue
            }
        } else {
            DialogResult::Continue
        }
    }

    fn mode(&self) -> InputMode {
        InputMode::Dialog
    }
}

/// Display icons and texts of tabs
struct TabsNavigator {
    rect: Rect,
    i: usize,
    mem_info: Vec<MemberInfo>,
    labels: Vec<LabelWidget>,
}

impl TabsNavigator {
    fn new(mem_info: Vec<MemberInfo>, init: usize) -> TabsNavigator {
        assert!(mem_info.len() > 0);

        let size = mem_info.len();
        let w = TAB_ICON_W * size as u32;
        let screen_w = SCREEN_CFG.screen_w;
        let x = screen_w as i32 - w as i32;
        let labels: Vec<LabelWidget> = mem_info
            .iter()
            .map(|member| member.text_id)
            .enumerate()
            .map(|(i, text_id)| LabelWidget::bordered(
                Rect::new(i as i32 * TAB_ICON_W as i32, TAB_ICON_H as i32, TAB_ICON_W, TAB_TEXT_H),
                crate::text::ui_txt(text_id),
                FontKind::S).centering())
            .collect();
        
        TabsNavigator {
            rect: Rect::new(x, 0, w, TAB_ICON_H + TAB_TEXT_H),
            i: init,
            mem_info,
            labels,
        }
    }

    fn set_current_tab(&mut self, i: usize) {
        self.i = i;
    }
}

impl Window for TabsNavigator {
    fn draw(
        &mut self, context: &mut Context, _game: &Game, _anim: Option<(&Animation, u32)>) {

        context.canvas.set_viewport(self.rect);

        for (i, member) in self.mem_info.iter().enumerate() {
            let dest_rect = Rect::new(TAB_ICON_W as i32 * i as i32, 0, TAB_ICON_W, TAB_ICON_H);
            let tex= context.sv.tex().get(member.idx);
            check_draw!(context.canvas.copy(&tex, None, dest_rect));
        }

        // Draw labels
        for label in &mut self.labels {
            label.draw(context);
        }

        // Draw border for selected tab
        let h = TAB_ICON_H + TAB_TEXT_H;
        let rect = Rect::new(TAB_ICON_W as i32 * self.i as i32, 0, TAB_ICON_W, h);
        context.canvas.set_draw_color(UI_CFG.color.tab_select_border_light.into());
        check_draw!(context.canvas.draw_rect(rect));
        let rect = Rect::new(TAB_ICON_W as i32 * self.i as i32 + 1, 1, TAB_ICON_W - 2, h - 2);
        context.canvas.set_draw_color(UI_CFG.color.tab_select_border_dark.into());
        check_draw!(context.canvas.draw_rect(rect));
    }
}

