use crate::{Array2d, Direction, Vec2d};
use std::collections::VecDeque;

pub struct PathFinding<F> {
    w: u32,
    h: u32,
    max_step: u32,
    map: F,
}

impl<F: Fn(Vec2d) -> bool> PathFinding<F> {
    pub fn new(w: u32, h: u32, max_step: u32, map: F) -> Self {
        PathFinding {
            w,
            h,
            max_step,
            map,
        }
    }

    pub fn is_inside(&self, pos: Vec2d) -> bool {
        pos.0 >= 0 && pos.1 >= 0 && pos.0 < self.w as i32 && pos.1 < self.h as i32
    }

    /// Calculate route by breadth-first search.
    pub fn route(&self, start: Vec2d, end: Vec2d) -> Option<Vec<Vec2d>> {
        let mut q = VecDeque::new();
        let mut check_map: Array2d<Option<Direction>> = Array2d::new(self.w, self.h, None);

        q.push_front((0, Direction::NONE, start));

        while let Some((step, dir, pos)) = q.pop_back() {
            if check_map[pos].is_some() {
                continue;
            }
            if step >= self.max_step {
                return None;
            }

            check_map[pos] = Some(dir);

            if pos == end {
                break;
            }

            if !(self.map)(pos) {
                continue;
            }

            let next_step = step + 1;
            for &dir in Direction::EIGHT_DIRS.iter() {
                let next_pos = pos + dir.as_vec();
                if self.is_inside(next_pos) && check_map[next_pos].is_none() {
                    q.push_front((next_step, dir, next_pos));
                }
            }
        }

        let mut route = Vec::new();
        let mut pos = end;

        loop {
            route.push(pos);
            if pos == start {
                break;
            }
            let dir = check_map[pos]?;
            pos = pos - dir.as_vec();
            assert!(route.len() < self.max_step as usize);
        }

        route.reverse();
        Some(route)
    }
}

#[cfg(test)]
mod pathfinding_test {
    use super::*;

    #[test]
    fn pathfinding_test() {
        let map = [[1, 1, 1, 1], [1, 0, 1, 1], [1, 0, 0, 1], [1, 1, 0, 1]];

        let route = PathFinding::new(4, 4, 100, |pos| map[pos.1 as usize][pos.0 as usize] != 0)
            .route(Vec2d(0, 0), Vec2d(3, 3));

        eprintln!("{:?}", route.unwrap());

        let map = [[1, 1, 1], [0, 0, 0], [1, 1, 1]];

        let route = PathFinding::new(3, 3, 100, |pos| map[pos.1 as usize][pos.0 as usize] != 0)
            .route(Vec2d(0, 0), Vec2d(2, 2));

        assert!(route.is_none());
    }
}
