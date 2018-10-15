## 0.3.0
> 2018-10-15

Though this is the third minor release it's the first where I consider it to be
an actual working program. It's silly and simple, but it works. See the README
for more details.

#### Changelog:
* [[`9cd6f2fc08`](https://github.com/sondr3/git-anger-management/commit/9cd6f2fc08)] - Update README
* [[`39eb4538d4`](https://github.com/sondr3/git-anger-management/commit/39eb4538d4)] - Format files and fix Clippy lints
* [[`a55ab9463f`](https://github.com/sondr3/git-anger-management/commit/a55ab9463f)] - Add structopt, you can now see commits in arbitrary directories
* [[`c6c58342fb`](https://github.com/sondr3/git-anger-management/commit/c6c58342fb)] - Add an even worse example to the README \[ci skip\]
* [[`c8f6f2baee`](https://github.com/sondr3/git-anger-management/commit/c8f6f2baee)] - Add crates.io version \[ci skip\]
* [[`03403bf6a2`](https://github.com/sondr3/git-anger-management/commit/03403bf6a2)] - Update README, set to version 0.2.0
* [[`4af31796e9`](https://github.com/sondr3/git-anger-management/commit/4af31796e9)] - Create function to clean out characters from words
* [[`e06432cdd5`](https://github.com/sondr3/git-anger-management/commit/e06432cdd5)] - Format, run and apply Clippy
* [[`02949f153a`](https://github.com/sondr3/git-anger-management/commit/02949f153a)] - Format code with rustfmt
* [[`395a6171d8`](https://github.com/sondr3/git-anger-management/commit/395a6171d8)] - Walk the commits, find the authors, see if they've been naughty
* [[`b4b14ec2e8`](https://github.com/sondr3/git-anger-management/commit/b4b14ec2e8)] - Create a bunch of methods for the Author struct
* [[`4fcd0cafa0`](https://github.com/sondr3/git-anger-management/commit/4fcd0cafa0)] - Implement equality check between two authors
* [[`2409bb20bb`](https://github.com/sondr3/git-anger-management/commit/2409bb20bb)] - Walk the commit tree for authors
* [[`13df37c940`](https://github.com/sondr3/git-anger-management/commit/13df37c940)] - Implement Author::new() and derive debug
* [[`3ee891179a`](https://github.com/sondr3/git-anger-management/commit/3ee891179a)] - You know what, I'm silly, this function can be a single line
* [[`ee47d63b9b`](https://github.com/sondr3/git-anger-management/commit/ee47d63b9b)] - Add and test function that update an occurrence
* [[`19dda6b122`](https://github.com/sondr3/git-anger-management/commit/19dda6b122)] - Test filter\_occurrences
* [[`13bedeb584`](https://github.com/sondr3/git-anger-management/commit/13bedeb584)] - Key in occurrences is now &str, create function to filter out words
* [[`ebc19cd855`](https://github.com/sondr3/git-anger-management/commit/ebc19cd855)] - Test naughty\_word function, make it take a slice and not a Vec
* [[`03e8d8186f`](https://github.com/sondr3/git-anger-management/commit/03e8d8186f)] - Refactor out loop into it's own naughty word function
* [[`86849b2afb`](https://github.com/sondr3/git-anger-management/commit/86849b2afb)] - Add license, rename, update Cargo.toml, silence warnings
* [[`bdfa8d8b7f`](https://github.com/sondr3/git-anger-management/commit/bdfa8d8b7f)] - Update .travis.yml file
* [[`34f94a5d3c`](https://github.com/sondr3/git-anger-management/commit/34f94a5d3c)] - Never, ever, write unsafe code
* [[`8b36a76aca`](https://github.com/sondr3/git-anger-management/commit/8b36a76aca)] - Make CI fail on warnings
* [[`00402fefb5`](https://github.com/sondr3/git-anger-management/commit/00402fefb5)] - Add README \[ci skip\]
* [[`8406e217b1`](https://github.com/sondr3/git-anger-management/commit/8406e217b1)] - Add Travis CI configuration, .gitignore file
* [[`e6dc313f84`](https://github.com/sondr3/git-anger-management/commit/e6dc313f84)] - Add a actual swear words list, not something I made up myself
* [[`d3c83b7b77`](https://github.com/sondr3/git-anger-management/commit/d3c83b7b77)] - Add struct for Authors that curse
* [[`06ea3b3a51`](https://github.com/sondr3/git-anger-management/commit/06ea3b3a51)] - Format with rustfmt, filter out occurrences that never happen
* [[`25ce3ad93b`](https://github.com/sondr3/git-anger-management/commit/25ce3ad93b)] - Add structopt once we want to make this into a program
* [[`52fb070a1f`](https://github.com/sondr3/git-anger-management/commit/52fb070a1f)] - Open the current directory as a git repository
* [[`475c2efb63`](https://github.com/sondr3/git-anger-management/commit/475c2efb63)] - Add a bunch of curse words
* [[`6840a70e5c`](https://github.com/sondr3/git-anger-management/commit/6840a70e5c)] - Add git2 as a dependency
* [[`5329b2e938`](https://github.com/sondr3/git-anger-management/commit/5329b2e938)] - In the beginning there was darkness...
