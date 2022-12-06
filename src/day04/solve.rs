
use std::ops::RangeInclusive;

use crate::util::Solution;
use color_eyre::eyre::ContextCompat;

use color_eyre::eyre::WrapErr;
use itertools::Itertools;

pub const SOLUTION: Solution<'static> = Solution {
    day: 04,
    title: "Camp Cleanup",
    input: include_str!("./in"),
    part1,
    part2,
};

fn parse(input: &str) -> Vec<(RangeInclusive<i32>, RangeInclusive<i32>)> {
    input
        .split_ascii_whitespace()
        .map(|line| {
            line.trim()
                .split(',')
                .map(|range| {
                    let (left, right): (i32, i32) = range
                        .split('-')
                        .map(|i| {
                            i.parse::<i32>()
                                .wrap_err(format!("Can't parse number: {}", i))
                                .unwrap()
                        })
                        .collect_tuple()
                        .wrap_err(format!("Input formatting incorrect for range: {}", range))
                        .unwrap();
                    (left, right)
                })
                .map(|(left, right)| left..=right)
                .collect_tuple()
                .unwrap()
        })
        .collect_vec()
}
fn part1(input: &str) -> Option<i32> {
    let ranges = parse(input);
    Some(
        ranges
            .iter()
            .filter(|(first, second)| {
                (first.contains(second.start()) && first.contains(second.end()))
                    || (second.contains(first.start()) && second.contains(first.end()))
            })
            .count() as i32,
    )
}

fn part2(input: &str) -> Option<i32> {
    let ranges = parse(input);
    Some(
        ranges
            .iter()
            .filter(|(first, second)| {
                first.contains(second.start())
                    || first.contains(second.end())
                    || second.contains(first.start())
                    || second.contains(first.end())
            })
            .count() as i32,
    )
}
