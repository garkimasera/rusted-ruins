
use sdl2::rect::Rect;
use super::commonuse::*;
use super::widget::*;
use super::winpos::WindowPos;

/// GroupWindow manages multiple windows.
/// Player can switches displaying windows.
pub struct GroupWindow {
    size: usize,
    current_window: usize,
    members: Vec<Option<Box<DialogWindow>>>,
    creator: fn(&Game, usize) -> Box<DialogWindow>,
}

impl GroupWindow {
    pub fn new(size: usize, init_win: usize, game: &Game,
               creator: fn(&Game, usize) -> Box<DialogWindow>) -> GroupWindow {
        
        assert!(init_win < size);
        let members: Vec<Option<Box<DialogWindow>>> = (0..size).into_iter().map(|_| None).collect();
        let mut group_window = GroupWindow {
            size: size,
            current_window: init_win,
            members: members,
            creator: creator,
        };
        group_window.switch(game, init_win);
        group_window
    }

    pub fn switch(&mut self, game: &Game, i_win: usize) {
        assert!(i_win < self.size);
        self.current_window = i_win;
        if self.members[i_win].is_none() {
            self.members[i_win] = Some((self.creator)(game, i_win as usize));
        }
    }
}

impl Window for GroupWindow {
    fn draw(
        &mut self, canvas: &mut WindowCanvas, game: &Game, sv: &mut SdlValues,
        anim: Option<(&Animation, u32)>) {

        if let Some(ref mut member) = self.members[self.current_window] {
            member.draw(canvas, game, sv, anim);
        }
    }
}

impl DialogWindow for GroupWindow {
    fn process_command(&mut self, command: &Command, pa: &mut DoPlayerAction) -> DialogResult {
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

