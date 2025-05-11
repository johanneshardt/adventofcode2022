use std::collections::HashSet;

use crate::util::Solution;

pub const SOLUTION: Solution<'static, u32, u32> = Solution {
    day: 06,
    title: "Tuning Trouble",
    input: include_str!("./main.input"),
    part1,
    part2,
};

fn parse(input: &str) -> Vec<char> {
    input.chars().collect()
}

fn part1(input: &str) -> Option<u32> {
    let chars = parse(input);
    let count = chars
        .windows(4)
        .take_while(|cs| {
            let mut s: HashSet<char> = HashSet::new();
            s.extend(cs.iter());
            s.len() < 4
        })
        .count()
        + 4;
    Some(count as u32)
}

fn part2(input: &str) -> Option<u32> {
    let chars = parse(input);
    let count = chars
        .windows(14)
        .take_while(|cs| {
            let mut s: HashSet<char> = HashSet::new();
            s.extend(cs.iter());
            s.len() < 14
        })
        .count()
        + 14;
    Some(count as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(dead_code)]
    const INPUT: &str = include_str!("./sample.input");

    #[test]
    fn sample_1() {
        let expected = 7;
        debug_assert_eq!((SOLUTION.part1)(INPUT).unwrap(), expected)
    }

    #[test]
    fn sample_2() {
        let expected = 19;
        debug_assert_eq!((SOLUTION.part2)(INPUT).unwrap(), expected)
    }
}
