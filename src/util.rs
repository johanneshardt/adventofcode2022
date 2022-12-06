use color_eyre::eyre::Result;
use color_eyre::eyre::WrapErr;
use std::fmt::Debug;
use std::fmt::Display;
use std::{fmt, fs, path::Path};

pub trait SolutionType: Display {}

pub struct Solution<'a, T> {
    pub day: i32,
    pub title: &'a str,
    pub input: &'a str,
    pub part1: fn(&str) -> Option<T>,
    pub part2: fn(&str) -> Option<T>,
}

// Todo implement benchmarking
impl<'a, T> Solution<'a, T> {
    fn evaluate(&self) -> (Option<T>, Option<T>) {
        ((self.part1)(self.input), (self.part2)(self.input))
    }
}

impl<'a, T> fmt::Debug for Solution<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Solution")
            .field("day", &self.day)
            .field("title", &self.title)
            .finish()
    }
}

impl<'a, T> fmt::Display for Solution<'a, T>
where
    T: Debug,
{
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

impl<'a> SolutionType for Solution<'a, i32> {}
impl<'a> SolutionType for Solution<'a, u32> {}
impl<'a> SolutionType for Solution<'a, String> {}

pub struct Solutions {
    pub all: Vec<Box<dyn SolutionType>>,
}

impl fmt::Display for Solutions {
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
