/// A single curse with a count of how many times it occurs
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct Curse {
    /// How many times have this curse been invoked?
    pub count: usize,
    /// The curses name
    pub curse: String,
}

impl Curse {
    /// Creates a new curse with a name and a count
    pub fn new(curse: impl Into<String>, count: usize) -> Self {
        Curse {
            curse: curse.into(),
            count,
        }
    }
}
