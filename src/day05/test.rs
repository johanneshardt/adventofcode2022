#[cfg(test)]
use crate::day05::solve::SOLUTION;

#[allow(dead_code)]
const INPUT: &str = include_str!("./sample");

#[test]
fn sample_1() {
    let expected: String = "CMZ".to_owned();
    debug_assert_eq!((SOLUTION.part1)(INPUT).unwrap(), expected)
}

#[test]
fn sample_2() {
    let expected: String = "MCD".to_owned();
    debug_assert_eq!((SOLUTION.part2)(INPUT).unwrap(), expected)
}
