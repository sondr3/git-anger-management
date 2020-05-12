# git-anger-management [![pipeline](https://github.com/sondr3/git-anger-management/workflows/pipeline/badge.svg)](https://github.com/sondr3/git-anger-management/actions) [![Crates.io](https://img.shields.io/crates/v/git-anger-management.svg)](https://crates.io/crates/git-anger-management) [![Docs.rs](https://docs.rs/git-anger-management/badge.svg)](https://docs.rs/crate/git-anger-management/)

## What

Have you ever wondered just how angry your co-workers and/or yourself are? Or
just how _naughty_ the commit log for a project is? Worry no more,
`git-anger-management` is here to help you. Simply run it against your
repository and it'll tell you who is the naughtiest of them all.

[![asciicast](https://asciinema.org/a/329563.svg)](https://asciinema.org/a/329563)

## Why

We all have those moments where we finally figured out why something was utterly
broken, wrong or both and then had nowhere to scream but to our git commit. I do
it all the time, and I wanted to know just how angry I am at my code.

# Installation

There are two possible ways to install the binary for this:

1. Installation via `cargo`. For this you need to have installed (I recommend
   installing via [rustup](https://rustup.rs/)), then run `cargo install
   git-anger-management`. As long as you have `~/.cargo/bin` in your `$PATH` you
   can now use this program by running it in your terminal.
2. Download the correct binary from the
   [releases](https://github.com/sondr3/git-anger-management/releases), extract
   it and copy the file to somewhere on your \$PATH and make it executable:
   `chmod +x git-anger-management`.

## Usage

Simply run the command `git anger-management` and you should get some output
that looks something like this:

```sh
$ git anger-management
Author         bitch  bloody  damn  fuck  fucking  shitty  Total
------         -----  ------  ----  ----  -------  ------  -----
John Doe       0      1       1     0     0        0       2
Ola Nordmann   1      0       0     0     1        0       2
Sondre Nilsen  0      0       0     1     1        1       3
------         -----  ------  ----  ----  -------  ------  -----
Overall        1      1       1     1     2        1       7
```

You can also look in other directories if you want to look somwhere else but
you're too lazy to actually `cd` into that directory. Finally, you can also
print the output as JSON if you want to further manipulate the data by adding
the `-j/--json` flag to the binary:

```sh
$ git anger-management --json
{"name":"repo","total_commits":5,"total_curses":7,"curses":{"bloody":1,"shitty":1,"fuck":1,"bitch":1,"damn":1,"fucking":2},"authors":{"Sondre Nilsen":{"name":"Sondre Nilsen","total_commits":3,"total_curses":3,"curses":{"shitty":1,"fucking":1,"fuck":1}},"Ola Nordmann":{"name":"Ola Nordmann","total_commits":1,"total_curses":2,"curses":{"bitch":1,"fucking":1}},"John Doe":{"name":"John Doe","total_commits":1,"total_curses":2,"curses":{"damn":1,"bloody":1}}}}⏎
```

Finally, you can also look at the help by running `git anger-management -h` for more options.

# License

GPLv3 or later.
