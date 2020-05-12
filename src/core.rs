use once_cell::sync::OnceCell;
use std::collections::HashSet;

/// Statically include the word list for curse words
pub static CURSES_FILE: &str = include_str!("words.txt");

fn curses() -> &'static HashSet<&'static str> {
    static INSTANCE: OnceCell<HashSet<&'static str>> = OnceCell::new();
    INSTANCE.get_or_init(|| CURSES_FILE.lines().map(|l| l.trim()).collect())
}

/// Cleans a string and returns a list containing the cleaned up words.
///
/// Of note here is that the implementation splits on any character that is not
/// a letter, even if it is in the middle of a "word". This should not be a
/// problem.
pub fn split_into_clean_words(input: &str) -> impl Iterator<Item = &str> {
    input
        .split(|c: char| !char::is_alphabetic(c))
        .filter(|w| !w.is_empty())
}

/// Checks if a word is naughty.
pub fn naughty_word(word: &str) -> bool {
    curses().contains(&word)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_naughty_words() {
        assert!(naughty_word("fuck"));
        assert!(naughty_word("cyberfuckers"));
        assert!(naughty_word("shitty"));
        assert!(!naughty_word("pretty"));
    }

    #[test]
    fn test_clean_word() {
        let words = split_into_clean_words("This! is a string: with, some. words in? it;");
        assert_eq!(
            vec!["This", "is", "a", "string", "with", "some", "words", "in", "it"],
            words.collect::<Vec<_>>()
        );
    }
}
