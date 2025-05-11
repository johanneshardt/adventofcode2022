use crate::util::Solution;
use itertools::Itertools;

pub const SOLUTION: Solution<'static, i32, i32> = Solution {
    day: 01,
    title: "Calorie Counting",
    input: include_str!("./main.input"),
    part1,
    part2,
};

fn parse(input: &str) -> Vec<i32> {
    input
        .split("\n\n")
        .map(|chunk| {
            chunk
                .split("\n")
                .map(|line| match line.trim().parse::<i32>() {
                    Ok(l) => l,
                    Err(_) => panic!("Parsing error for line: `{}`", line),
                })
                .sum()
        })
        .sorted()
        .rev()
        .collect()
}

fn part1(input: &str) -> Option<i32> {
    parse(input).first().map(|i| i.to_owned())
}

fn part2(input: &str) -> Option<i32> {
    Some(parse(input)[0..3].iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(dead_code)]
    const INPUT: &str = include_str!("./sample.input");

    #[test]
    fn sample_1() {
        let expected: i32 = 24000;
        debug_assert_eq!((SOLUTION.part1)(INPUT).unwrap(), expected)
    }

    #[test]
    fn sample_2() {
        let expected: i32 = 45000;
        debug_assert_eq!((SOLUTION.part2)(INPUT).unwrap(), expected);
    }
}
