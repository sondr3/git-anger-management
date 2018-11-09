#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]
#![forbid(unsafe_code)]
#[macro_use]
extern crate lazy_static;
extern crate hashbrown;

mod author;
mod core;
mod repo;

pub use author::Author;
pub use core::{naughty_word, split_into_clean_words};
pub use repo::Repo;
