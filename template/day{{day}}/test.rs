#[cfg(test)]
use crate::day{{day}}::solve::SOLUTION;

#[allow(dead_code)]
const INPUT: &str = include_str!("./sample");

#[test]
fn sample_1() {
    let expected: i32 = 0;
    debug_assert_eq!((SOLUTION.part1)(INPUT).unwrap(), expected)
}

#[test]
fn sample_2() {
    let expected: i32 = 0;
    debug_assert_eq!((SOLUTION.part2)(INPUT).unwrap(), expected)
}
