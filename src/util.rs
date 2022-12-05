use color_eyre::eyre::Result;
use color_eyre::eyre::WrapErr;
use std::{fmt, fs, path::Path};

pub struct Solution<'a> {
    pub day: i32,
    pub title: &'a str,
    pub input: &'a str,
    pub part1: fn(&str) -> Option<i32>,
    pub part2: fn(&str) -> Option<i32>,
}

// Todo implement benchmarking
impl<'a> Solution<'a> {
    fn evaluate(&self) -> (Option<i32>, Option<i32>) {
        ((self.part1)(&self.input), (self.part2)(&self.input))
    }
}

impl<'a> fmt::Debug for Solution<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Solution")
            .field("day", &self.day)
            .field("title", &self.title)
            .finish()
    }
}

impl<'a> fmt::Display for Solution<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (p1, p2) = self.evaluate();
        write!(
            f,
            "{solution:?} evaluates to: {{ part 1 -> {part1:?}, part 2 -> {part2:?} }}",
            solution = self,
            part1 = p1,
            part2 = p2
        )
    }
}

pub struct Solutions<'a> {
    pub all: Vec<Solution<'a>>,
}

impl fmt::Display for Solutions<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.all.iter().fold(Ok(()), |res, solution| {
            res.and_then(|_| writeln!(f, "{}", solution)) // clever solution to implement Display for Vec<Solution>, since Vec is outside the crate (https://github.com/apolitical/impl-display-for-vec)
        })
    }
}

#[allow(dead_code)]
pub fn read_file(path: &Path) -> Result<String> {
    fs::read_to_string(path).wrap_err(format!("Couldn't read file, with path '{:?}'", path))
}
