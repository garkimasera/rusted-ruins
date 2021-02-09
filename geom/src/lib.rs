//! This crate provide functions for 2d array and vector

extern crate serde;
#[macro_use]
extern crate serde_derive;

mod shape;
pub use shape::*;

use std::fmt;
use std::ops::{Add, Index, IndexMut, Mul, Range, Sub};
use thiserror::Error;

const OUT_OF_BOUNDS_ERR_MSG: &'static str = "Array2d: index out of bounds";

/// Represents coordinates on a 2D array
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Vec2d(pub i32, pub i32);

impl Vec2d {
    pub fn new(x: i32, y: i32) -> Vec2d {
        Vec2d(x, y)
    }

    /// Iterate from (0, 0) to (self.0 - 1, self.1 - 1)
    pub fn iter_from_zero(self) -> RectIter {
        assert!(self.0 > 1);
        assert!(self.1 > 1);
        RectIter::new((0, 0), (self.0 - 1, self.1 - 1))
    }

    /// Calculate Manhattan distance between two points
    pub fn mdistance(self, another: Vec2d) -> i32 {
        (self.0 - another.0).abs() + (self.1 - another.1).abs()
    }

    /// Calculate square of distance between two points
    pub fn distance2(self, another: Vec2d) -> f32 {
        let x = (self.0 - another.0) as f32;
        let y = (self.1 - another.1) as f32;
        x * x + y * y
    }

    /// Gien tile is adjacent or not
    pub fn is_adjacent(self, another: Vec2d) -> bool {
        if self == another {
            false
        } else {
            let diff_x = (self.0 - another.0).abs();
            let diff_y = (self.1 - another.1).abs();
            (diff_x == 0 || diff_x == 1) && (diff_y == 0 || diff_y == 1)
        }
    }
}

impl From<(i32, i32)> for Vec2d {
    #[inline]
    fn from(p: (i32, i32)) -> Vec2d {
        Vec2d(p.0, p.1)
    }
}

impl From<(u32, u32)> for Vec2d {
    #[inline]
    fn from(p: (u32, u32)) -> Vec2d {
        Vec2d(p.0 as i32, p.1 as i32)
    }
}

impl Add for Vec2d {
    type Output = Vec2d;
    #[inline]
    fn add(self, other: Vec2d) -> Vec2d {
        Vec2d(self.0 + other.0, self.1 + other.1)
    }
}

impl Add<(i32, i32)> for Vec2d {
    type Output = Vec2d;
    #[inline]
    fn add(self, other: (i32, i32)) -> Vec2d {
        Vec2d(self.0 + other.0, self.1 + other.1)
    }
}

impl Sub for Vec2d {
    type Output = Vec2d;
    #[inline]
    fn sub(self, other: Vec2d) -> Vec2d {
        Vec2d(self.0 - other.0, self.1 - other.1)
    }
}

impl Sub<(i32, i32)> for Vec2d {
    type Output = Vec2d;
    #[inline]
    fn sub(self, other: (i32, i32)) -> Vec2d {
        Vec2d(self.0 - other.0, self.1 - other.1)
    }
}

impl Mul<i32> for Vec2d {
    type Output = Vec2d;
    #[inline]
    fn mul(self, other: i32) -> Vec2d {
        Vec2d(self.0 * other, self.1 * other)
    }
}

impl PartialEq<(i32, i32)> for Vec2d {
    fn eq(&self, other: &(i32, i32)) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl fmt::Display for Vec2d {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl std::str::FromStr for Vec2d {
    type Err = Vec2dParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split(',');
        let x: i32 = if let Some(s0) = s.next() {
            s0.parse()?
        } else {
            return Err(Vec2dParseError::InvalidInput);
        };
        let y: i32 = if let Some(s1) = s.next() {
            s1.parse()?
        } else {
            return Err(Vec2dParseError::InvalidInput);
        };

        if s.next().is_some() {
            return Err(Vec2dParseError::InvalidInput);
        }

        Ok(Vec2d(x, y))
    }
}

#[derive(Error, Debug)]
pub enum Vec2dParseError {
    #[error("{0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("")]
    InvalidInput,
}

/// Base type for 2D map
#[derive(Clone, Serialize, Deserialize)]
pub struct Array2d<T> {
    w: u32,
    h: u32,
    v: Vec<T>,
}

impl<T> Array2d<T> {
    pub fn from_fn<F>(w: u32, h: u32, f: F) -> Array2d<T>
    where
        F: FnMut((u32, u32)) -> T,
    {
        let len = (w * h) as usize;
        let mut v = Vec::with_capacity(len);
        let mut f = f;

        for y in 0..h {
            for x in 0..w {
                v.push(f((x, y)));
            }
        }

        assert!(v.len() == len);

        Array2d { w, h, v }
    }

