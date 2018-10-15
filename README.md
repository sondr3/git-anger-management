# git-anger-management [![Build Status](https://travis-ci.com/sondr3/git-anger-management.svg?token=jVZ9BLfdPx6kBm4z8gXS&branch=master)](https://travis-ci.com/sondr3/git-anger-management) [![Crates.io](https://img.shields.io/crates/v/git-anger-management.svg)](https://crates.io/crates/git-anger-management)

## What

Have you ever wondered how much you or your co-workers actually curse in your
commit messages? Worry no more, `git-anger-management` is here to help you.
Simply run it against your repository and it'll tell you who is the naughtiest
of them all.

## Why

Some times the only way to vent at the ridiculous crap we make is to write
really angry commit messages, I do it all the time. And I wanted to know just
how angry I get.

# Installation

Make sure you have Rust installed (I recommend installing via
[rustup](https://rustup.rs/)), then run `cargo install git-anger-management`.
You can now check how naughty you are by running `git anger-management` in the
directory where you want to check naughtiness.

Output should look something like this:

```sh
$ git anger-management
Sondre Nilsen
{
    "goddamn": 1,
    "shit": 5,
    "fucking": 13,
    "tits": 1,
    "fuck": 12
}
```
