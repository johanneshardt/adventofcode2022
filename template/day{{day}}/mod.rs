use crate::util::Solution;
use itertools::Itertools;

pub const SOLUTION: Solution<'static, {{returntype}}> = Solution {
    day: {{day}},
    title: "{{title}}",
    input: include_str!("./main.input"),
    part1,
    part2,
};

fn part1(input: &str) -> Option<{{returntype}}> {
    None
}

fn part2(input: &str) -> Option<{{returntype}}> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(dead_code)]
    const INPUT: &str = include_str!("./sample.input");

    #[test]
    fn sample_1() {
        let expected = todo!();
        debug_assert_eq!((SOLUTION.part1)(INPUT).unwrap(), expected)
    }

    #[test]
    fn sample_2() {
        let expected = todo!();
        debug_assert_eq!((SOLUTION.part2)(INPUT).unwrap(), expected)
    }
}