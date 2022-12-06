use crate::util::Solution;
use itertools::Itertools;

pub const SOLUTION: Solution<'static> = Solution {
    day: 01,
    title: "Calorie Counting",
    input: include_str!("./in"),
    part1,
    part2,
};

fn parse(input: &str) -> Vec<i32> {
    input
        .split("\n\n")
        .map(|chunk| {
            chunk
                .split_ascii_whitespace()
                .map(|line| match line.trim().parse::<i32>() {
                    Ok(l) => l,
                    Err(_) => panic!("Parsing error: {}", line),
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
