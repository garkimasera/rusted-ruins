
use super::commonuse::*;

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
        group_window.switch(init_win, game);
        group_window
    }

    pub fn switch(&mut self, i_win: usize, game: &Game) {
        assert!(i_win < self.size);
        self.current_window = i_win;
        if self.members[i_win].is_none() {
            self.members[i_win] = Some((self.creator)(game, i_win as usize));
        }
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

