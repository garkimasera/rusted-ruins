use super::commonuse::*;
use super::widget::LabelWidget;
use super::widget::WidgetTrait;
use crate::context::textrenderer::FontKind;
use common::basic::*;
use common::objholder::*;

#[derive(Clone, Copy)]
pub struct MemberInfo {
    pub creator: fn(&Game) -> Box<dyn DialogWindow>,
    pub idx: UIImgIdx,
    pub text_id: &'static str,
}

/// GroupWindow manages multiple windows.
/// Player can switches displaying windows.
pub struct GroupWindow {
    size: usize,
    current_window: usize,
    members: Vec<Option<Box<dyn DialogWindow>>>,
    mem_info: Vec<MemberInfo>,
    tab_navigator: TabsNavigator,
}

impl GroupWindow {
    pub fn new(
        size: usize,
        init_win: usize,
        game: &Game,
        mem_info: Vec<MemberInfo>,
        window_top_left: (i32, i32),
    ) -> GroupWindow {
        assert!(init_win < size);
        let members: Vec<Option<Box<dyn DialogWindow>>> = (0..size).map(|_| None).collect();
        let tab_navigator = TabsNavigator::new(window_top_left, mem_info.clone(), init_win);

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
        if let Some(ref mut member) = self.members[self.current_window] {
            member.draw(context, game, anim);
        }

        self.tab_navigator.draw(context, game, anim);
    }
}

impl DialogWindow for GroupWindow {
    fn process_command(&mut self, command: &Command, pa: &mut DoPlayerAction) -> DialogResult {
        match command {
            Command::RotateWindowRight => {
                self.rotate_right(pa.game());
                return DialogResult::Continue;
            }
            Command::RotateWindowLeft => {
                self.rotate_left(pa.game());
                return DialogResult::Continue;
            }
            _ => (),
        }
        if let Some(ref mut member) = self.members[self.current_window] {
            match member.process_command(command, pa) {
                DialogResult::Close => DialogResult::Close,
                _ => DialogResult::Continue,
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
    fn new(p: (i32, i32), mem_info: Vec<MemberInfo>, init: usize) -> TabsNavigator {
        assert!(!mem_info.is_empty());

        let size = mem_info.len();
        let w = TAB_ICON_W * size as u32;
        let h = TAB_ICON_H + TAB_TEXT_H;
        let rect = Rect::new(p.0, p.1 - h as i32 - WINDOW_BORDER_THICKNESS as i32, w, h);
        let labels: Vec<LabelWidget> = mem_info
            .iter()
            .map(|member| member.text_id)
            .enumerate()
            .map(|(i, text_id)| {
                LabelWidget::bordered(
                    Rect::new(
                        i as i32 * TAB_ICON_W as i32,
                        TAB_ICON_H as i32,
                        TAB_ICON_W,
                        TAB_TEXT_H,
                    ),
                    crate::text::ui_txt(text_id),
                    FontKind::S,
                )
                .centering()
            })
            .collect();

        TabsNavigator {
            rect,
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
    fn draw(&mut self, context: &mut Context, _game: &Game, _anim: Option<(&Animation, u32)>) {
        lazy_static! {
            static ref MAKE_DARK_IDX: UIImgIdx = common::gobj::id_to_idx("!make-dark");
        };
        crate::draw::border::draw_rect_border(context, self.rect);

        use sdl2::pixels::Color;
        let window_bg: Color = UI_CFG.color.window_bg.into();
        let border_light: Color = UI_CFG.color.border_light.into();
        let border_dark: Color = UI_CFG.color.border_dark.into();

        for (i, member) in self.mem_info.iter().enumerate() {
            let dest_rect = Rect::new(TAB_ICON_W as i32 * i as i32, 0, TAB_ICON_W, TAB_ICON_H);
            context.render_tex(member.idx, dest_rect);
        }

        // Draw labels
        for label in &mut self.labels {
            label.draw(context);
        }

        // Erase border between tabs and window
        context.set_viewport({
            let mut r = self.rect;
            r.set_height(self.rect.height() + WINDOW_BORDER_THICKNESS);
            r
        });
        let w = TAB_ICON_W;
        let h = TAB_ICON_H + TAB_TEXT_H;
        let r = Rect::new(
            0,
            h as i32,
            w * self.mem_info.len() as u32,
            WINDOW_BORDER_THICKNESS,
        );
        context.canvas.set_draw_color(window_bg);
        try_sdl!(context.canvas.fill_rect(r));

        // Draw borders
        context.canvas.set_draw_color(border_light);
        for i in 0..self.mem_info.len() {
            let (i, w, h) = (i as i32, w as i32, h as i32);

            if self.i as i32 != i {
                // Draw horizontal border
                context.canvas.set_draw_color(border_dark);
                try_sdl!(context.canvas.draw_line((i * w, h), ((i + 1) * w, h)));
                context.canvas.set_draw_color(border_light);
                try_sdl!(context
                    .canvas
                    .draw_line((i * w, h + 1), ((i + 1) * w, h + 1)));
                context.canvas.set_draw_color(border_dark);
                try_sdl!(context
                    .canvas
                    .draw_line((i * w + 1, h + 2), ((i + 1) * w + 1, h + 2)));

                // Make rendered text and icon dark if not selected
                context.render_tex(
                    *MAKE_DARK_IDX,
                    Rect::new(
                        w * i + WINDOW_BORDER_THICKNESS as i32,
                        0,
                        w as u32 - WINDOW_BORDER_THICKNESS,
                        h as u32,
                    ),
                );
            }

            // Draw vertical border
            context.canvas.set_draw_color(border_dark);
            try_sdl!(context
                .canvas
                .draw_line(((i + 1) * w - 1, 0), ((i + 1) * w - 1, h + 1)));
            context.canvas.set_draw_color(border_light);
            try_sdl!(context
                .canvas
                .draw_line(((i + 1) * w, 0), ((i + 1) * w, h + 1)));
            context.canvas.set_draw_color(border_dark);
            try_sdl!(context
                .canvas
                .draw_line(((i + 1) * w + 1, 0), ((i + 1) * w + 1, h + 1)));
        }
    }
}
