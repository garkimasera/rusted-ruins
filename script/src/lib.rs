#![warn(
    rust_2018_compatibility,
    rust_2018_idioms,
    future_incompatible,
    nonstandard_style
)]

#[macro_use]
extern crate log;
extern crate rusted_ruins_common as common;

mod engine;
mod error;
#[macro_use]
mod gamedata;
mod random;
mod rr;
mod run;
mod script_yield;

pub use engine::{enter, ScriptEngine};
pub use error::Error;
pub use gamedata::{set_game_methods, GameMethods};
pub use script_yield::*;
