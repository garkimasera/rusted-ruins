#![warn(
    rust_2018_compatibility,
    rust_2018_idioms,
    future_incompatible,
    nonstandard_style
)]
#![allow(clippy::comparison_chain, clippy::type_complexity)]

extern crate rusted_ruins_common as common;

mod engine;
mod error;
mod message;
mod parse;
mod random;
mod rr;

pub use engine::ScriptEngine;
pub use error::Error;
pub use message::*;
