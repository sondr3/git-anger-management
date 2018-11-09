#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]
//#![deny(missing_docs)]
//#![deny(missing_debug_implementations)]
#![forbid(unsafe_code)]
#![doc(html_root_url = "https://docs.rs/git-anger-management/0.4.0")]

//! # git-anger-management
//!
//! ## What
//!
//! Have you ever wondered how much you or your co-workers actually curse in your
//! commit messages? Worry no more, `git-anger-management` is here to help you.
//! Simply run it against your repository and it'll tell you who is the naughtiest
//! of them all.
//!
//! ## Why
//!
//! Some times the only way to vent at the ridiculous crap we make is to write
//! really angry commit messages, I do it all the time. And I wanted to know just
//! how angry I get.
//!
//! # Installation
//!
//! Make sure you have Rust installed (I recommend installing via
//! [rustup](https://rustup.rs/)), then run `cargo install git-anger-management`.
//! You can now check how naughty you are by running `git anger-management` in the
//! directory where you want to check naughtiness.
//!
//! Output should look something like this:
//!
//! ```sh
//! $ git anger-management
//! repo: (32/512) naughty commits/commits
//! Sondre Nilsen: (32/438) naughty commits/commits
//! {
//!     "goddamn": 1,
//!     "fuck": 12,
//!     "fucking": 13,
//!     "shit": 5,
//!     "tits": 1
//! }
//! ```
//!
//! You can also point it to other directories if you want to look somwhere else
//! but you're too lazy to actually `cd` into that directory:
//!
//! ```sh
//! $ git anger-management ../../other-repo/
//! other-repo: (3/56) naughty commits/commits
//! Sondre Nilsen: (3/56) naughty commits/commits
//! {
//!     "goddamn": 1,
//!     "fucking": 1,
//!     "fuck": 1
//! }
//! ```
//!
//! Or look at the help by running `git anger-management -h`.

#[macro_use]
extern crate lazy_static;
extern crate hashbrown;

mod author;
mod core;
mod repo;

pub use author::Author;
pub use core::{naughty_word, split_into_clean_words};
pub use repo::Repo;
