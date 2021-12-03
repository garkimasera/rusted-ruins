//! Helper crate for using thread local and fast random number generator
#![warn(
    rust_2018_compatibility,
    rust_2018_idioms,
    future_incompatible,
    nonstandard_style
)]
#![allow(clippy::redundant_closure)]

use rand::distributions::uniform::{SampleRange, SampleUniform};
pub use rand::seq::SliceRandom;
pub use rand::thread_rng;
pub use rand::Rng;
use rand::RngCore;
use rand::SeedableRng;
use rand_xorshift::XorShiftRng;
use std::cell::RefCell;

#[derive(Debug, Clone, Copy)]
pub struct GameRng;

thread_local!(static XORSHIFT_RNG: RefCell<XorShiftRng> = {
    let xorshift_rng = XorShiftRng::from_seed([0; 16]);
    RefCell::new(xorshift_rng)
});

impl RngCore for GameRng {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        XORSHIFT_RNG.with(|xorshift_rng| xorshift_rng.borrow_mut().next_u32())
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        XORSHIFT_RNG.with(|xorshift_rng| xorshift_rng.borrow_mut().next_u64())
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        XORSHIFT_RNG.with(|xorshift_rng| xorshift_rng.borrow_mut().fill_bytes(dest))
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), ::rand::Error> {
        XORSHIFT_RNG.with(|xorshift_rng| xorshift_rng.borrow_mut().try_fill_bytes(dest))
    }
}

pub fn get_rng() -> GameRng {
    GameRng
}

/// Reseed
pub fn reseed(fixed: bool) {
    let new_rng = if fixed {
        XorShiftRng::seed_from_u64(0x7275696e730a)
    } else {
        XorShiftRng::from_rng(thread_rng()).expect("reseed from thread rng failed")
    };
    XORSHIFT_RNG.with(|xorshift_rng| {
        xorshift_rng.replace(new_rng);
    })
}

pub fn next_u32() -> u32 {
    let mut rng = GameRng;
    rng.next_u32()
}

pub fn gen_range<T: SampleUniform, R: SampleRange<T>>(range: R) -> T {
    let mut rng = GameRng;
    rng.gen_range(range)
}

/// Calculate the sum of dices
/// n is the number of dice rolled, and x is the number of die faces
pub fn roll_dice<N1: Into<i32>, N2: Into<i32>>(n: N1, x: N2) -> i32 {
    let n = n.into();
    let x = x.into();
    let mut sum = 0;
    for _ in 0..n {
        sum += gen_range(1..(x + 1));
    }
    sum
}

/// Return bool from given probability
pub fn gen_bool(p: f32) -> bool {
    let mut rng = GameRng;
    rng.gen_bool(p.clamp(0.0, 1.0).into())
}

/// Choose a element from weight
pub fn choose<T, F>(values: &[T], mut weight: F) -> Option<(usize, &T)>
where
    F: FnMut(&T) -> f32,
{
    let sum: f32 = values.iter().map(|value| weight(value)).sum();
    if sum <= 0.0 {
        return None;
    }

    let mut rng = GameRng;
    let r = rng.gen_range(0.0..sum);
    let mut a = 0.0;

    let mut first_enable_value = None;
    for (i, value) in values.iter().enumerate() {
        let weight = weight(value);
        if weight > 0.0 && first_enable_value.is_none() {
            first_enable_value = Some((i, value));
        }
        a += weight;
        if r < a {
            return Some((i, value));
        }
    }
    first_enable_value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn average() {
        let mut sum = 0.0;
        const N: usize = 100000;

        for _ in 0..N {
            sum += gen_range(0.0..1.0);
        }

        let average = sum / N as f64;
        println!("average is {}", average);
    }
}
