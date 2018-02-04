//! Helper crate for using thread local and fast random number generator

extern crate rand;

use std::cell::RefCell;
use rand::{SeedableRng, XorShiftRng};
use rand::distributions::range::SampleRange;
pub use rand::Rng;

#[derive(Debug, Clone, Copy)]
pub struct GameRng;

thread_local!(static XORSHIFT_RNG: RefCell<XorShiftRng> = {
    let xorshift_rng = XorShiftRng::new_unseeded();
    RefCell::new(xorshift_rng)
});

impl Rng for GameRng {
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
}

pub fn get_rng() -> GameRng {
    GameRng
}

/// Reseed by 32bit integer
pub fn reseed(s: u32) {
    const A: u32 = 1103515245;
    const B: u32 = 12345;

    let a = s.wrapping_mul(A).wrapping_add(B);
    let b = a.wrapping_mul(A).wrapping_add(B);
    let c = b.wrapping_mul(A).wrapping_add(B);
    let d = c.wrapping_mul(A).wrapping_add(B);

    println!("{:x}\n{:x}\n{:x}\n{:x}", a, b, c, d);
    XORSHIFT_RNG.with(|xorshift_rng| {
        xorshift_rng.borrow_mut().reseed([a, b, c, d]);
    })
}

pub fn next_u32() -> u32 {
    let mut rng = GameRng;
    rng.next_u32()
}

pub fn gen_range<T: PartialOrd + SampleRange>(low: T, high: T) -> T {
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

