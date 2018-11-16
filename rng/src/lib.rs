//! Helper crate for using thread local and fast random number generator

extern crate rand;
extern crate rand_xorshift;

use std::cell::RefCell;
use rand::SeedableRng;
use rand::distributions::uniform::{SampleUniform, SampleBorrow};
use rand::{RngCore, thread_rng};
use rand_xorshift::XorShiftRng;
pub use rand::Rng;
pub use rand::seq::SliceRandom;

#[derive(Debug, Clone, Copy)]
pub struct GameRng;

thread_local!(static XORSHIFT_RNG: RefCell<XorShiftRng> = {
    let xorshift_rng = XorShiftRng::from_seed([0; 16]);
    RefCell::new(xorshift_rng)
});

impl RngCore for GameRng {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        XORSHIFT_RNG.with(|xorshift_rng| {
            xorshift_rng.borrow_mut().next_u32()
        })
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        XORSHIFT_RNG.with(|xorshift_rng| {
            xorshift_rng.borrow_mut().next_u64()
        })
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        XORSHIFT_RNG.with(|xorshift_rng| {
            xorshift_rng.borrow_mut().fill_bytes(dest)
        })
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), ::rand::Error> {
        XORSHIFT_RNG.with(|xorshift_rng| {
            xorshift_rng.borrow_mut().try_fill_bytes(dest)
        })
    }
}

pub fn get_rng() -> GameRng {
    GameRng
}

/// Reseed
pub fn reseed() {
    XORSHIFT_RNG.with(|xorshift_rng| {
        xorshift_rng.replace(XorShiftRng::from_rng(thread_rng()).expect("reseed from thread rng failed"));
    })
}

pub fn next_u32() -> u32 {
    let mut rng = GameRng;
    rng.next_u32()
}

pub fn gen_range<T: SampleUniform, B: SampleBorrow<T> + Sized>(low: B, high: B) -> T {
    let mut rng = GameRng;
    rng.gen_range(low, high)
}

/// Calculate the sum of dices
/// n is the number of dice rolled, and x is the number of die faces
pub fn dice(n: i32, x: i32) -> i32 {
    let mut sum = 0;
    for _ in 0..n {
        sum += gen_range(1, x + 1);
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn average() {
        let mut sum = 0.0;
        const N: usize = 100000;
    
        for _ in 0..N {
            sum += gen_range(0.0, 1.0);
        }

        let average = sum / N as f64;
        println!("average is {}", average);
    }    
}

