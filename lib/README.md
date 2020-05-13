<h1 align="center">git-anger-library</h1>
<p align="center">
   <a href="https://github.com/sondr3/git-anger-management/actions"><img alt="GitHub Actions Status" src="https://github.com/sondr3/git-anger-management/workflows/pipeline/badge.svg" /></a>
   <a href="https://crates.io/crates/git-anger-library"><img alt="Crates.io" src="https://img.shields.io/crates/v/git-anger-library?label=library"></a>
   <a href="https://docs.rs/crate/git-anger-library/"><img alt="Library documentation" src="https://docs.rs/git-anger-library/badge.svg"></a>
</p>

<p align="center">
   <strong>A fun little utility to figure out how naughty projects are.</strong>
</p>

## What

This is the main library that drives the
[`git-anger-management`](https://crates.io/crates/git-anger-management)
CLI-application, this is not really useful for anything besides that... but if
you for some reason want to, feel free!

## Features:

**Note:** None of these features are enabled by default, so you have to opt into
it like so:

```toml
[dependencies]
git-anger-library = { version = "0.8.0", features=["table", "json"] }
```

- `json`: Enables Serde serialization of the processed data using with the
  `print_json()` method.
- `table`: Enables pretty printing of the processed data using TabWriter with
  the function `print_table()`.

## License

GPLv3 or later.
