use hashbrown::HashSet;
use lazy_static::lazy_static;

pub static CURSES_FILE: &str = include_str!("words.txt");
lazy_static! {
    pub static ref CURSES: HashSet<&'static str> = CURSES_FILE.lines().collect();
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
    CURSES.contains(&word)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_naughty_words() {
        assert!(naughty_word("fuck"));
        assert!(naughty_word("cyberfuckers"));
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
