use color_eyre::eyre::Result;
use color_eyre::eyre::WrapErr;
use std::fmt::Debug;
use std::fmt::Display;
use std::{fmt, fs, path::Path};

pub trait SolutionType: Display + Debug {}

pub struct Solution<'a, T, U> {
    pub day: i32,
    pub title: &'a str,
    pub input: &'a str,
    pub part1: fn(&str) -> Option<T>,
    pub part2: fn(&str) -> Option<U>,
}

// Todo implement benchmarking
impl<'a, T, U> Solution<'a, T, U> {
    fn evaluate(&self) -> (Option<T>, Option<U>) {
        ((self.part1)(self.input), (self.part2)(self.input))
    }
}

impl<'a, T, U> fmt::Debug for Solution<'a, T, U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Solution")
            .field("day", &self.day)
            .field("title", &self.title)
            .finish()
    }
}

impl<'a, T, U> fmt::Display for Solution<'a, T, U>
where
    T: Debug,
    U: Debug,
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

impl<'a, T, U> SolutionType for Solution<'a, T, U>
where
    T: Debug,
    U: Debug,
{
}

impl<'a, T, U> From<Solution<'a, T, U>> for Box<dyn SolutionType + 'a>
where
    Solution<'a, T, U>: SolutionType + 'a, // Solution implements SolutionType trait (covered by blanket impl)
    T: Debug + 'a,
    U: Debug + 'a,
{
    fn from(solution: Solution<'a, T, U>) -> Self {
        Box::new(solution) // Solution coerced to dyn SolutionType, see https://doc.rust-lang.org/reference/type-coercions.html#r-coerce.unsize
    }
}
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

#[derive(PartialEq, Eq)]
pub struct AsciiImage(String);

impl From<String> for AsciiImage {
    fn from(value: String) -> Self {
        AsciiImage { 0: value }
    }
}

// Needed to display solution nicely, since Option<String> will use the String debug implementation, which
// escapes whitespace instead of printing it
impl fmt::Debug for AsciiImage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n{}\n", self.0)
    }
}
