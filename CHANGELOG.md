## v0.8.1 git-anger-lib

> 2020-05-13

A patch release to update the documentation and make `docs.rs` build with all
features enabled.

## Changes

- [[`fea9645fca`](https://github.com/sondr3/git-anger-management/commit/fea9645fca)] - Update metadata for docs.rs build, release v0.8.1
- [[`b2fc2fcd58`](https://github.com/sondr3/git-anger-management/commit/b2fc2fcd58)] - Update README, CHANGELOG

## v0.8.0 git-anger-lib

> 2020-05-13

Entirely a behind the scenes release, this is so that downstream users can properly use the features that they require:

- `json`: Turns on Serde serialization of the repo data
- `table`: Turns on printing a prettified table to `stdout`

## Changes

- [[`b05911cbd9`](https://github.com/sondr3/git-anger-management/commit/b05911cbd9)] - Release v0.8.0-lib
- [[`8e4745f94b`](https://github.com/sondr3/git-anger-management/commit/8e4745f94b)] - Properly feature gate features
- [[`f2ece8a634`](https://github.com/sondr3/git-anger-management/commit/f2ece8a634)] - Add READMEs for the individual workspace members
- [[`5e8cf95e7a`](https://github.com/sondr3/git-anger-management/commit/5e8cf95e7a)] - Update CI, README for workspace
- [[`a194a4d4d6`](https://github.com/sondr3/git-anger-management/commit/a194a4d4d6)] - Fix CI error
- [[`2c58f3939f`](https://github.com/sondr3/git-anger-management/commit/2c58f3939f)] - Split into workspace
- [[`c44b1a63bc`](https://github.com/sondr3/git-anger-management/commit/c44b1a63bc)] - Update changelog, installation instructions in README

## 0.7.0

> 2020-05-12

This is a fairly large structural overhaul; mostly from having gotten a fair bit
more experience programming in Rust. Notable changes in this release include:

- You can now print the repository output as either JSON or a simple table.
- Progress bars were removed, mostly due to massively slowing down the program
- A fairly sizeable overhaul of the internals, making it more idiomatic and
  faster

On the other end binaries are now automatically created for all major platforms,
tests are run on both stable and nightly and we've transitioned to using GitHub
Actions instead of TravisCI.

- [[`9cf642e1cd`](https://github.com/sondr3/git-anger-management/commit/9cf642e1cd)] - Update the documentation
- [[`02836d39c8`](https://github.com/sondr3/git-anger-management/commit/02836d39c8)] - Start splitting into features for the future
- [[`2f1cae9bdf`](https://github.com/sondr3/git-anger-management/commit/2f1cae9bdf)] - Use GH Actions Badge instead of Travis
- [[`6442b684f4`](https://github.com/sondr3/git-anger-management/commit/6442b684f4)] - Update tests
- [[`5b4b257197`](https://github.com/sondr3/git-anger-management/commit/5b4b257197)] - Remove unused method
- [[`ac6e891324`](https://github.com/sondr3/git-anger-management/commit/ac6e891324)] - Fix total curse values being wrong
- [[`7db0b523f8`](https://github.com/sondr3/git-anger-management/commit/7db0b523f8)] - Fix some words slipping through due to newlines
- [[`a0435c6b30`](https://github.com/sondr3/git-anger-management/commit/a0435c6b30)] - Move to TabWriter to print output
- [[`6130210600`](https://github.com/sondr3/git-anger-management/commit/6130210600)] - Move reading commits etc into Repo::new()
- [[`d9f5571627`](https://github.com/sondr3/git-anger-management/commit/d9f5571627)] - Restructure app a bit
- [[`8358880979`](https://github.com/sondr3/git-anger-management/commit/8358880979)] - Add test repo, start refactoring app
- [[`5bf3d245bf`](https://github.com/sondr3/git-anger-management/commit/5bf3d245bf)] - Remove indicatif, simply print a message and clear it
- [[`41dc63dd0c`](https://github.com/sondr3/git-anger-management/commit/41dc63dd0c)] - Move to GitHub Actions for CI, update license
- [[`665eb3d536`](https://github.com/sondr3/git-anger-management/commit/665eb3d536)] - Add totals to table, refactor type signatures slightly
- [[`6116a1efa5`](https://github.com/sondr3/git-anger-management/commit/6116a1efa5)] - Remove curse struct, not needed anymore
- [[`b4d9536549`](https://github.com/sondr3/git-anger-management/commit/b4d9536549)] - Remove wang and knob from wordlist, too common to be curses
- [[`c1870b7c50`](https://github.com/sondr3/git-anger-management/commit/c1870b7c50)] - Print everything as a table
- [[`4dd3161415`](https://github.com/sondr3/git-anger-management/commit/4dd3161415)] - Updated dependencies, fixed lints and refactored to work with new versions
- [[`49e8156c19`](https://github.com/sondr3/git-anger-management/commit/49e8156c19)] - A really, really hacky and stupid ugly way to print things in columns
- [[`251852b97c`](https://github.com/sondr3/git-anger-management/commit/251852b97c)] - Add some documentation
- [[`dd597771f6`](https://github.com/sondr3/git-anger-management/commit/dd597771f6)] - Output information in tables instead of pretty printing it as a string
- [[`f1a541cb05`](https://github.com/sondr3/git-anger-management/commit/f1a541cb05)] - Add option to only display information about the repo, omitting authors
- [[`59d99940bf`](https://github.com/sondr3/git-anger-management/commit/59d99940bf)] - Add struct for curses

## 0.6.0

> 2018-12-27

This release adds progress bars when running `git-anger-management`. Useful when
parsing commits in larger repositories as this will show the elapsed time, time
remaining etc. The rest of the changes are behind-the-scenes stuff, updated
dependencies and updated to the latest Rust edition (2018).

- [[`53e31ad5a8`](https://github.com/sondr3/git-anger-management/commit/53e31ad5a8)] - Update README to include asciinema "video" of output
- [[`4f0a873733`](https://github.com/sondr3/git-anger-management/commit/4f0a873733)] - Update CLI struct to use newer StructOpt changes
- [[`27937aced4`](https://github.com/sondr3/git-anger-management/commit/27937aced4)] - Update to Rust 2018
- [[`3b441d666b`](https://github.com/sondr3/git-anger-management/commit/3b441d666b)] - Update Travis configuration
- [[`db5c363fe5`](https://github.com/sondr3/git-anger-management/commit/db5c363fe5)] - Update dependencies
- [[`544204594f`](https://github.com/sondr3/git-anger-management/commit/544204594f)] - Remove version-sync from dev-dependencies
- [[`e9348326cd`](https://github.com/sondr3/git-anger-management/commit/e9348326cd)] - Add progress bars to CLI to track progress
- [[`ce0b7678ec`](https://github.com/sondr3/git-anger-management/commit/ce0b7678ec)] - Move magic commands to below documentation, add a few more deny things

## 0.5.1

> 2018-11-09

Minor bug fixes, forgot to update the link to the documentation and fixed some
outdated Clippy settings.

- [[`a3a527e1c1`](https://github.com/sondr3/git-anger-management/commit/a3a527e1c1)] - A pedantic Clippy was very annoying, disable for now
- [[`96c5ead508`](https://github.com/sondr3/git-anger-management/commit/96c5ead508)] - Update Cargo.toml and refactor it slightly
- [[`8e6acc63b1`](https://github.com/sondr3/git-anger-management/commit/8e6acc63b1)] - Update Clippy settings, make it even... worse

## 0.5.0

> 2018-11-09

This release focuses on performance, even though it might be fairly minor. We
switched the standard library version of HashMap and HashSet to the ones
implemented in [hashbrown](https://github.com/Amanieu/hashbrown), switched to
using `lazy_static` since it doesn't require `unsafe` code anymore and switched
to using `LTO=thin` for releases, all giving us some minor speed boosts. You can
see more in the git commits for the respective changes. We also added
documentation and also display total counts of naughty words across all authors
in the repository.

- [[`21fd6a5d33`](https://github.com/sondr3/git-anger-management/commit/21fd6a5d33)] - Document all the things, ensure it will forever be documented
- [[`4e2d1fa786`](https://github.com/sondr3/git-anger-management/commit/4e2d1fa786)] - Make sure the version number is always up to date
- [[`1b663923ad`](https://github.com/sondr3/git-anger-management/commit/1b663923ad)] - Initial pass at writing documentation
- [[`22c851a22c`](https://github.com/sondr3/git-anger-management/commit/22c851a22c)] - Use LTO=thin when running a release version, even more minor speedups
- [[`cde15774c3`](https://github.com/sondr3/git-anger-management/commit/cde15774c3)] - Refactor whole thing into several files, library and binary
- [[`99488db4f6`](https://github.com/sondr3/git-anger-management/commit/99488db4f6)] - Use lazy_static to lazily create curses, minor speedups again
- [[`6ce95329fc`](https://github.com/sondr3/git-anger-management/commit/6ce95329fc)] - Include Cargo.lock since this is a binary application
- [[`92646d638c`](https://github.com/sondr3/git-anger-management/commit/92646d638c)] - Use hashbrown instead of std::{HashMap, HashSet} for minor speedups
- [[`b0284df3cf`](https://github.com/sondr3/git-anger-management/commit/b0284df3cf)] - Fix Clippy lint
- [[`d2b66c882e`](https://github.com/sondr3/git-anger-management/commit/d2b66c882e)] - Show curses per repo and not just per author
- [[`b2c074db76`](https://github.com/sondr3/git-anger-management/commit/b2c074db76)] - Change update_occurrence so that it doesn't copy strings
- [[`4ae3a8235f`](https://github.com/sondr3/git-anger-management/commit/4ae3a8235f)] - Use if let on both author and commit message
- [[`cb090b909d`](https://github.com/sondr3/git-anger-management/commit/cb090b909d)] - Refactor adding commits to a vec to be in their own little scope
- [[`0a77ad1a2f`](https://github.com/sondr3/git-anger-management/commit/0a77ad1a2f)] - Remove badge because crates.io doesn't work with travis.com

## 0.4.0

> 2018-10-18

Thanks to everyone who chimed in on both reddit and on GitHub for helping!
Especially [Darren Tsung](https://github.com/DarrenTsung), whose pull request
has shown me so much new stuff that I haven't learned yet. I've applied some of
the suggestions from him making the code more readable and added a verbosity
flag and removed some unneeded words from the word list.

#### Changelog:

- [[`fda389c4d9`](https://github.com/sondr3/git-anger-management/commit/fda389c4d9)] - Update version of git-anger-management
- [[`e29428bd5a`](https://github.com/sondr3/git-anger-management/commit/e29428bd5a)] - Update changelog
- [[`d0ccfaff2c`](https://github.com/sondr3/git-anger-management/commit/d0ccfaff2c)] - Fix failing CI due to formatting
- [[`19bb81d936`](https://github.com/sondr3/git-anger-management/commit/19bb81d936)] - Add verbosity flag
- [[`8be2eca194`](https://github.com/sondr3/git-anger-management/commit/8be2eca194)] - Match against what directory to look from, neat
- [[`e1ab587700`](https://github.com/sondr3/git-anger-management/commit/e1ab587700)] - More suggestions from #1
- [[`8f34037514`](https://github.com/sondr3/git-anger-management/commit/8f34037514)] - Minor removal of extra stupid words
- [[`45c328ca9b`](https://github.com/sondr3/git-anger-management/commit/45c328ca9b)] - Use split_into_clean_words() from #1
- [[`f255dca68c`](https://github.com/sondr3/git-anger-management/commit/f255dca68c)] - Format code
- [[`7d7c595c37`](https://github.com/sondr3/git-anger-management/commit/7d7c595c37)] - Minor cleanups and refactorings
- [[`7d63834b84`](https://github.com/sondr3/git-anger-management/commit/7d63834b84)] - Refactor author initialization
- [[`d60bf41e32`](https://github.com/sondr3/git-anger-management/commit/d60bf41e32)] - Remove functions where you can directly update values
- [[`6fabb676bb`](https://github.com/sondr3/git-anger-management/commit/6fabb676bb)] - We can just derive PartialEq, no need for our own implementation
- [[`feb2db45c3`](https://github.com/sondr3/git-anger-management/commit/feb2db45c3)] - Add a struct for the repo being counted
- [[`585f7a1674`](https://github.com/sondr3/git-anger-management/commit/585f7a1674)] - Fix wrong ordering on counting commits
- [[`1ca19cbf2d`](https://github.com/sondr3/git-anger-management/commit/1ca19cbf2d)] - Implement Display for Author, add counters for commits/curses

## 0.3.0

> 2018-10-15

Though this is the third minor release it's the first where I consider it to be
an actual working program. It's silly and simple, but it works. See the README
for more details.

#### Changelog:

- [[`9cd6f2fc08`](https://github.com/sondr3/git-anger-management/commit/9cd6f2fc08)] - Update README
- [[`39eb4538d4`](https://github.com/sondr3/git-anger-management/commit/39eb4538d4)] - Format files and fix Clippy lints
- [[`a55ab9463f`](https://github.com/sondr3/git-anger-management/commit/a55ab9463f)] - Add structopt, you can now see commits in arbitrary directories
- [[`c6c58342fb`](https://github.com/sondr3/git-anger-management/commit/c6c58342fb)] - Add an even worse example to the README \[ci skip\]
- [[`c8f6f2baee`](https://github.com/sondr3/git-anger-management/commit/c8f6f2baee)] - Add crates.io version \[ci skip\]
- [[`03403bf6a2`](https://github.com/sondr3/git-anger-management/commit/03403bf6a2)] - Update README, set to version 0.2.0
- [[`4af31796e9`](https://github.com/sondr3/git-anger-management/commit/4af31796e9)] - Create function to clean out characters from words
- [[`e06432cdd5`](https://github.com/sondr3/git-anger-management/commit/e06432cdd5)] - Format, run and apply Clippy
- [[`02949f153a`](https://github.com/sondr3/git-anger-management/commit/02949f153a)] - Format code with rustfmt
- [[`395a6171d8`](https://github.com/sondr3/git-anger-management/commit/395a6171d8)] - Walk the commits, find the authors, see if they've been naughty
- [[`b4b14ec2e8`](https://github.com/sondr3/git-anger-management/commit/b4b14ec2e8)] - Create a bunch of methods for the Author struct
- [[`4fcd0cafa0`](https://github.com/sondr3/git-anger-management/commit/4fcd0cafa0)] - Implement equality check between two authors
- [[`2409bb20bb`](https://github.com/sondr3/git-anger-management/commit/2409bb20bb)] - Walk the commit tree for authors
- [[`13df37c940`](https://github.com/sondr3/git-anger-management/commit/13df37c940)] - Implement Author::new() and derive debug
- [[`3ee891179a`](https://github.com/sondr3/git-anger-management/commit/3ee891179a)] - You know what, I'm silly, this function can be a single line
- [[`ee47d63b9b`](https://github.com/sondr3/git-anger-management/commit/ee47d63b9b)] - Add and test function that update an occurrence
- [[`19dda6b122`](https://github.com/sondr3/git-anger-management/commit/19dda6b122)] - Test filter_occurrences
- [[`13bedeb584`](https://github.com/sondr3/git-anger-management/commit/13bedeb584)] - Key in occurrences is now &str, create function to filter out words
- [[`ebc19cd855`](https://github.com/sondr3/git-anger-management/commit/ebc19cd855)] - Test naughty_word function, make it take a slice and not a Vec
- [[`03e8d8186f`](https://github.com/sondr3/git-anger-management/commit/03e8d8186f)] - Refactor out loop into it's own naughty word function
- [[`86849b2afb`](https://github.com/sondr3/git-anger-management/commit/86849b2afb)] - Add license, rename, update Cargo.toml, silence warnings
- [[`bdfa8d8b7f`](https://github.com/sondr3/git-anger-management/commit/bdfa8d8b7f)] - Update .travis.yml file
- [[`34f94a5d3c`](https://github.com/sondr3/git-anger-management/commit/34f94a5d3c)] - Never, ever, write unsafe code
- [[`8b36a76aca`](https://github.com/sondr3/git-anger-management/commit/8b36a76aca)] - Make CI fail on warnings
- [[`00402fefb5`](https://github.com/sondr3/git-anger-management/commit/00402fefb5)] - Add README \[ci skip\]
- [[`8406e217b1`](https://github.com/sondr3/git-anger-management/commit/8406e217b1)] - Add Travis CI configuration, .gitignore file
- [[`e6dc313f84`](https://github.com/sondr3/git-anger-management/commit/e6dc313f84)] - Add a actual swear words list, not something I made up myself
- [[`d3c83b7b77`](https://github.com/sondr3/git-anger-management/commit/d3c83b7b77)] - Add struct for Authors that curse
- [[`06ea3b3a51`](https://github.com/sondr3/git-anger-management/commit/06ea3b3a51)] - Format with rustfmt, filter out occurrences that never happen
- [[`25ce3ad93b`](https://github.com/sondr3/git-anger-management/commit/25ce3ad93b)] - Add structopt once we want to make this into a program
- [[`52fb070a1f`](https://github.com/sondr3/git-anger-management/commit/52fb070a1f)] - Open the current directory as a git repository
- [[`475c2efb63`](https://github.com/sondr3/git-anger-management/commit/475c2efb63)] - Add a bunch of curse words
- [[`6840a70e5c`](https://github.com/sondr3/git-anger-management/commit/6840a70e5c)] - Add git2 as a dependency
- [[`5329b2e938`](https://github.com/sondr3/git-anger-management/commit/5329b2e938)] - In the beginning there was darkness...