    pub fn size(&self) -> (u32, u32) {
        (self.w, self.h)
    }

    /// If pos is out of range, returns None.
    pub fn get<P: Into<Vec2d>>(&self, p: P) -> Option<&T> {
        let p = p.into();
        if self.in_range(p) {
            Some(&self[p])
        } else {
            None
        }
    }

    pub fn iter<'a>(&'a self) -> Array2dIter<'a, T> {
        Array2dIter {
            array: &self,
            rectiter: RectIter::new((0, 0), (self.w - 1, self.h - 1)),
        }
    }

    pub fn iter_with_idx<'a>(&'a self) -> Array2dIterWithIdx<'a, T> {
        Array2dIterWithIdx {
            array: &self,
            rectiter: RectIter::new((0, 0), (self.w - 1, self.h - 1)),
        }
    }

    pub fn iter_idx(&self) -> RectIter {
        RectIter::new((0, 0), (self.w - 1, self.h - 1))
    }

    pub fn in_range<P: Into<Vec2d>>(&self, p: P) -> bool {
        let p = p.into();
        p.0 >= 0 && p.1 >= 0 && (p.0 as u32) < self.w && (p.1 as u32) < self.h
    }
}

impl<T> Array2d<T>
where
    T: Clone,
{
    pub fn new(w: u32, h: u32, v: T) -> Array2d<T> {
        let len = (w * h) as usize;
        let v = vec![v; len];

        Array2d { w, h, v }
    }

    pub fn swap<P: Into<Vec2d>>(&mut self, a: P, b: P) {
        let a = a.into();
        let b = b.into();
        debug_assert!(0 <= a.0 && a.0 < self.w as i32, OUT_OF_BOUNDS_ERR_MSG);
        debug_assert!(0 <= a.1 && a.1 < self.h as i32, OUT_OF_BOUNDS_ERR_MSG);
        debug_assert!(0 <= b.0 && b.0 < self.w as i32, OUT_OF_BOUNDS_ERR_MSG);
        debug_assert!(0 <= b.1 && b.1 < self.h as i32, OUT_OF_BOUNDS_ERR_MSG);

        self.v.swap(
            (a.1 * self.w as i32 + a.0) as usize,
            (b.1 * self.w as i32 + b.0) as usize,
        );
    }

    /// Clip and create new Array2d
    /// Outside of original array is filled by given default value
    pub fn clip_with_default<P: Into<Vec2d>>(
        &self,
        topleft: P,
        bottomright: P,
        default: T,
    ) -> Array2d<T> {
        let topleft = topleft.into();
        let bottomright = bottomright.into();

        let w = bottomright.0 - topleft.0;
        let h = bottomright.1 - topleft.1;
        assert!(w >= 0 && h >= 0);
        let mut a = Array2d::new(w as u32, h as u32, default);

        for j in 0..h {
            for i in 0..w {
                let orig = (i + topleft.0, j + topleft.1);
                if self.in_range(orig) {
                    a[(i, j)] = self[orig].clone();
                }
            }
        }
        a
    }
}

impl<T> Index<(u32, u32)> for Array2d<T> {
    type Output = T;
    #[inline]
    fn index(&self, index: (u32, u32)) -> &T {
        debug_assert!(index.0 < self.w, OUT_OF_BOUNDS_ERR_MSG);
        debug_assert!(index.1 < self.h, OUT_OF_BOUNDS_ERR_MSG);

        &self.v[(index.1 * self.w + index.0) as usize]
    }
}

impl<T> IndexMut<(u32, u32)> for Array2d<T> {
    #[inline]
    fn index_mut(&mut self, index: (u32, u32)) -> &mut T {
        debug_assert!(index.0 < self.w, OUT_OF_BOUNDS_ERR_MSG);
        debug_assert!(index.1 < self.h, OUT_OF_BOUNDS_ERR_MSG);

        &mut self.v[(index.1 * self.w + index.0) as usize]
    }
}

