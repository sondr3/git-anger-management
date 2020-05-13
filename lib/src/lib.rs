//! # git-anger-library
//! [![pipeline](https://github.com/sondr3/git-anger-management/workflows/pipeline/badge.svg)](https://github.com/sondr3/git-anger-management/actions)
//! [![Crates.io](https://img.shields.io/crates/v/git-anger-library.svg)](https://crates.io/crates/git-anger-library)
//! [![Docs.rs](https://docs.rs/git-anger-library/badge.svg)](https://docs.rs/crate/git-anger-library/)
//!
//! ## What
//!
//! This is the main library that drives the
//! [`git-anger-management`](https://crates.io/crates/git-anger-management)
//! CLI-application, this is not really useful for anything besides that... but if
//! you for some reason want to, feel free!
//!
//! ## Features:
//!
//! **Note:** None of these features are enabled by default, so you have to opt into
//! it like so:
//!
//! ```toml
//! [dependencies]
//! git-anger-library = { version = "0.8.0", features=["table", "json"] }
//! ```
//!
//! - `json`: Enables Serde serialization of the processed data using with the
//! `print_json()` method.
//! - `table`: Enables pretty printing of the processed data using TabWriter with
//! the function `print_table()`.
//!
//! ## License
//!
//! GPLv3 or later.
//!
/// A git author
pub mod author;
/// Core algorithms and functionality
pub mod core;
/// A simplified representation of a git repository
pub mod repo;
