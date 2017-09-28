
//! This crate provide functions for 2d array and vector

extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::ops::{Index, IndexMut, Add, Sub, Mul, Range};
use std::fmt;

const OUT_OF_BOUNDS_ERR_MSG: &'static str = "Array2d: index out of bounds";

/// Represents coordinates on a 2D array
#[derive(PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
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

/// Base type for 2D map
#[derive(Clone, Serialize, Deserialize)]
pub struct Array2d<T> {
    w: u32,
    h: u32,
    v: Vec<T>,
}

impl<T> Array2d<T> {
    pub fn from_fn<F>(w: u32, h: u32, f: F) -> Array2d<T> where F: FnMut((u32, u32)) -> T {
        let len = (w * h) as usize;
        let mut v = Vec::with_capacity(len);
        let mut f = f;

        for y in 0..h {
            for x in 0..w {
                v.push(f((x, y)));
            }
        }

        assert!(v.len() == len);
        
        Array2d {
            w: w, h: h, v: v,
        }
    }

    pub fn size(&self) -> (u32, u32) {
        (self.w, self.h)
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
}

impl<T> Array2d<T> where T: Clone {
    pub fn new(w: u32, h: u32, v: T) -> Array2d<T> {
        let len = (w * h) as usize;
        let v = vec![v; len];

        Array2d {
            w: w, h: h, v: v,
        }
    }

    pub fn swap<P: Into<Vec2d>>(&mut self, a: P, b: P) {
        let a = a.into();
        let b = b.into();
        debug_assert!(0 <= a.0 && a.0 < self.w as i32, OUT_OF_BOUNDS_ERR_MSG);
        debug_assert!(0 <= a.1 && a.1 < self.h as i32, OUT_OF_BOUNDS_ERR_MSG);
        debug_assert!(0 <= b.0 && b.0 < self.w as i32, OUT_OF_BOUNDS_ERR_MSG);
        debug_assert!(0 <= b.1 && b.1 < self.h as i32, OUT_OF_BOUNDS_ERR_MSG);

        self.v.swap((a.1 * self.w as i32 + a.0) as usize, (b.1 * self.w as i32 + b.0) as usize);
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
        debug_assert!(0 <= index.0 && index.0 < self.w as i32, OUT_OF_BOUNDS_ERR_MSG);
        debug_assert!(0 <= index.1 && index.1 < self.h as i32, OUT_OF_BOUNDS_ERR_MSG);
        
        &self.v[(index.1 as u32 * self.w + index.0 as u32) as usize]
    }
}

impl<T> IndexMut<(i32, i32)> for Array2d<T> {
    #[inline]
    fn index_mut(&mut self, index: (i32, i32)) -> &mut T {
        debug_assert!(0 <= index.0 && index.0 < self.w as i32, OUT_OF_BOUNDS_ERR_MSG);
        debug_assert!(0 <= index.1 && index.1 < self.h as i32, OUT_OF_BOUNDS_ERR_MSG);
        
        &mut self.v[(index.1 as u32 * self.w + index.0 as u32) as usize]
    }
}

impl<T> Index<Vec2d> for Array2d<T> {
    type Output = T;
    #[inline]
    fn index(&self, index: Vec2d) -> &T {
        debug_assert!(0 <= index.0 && index.0 < self.w as i32, OUT_OF_BOUNDS_ERR_MSG);
        debug_assert!(0 <= index.0 && index.1 < self.h as i32, OUT_OF_BOUNDS_ERR_MSG);
        
        &self.v[(index.1 as usize) * self.w as usize + index.0 as usize]
    }
}

impl<T> IndexMut<Vec2d> for Array2d<T> {
    #[inline]
    fn index_mut(&mut self, index: Vec2d) -> &mut T {
        debug_assert!(0 <= index.0 && index.0 < self.w as i32, OUT_OF_BOUNDS_ERR_MSG);
        debug_assert!(0 <= index.1 && index.1 < self.h as i32, OUT_OF_BOUNDS_ERR_MSG);
        
        &mut self.v[(index.1 as usize) * self.w as usize + index.0 as usize]
    }
}

impl<T> fmt::Debug for Array2d<T> where T: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        for y in 0..self.h {
            write!(f, "[")?;
            for x in 0..self.w {
                if x == self.w - 1 {
                    write!(f, "{:?}", self[(x, y)])?;
                }else{
                    write!(f, "{:?}, ", self[(x, y)])?;
                }
            }
            if y == self.h - 1 {
                write!(f, "]")?;
            }else{
                write!(f, "], ")?;
            }
        }
        write!(f, "]")?;
        Ok(())
    }
}

#[derive(Clone)]
pub struct Array2dIter<'a, T> where T: 'a {
    array: &'a Array2d<T>,
    rectiter: RectIter,
}

impl<'a, T> Iterator for Array2dIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        if let Some(a) = self.rectiter.next() {
            Some(&self.array[a])
        }else{
            None
        }
    }
}

#[derive(Clone)]
pub struct Array2dIterWithIdx<'a, T> where T: 'a {
    array: &'a Array2d<T>,
    rectiter: RectIter,
}

impl<'a, T> Iterator for Array2dIterWithIdx<'a, T> {
    type Item = (Vec2d, &'a T);
    fn next(&mut self) -> Option<(Vec2d, &'a T)> {
        if let Some(a) = self.rectiter.next() {
            Some((a, &self.array[a]))
        }else{
            None
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct RectIter {
    top_left:     Vec2d,
    right_bottom: Vec2d,
    value:        Vec2d,
}

impl RectIter {
    /// Create rectangle iterator. It includes right_bottom
    pub fn new<T: Into<Vec2d>>(top_left: T, right_bottom: T) -> RectIter {
        let top_left = top_left.into();
        let right_bottom = right_bottom.into();

        RectIter {
            top_left:     top_left,
            right_bottom: right_bottom,
            value:        Vec2d::new(top_left.0 - 1, top_left.1),
        }
    }

    /// Iterator for one tile
    pub fn one<T: Into<Vec2d>>(t: T) -> RectIter {
        let t = t.into();
        RectIter {
            top_left:     t,
            right_bottom: t,
            value:        Vec2d::new(t.0 - 1, t.1),
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
        }else{
            self.value.0 += 1;
        }
        Some(self.value)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum HDirection {
    None, Left, Right,
}

impl HDirection {
    #[inline]
    pub fn as_int(&self) -> i32 {
        match *self {
            HDirection::None => 0, HDirection::Left => -1, HDirection::Right => 1,
        }
    }

    #[inline]
    pub fn as_vec(&self) -> Vec2d {
        Vec2d::new(self.as_int(), 0)
    }
}

impl Default for VDirection {
    fn default() -> VDirection {
        VDirection::None
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum VDirection {
    None, Up, Down,
}

impl VDirection {
    #[inline]
    pub fn as_int(&self) -> i32 {
        match *self {
            VDirection::None => 0, VDirection::Up => -1, VDirection::Down => 1,
        }
    }

    #[inline]
    pub fn as_vec(&self) -> Vec2d {
        Vec2d::new(0, self.as_int())
    }
}

impl Default for HDirection {
    fn default() -> HDirection {
        HDirection::None
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Direction {
    pub hdir: HDirection,
    pub vdir: VDirection,
}

impl Direction {
    pub fn new(hdir: HDirection, vdir: VDirection) -> Direction {
        Direction {
            hdir: hdir, vdir: vdir,
        }
    }
    #[inline]
    pub fn as_vec(&self) -> Vec2d {
        self.hdir.as_vec() + self.vdir.as_vec()
    }
}