impl<T> Index<(i32, i32)> for Array2d<T> {
    type Output = T;
    #[inline]
    fn index(&self, index: (i32, i32)) -> &T {
        debug_assert!(
            0 <= index.0 && index.0 < self.w as i32,
            OUT_OF_BOUNDS_ERR_MSG
        );
        debug_assert!(
            0 <= index.1 && index.1 < self.h as i32,
            OUT_OF_BOUNDS_ERR_MSG
        );

        &self.v[(index.1 as u32 * self.w + index.0 as u32) as usize]
    }
}

impl<T> IndexMut<(i32, i32)> for Array2d<T> {
    #[inline]
    fn index_mut(&mut self, index: (i32, i32)) -> &mut T {
        debug_assert!(
            0 <= index.0 && index.0 < self.w as i32,
            OUT_OF_BOUNDS_ERR_MSG
        );
        debug_assert!(
            0 <= index.1 && index.1 < self.h as i32,
            OUT_OF_BOUNDS_ERR_MSG
        );

        &mut self.v[(index.1 as u32 * self.w + index.0 as u32) as usize]
    }
}

impl<T> Index<Vec2d> for Array2d<T> {
    type Output = T;
    #[inline]
    fn index(&self, index: Vec2d) -> &T {
        debug_assert!(
            0 <= index.0 && index.0 < self.w as i32,
            OUT_OF_BOUNDS_ERR_MSG
        );
        debug_assert!(
            0 <= index.1 && index.1 < self.h as i32,
            OUT_OF_BOUNDS_ERR_MSG
        );

        &self.v[(index.1 as usize) * self.w as usize + index.0 as usize]
    }
}

impl<T> IndexMut<Vec2d> for Array2d<T> {
    #[inline]
    fn index_mut(&mut self, index: Vec2d) -> &mut T {
        debug_assert!(
            0 <= index.0 && index.0 < self.w as i32,
            OUT_OF_BOUNDS_ERR_MSG
        );
        debug_assert!(
            0 <= index.1 && index.1 < self.h as i32,
            OUT_OF_BOUNDS_ERR_MSG
        );

        &mut self.v[(index.1 as usize) * self.w as usize + index.0 as usize]
    }
}

impl<T> fmt::Debug for Array2d<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        for y in 0..self.h {
            write!(f, "[")?;
            for x in 0..self.w {
                if x == self.w - 1 {
                    write!(f, "{:?}", self[(x, y)])?;
                } else {
                    write!(f, "{:?}, ", self[(x, y)])?;
                }
            }
            if y == self.h - 1 {
                write!(f, "]")?;
            } else {
                write!(f, "], ")?;
            }
        }
        write!(f, "]")?;
        Ok(())
    }
}

#[derive(Clone)]
pub struct Array2dIter<'a, T>
where
    T: 'a,
{
    array: &'a Array2d<T>,
    rectiter: RectIter,
}

impl<'a, T> Iterator for Array2dIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        if let Some(a) = self.rectiter.next() {
            Some(&self.array[a])
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct Array2dIterWithIdx<'a, T>
where
    T: 'a,
{
    array: &'a Array2d<T>,
    rectiter: RectIter,
}

impl<'a, T> Iterator for Array2dIterWithIdx<'a, T> {
    type Item = (Vec2d, &'a T);
    fn next(&mut self) -> Option<(Vec2d, &'a T)> {
        if let Some(a) = self.rectiter.next() {
            Some((a, &self.array[a]))
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct RectIter {
    top_left: Vec2d,
    right_bottom: Vec2d,
    value: Vec2d,
}

impl RectIter {
    /// Create rectangle iterator. It includes right_bottom
    pub fn new<T: Into<Vec2d>>(top_left: T, right_bottom: T) -> RectIter {
        let top_left = top_left.into();
        let right_bottom = right_bottom.into();

        RectIter {
            top_left,
            right_bottom,
            value: Vec2d(top_left.0 - 1, top_left.1),
        }
    }

    /// Iterator for one tile
    pub fn one<T: Into<Vec2d>>(t: T) -> RectIter {
        let t = t.into();
        RectIter {
            top_left: t,
            right_bottom: t,
            value: Vec2d(t.0 - 1, t.1),
        }
    }

    #[inline]
    pub fn iter0(&self) -> Range<i32> {
        self.top_left.0..(self.right_bottom.0 + 1)
    }

    #[inline]
    pub fn iter1(&self) -> Range<i32> {
        self.top_left.1..(self.right_bottom.1 + 1)
    }
}

impl Iterator for RectIter {
    type Item = Vec2d;
    fn next(&mut self) -> Option<Vec2d> {
        if self.value.0 == self.right_bottom.0 {
            if self.value.1 == self.right_bottom.1 {
                return None;
            }
            self.value.0 = self.top_left.0;
            self.value.1 += 1;
        } else {
            self.value.0 += 1;
        }
        Some(self.value)
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct LineIter {
    start: Vec2d,
    end: Vec2d,
    p: Vec2d,
    dx: i32,
    dy: i32,
    dir_x: i32,
    dir_y: i32,
    err: i32,
    finished: bool,
}

impl LineIter {
    pub fn new<V: Into<Vec2d>>(start: V, end: V) -> LineIter {
        let start = start.into();
        let end = end.into();

        let dx = (end.0 - start.0).abs();
        let dy = (end.1 - start.1).abs();
        let dir_x = if start.0 < end.0 { 1 } else { -1 };
        let dir_y = if start.1 < end.1 { 1 } else { -1 };
        let err = if dx > dy { dx } else { -dy } / 2;
        let p = start;

        LineIter {
            start,
            end,
            p,
            dx,
            dy,
            dir_x,
            dir_y,
            err,
            finished: false,
        }
    }
}

impl Iterator for LineIter {
    type Item = Vec2d;
    fn next(&mut self) -> Option<Vec2d> {
        if self.finished {
            return None;
        }

        let returnval = self.p;

        if self.p == self.end {
            self.finished = true;
            return Some(self.p);
        }

        let e = self.err;
        if e > -self.dx {
            self.err -= self.dy;
            self.p.0 += self.dir_x;
        }
        if e < self.dy {
            self.err += self.dx;
            self.p.1 += self.dir_y;
        }
        Some(returnval)
    }
}

/// Iterate around center, and the range is manhattan distance
#[derive(Clone, Copy, PartialEq)]
pub struct MDistRangeIter {
    center: Vec2d,
    r: i32,
    rectiter: RectIter,
}

impl MDistRangeIter {
    pub fn new<V: Into<Vec2d>>(center: V, r: i32) -> MDistRangeIter {
        assert!(r >= 0);
        let center = center.into();

        let top_left = Vec2d(center.0 - r, center.1 - r);
        let right_bottom = Vec2d(center.0 + r, center.1 + r);

        MDistRangeIter {
            center,
            r,
            rectiter: RectIter::new(top_left, right_bottom),
        }
    }
}

impl Iterator for MDistRangeIter {
    type Item = (i32, Vec2d);
    fn next(&mut self) -> Option<(i32, Vec2d)> {
        while let Some(p) = self.rectiter.next() {
            let mdistance = self.center.mdistance(p);
            if self.r >= mdistance {
                return Some((mdistance, p));
            }
        }
        None
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SpiralIter {
    len_twice: i32,
    i: i32,
    dir: i32,
    value: Vec2d,
}

impl SpiralIter {
    pub fn new<V: Into<Vec2d>>(center: V) -> Self {
        Self {
            len_twice: 0,
            i: 0,
            dir: 0,
            value: center.into(),
        }
    }
}

impl Iterator for SpiralIter {
    type Item = Vec2d;
    fn next(&mut self) -> Option<Vec2d> {
        const DIRS: &[Vec2d] = &[Vec2d(0, 1), Vec2d(1, 0), Vec2d(0, -1), Vec2d(-1, 0)];
        let value = self.value;
        let len = self.len_twice / 2 + 1;

        self.i += 1;
        self.value = self.value + DIRS[self.dir as usize];

        if self.i >= len {
            if self.dir == DIRS.len() as i32 - 1 {
                self.dir = 0;
            } else {
                self.dir += 1;
            }
            self.len_twice += 1;
            self.i = 0;
        }

        Some(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let v: Vec<_> = SpiralIter::new((0, 0)).take(9).collect();
        assert_eq!(
            v,
            &[
                Vec2d(0, 0),
                Vec2d(0, 1),
                Vec2d(1, 1),
                Vec2d(1, 0),
                Vec2d(1, -1),
                Vec2d(0, -1),
                Vec2d(-1, -1),
                Vec2d(-1, 0),
                Vec2d(-1, 1),
            ],
        );
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum HDirection {
    None,
    Left,
    Right,
}

impl HDirection {
    #[inline]
    pub fn as_int(&self) -> i32 {
        match *self {
            HDirection::None => 0,
            HDirection::Left => -1,
            HDirection::Right => 1,
        }
    }

    #[inline]
    pub fn as_vec(&self) -> Vec2d {
        Vec2d(self.as_int(), 0)
    }

    #[inline]
    pub fn is_none(&self) -> bool {
        *self == HDirection::None
    }
}

impl Default for VDirection {
    fn default() -> VDirection {
        VDirection::None
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum VDirection {
    None,
    Up,
    Down,
}

impl VDirection {
    #[inline]
    pub fn as_int(&self) -> i32 {
        match *self {
            VDirection::None => 0,
            VDirection::Up => -1,
            VDirection::Down => 1,
        }
    }

    #[inline]
    pub fn as_vec(&self) -> Vec2d {
        Vec2d(0, self.as_int())
    }

    #[inline]
    pub fn is_none(&self) -> bool {
        *self == VDirection::None
    }
}

impl Default for HDirection {
    fn default() -> HDirection {
        HDirection::None
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, Serialize, Deserialize)]
pub struct Direction {
    pub hdir: HDirection,
    pub vdir: VDirection,
}

impl Direction {
    pub fn new(hdir: HDirection, vdir: VDirection) -> Direction {
        Direction { hdir, vdir }
    }

    pub fn none() -> Direction {
        Direction {
            hdir: HDirection::None,
            vdir: VDirection::None,
        }
    }
    #[inline]
    pub fn as_vec(&self) -> Vec2d {
        self.hdir.as_vec() + self.vdir.as_vec()
    }
    #[inline]
    pub fn is_none(&self) -> bool {
        self.hdir.is_none() && self.vdir.is_none()
    }

    pub const N: Direction = Direction {
        hdir: HDirection::None,
        vdir: VDirection::Up,
    };
    pub const S: Direction = Direction {
        hdir: HDirection::None,
        vdir: VDirection::Down,
    };
    pub const E: Direction = Direction {
        hdir: HDirection::Right,
        vdir: VDirection::None,
    };
    pub const W: Direction = Direction {
        hdir: HDirection::Left,
        vdir: VDirection::None,
    };
    pub const NE: Direction = Direction {
        hdir: HDirection::Right,
        vdir: VDirection::Up,
    };
    pub const NW: Direction = Direction {
        hdir: HDirection::Left,
        vdir: VDirection::Up,
    };
    pub const SE: Direction = Direction {
        hdir: HDirection::Right,
        vdir: VDirection::Down,
    };
    pub const SW: Direction = Direction {
        hdir: HDirection::Left,
        vdir: VDirection::Down,
    };
    pub const NONE: Direction = Direction {
        hdir: HDirection::None,
        vdir: VDirection::None,
    };

    pub const EIGHT_DIRS: [Direction; 8] = [
        Self::E,
        Self::SE,
        Self::S,
        Self::SW,
        Self::W,
        Self::NW,
        Self::N,
        Self::NE,
    ];
}

/// Direction from p1 to p2
pub fn dir_by_2pos(p1: Vec2d, p2: Vec2d) -> Direction {
    let dx = p2.0 - p1.0;
    let dy = p2.1 - p1.1;

    Direction::new(
        if dx < 0 {
            HDirection::Left
        } else if dx > 0 {
            HDirection::Right
        } else {
            HDirection::None
        },
        if dy < 0 {
            VDirection::Up
        } else if dy > 0 {
            VDirection::Down
        } else {
            VDirection::None
        },
    )
}

#[test]
fn dir_2pos_test() {
    assert_eq!(dir_by_2pos(Vec2d(1, 1), Vec2d(2, 2)), Direction::SE);
}
