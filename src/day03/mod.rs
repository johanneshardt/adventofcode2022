use std::{collections::HashMap, collections::HashSet};

use crate::util::Solution;
use itertools::Itertools;

pub const SOLUTION: Solution<'static, i32, i32> = Solution {
    day: 03,
    title: "Rucksack Reorganization",
    input: include_str!("./main.input"),
    part1,
    part2,
};

fn char_to_priority(c: char) -> Option<u32> {
    match c as u32 {
        65..=90 => Some((c as u32) - 65 + 27),
        97..=122 => Some((c as u32) - 97 + 1),
        _ => None,
    }
}

fn part1(input: &str) -> Option<i32> {
    let common_items: Vec<char> = input
        .split_ascii_whitespace()
        .map(|line| {
            let (left, right) = line.split_at(line.len() / 2);
            let intersection: Vec<char> = left
                .chars()
                .filter(|c| right.contains(*c))
                .unique()
                .collect(); // O(nm), don't wanna bother with a hashmap though
            let err_msg = format!(
                "Couldn't parse line: {}, common chars: {:?}",
                line, &intersection
            );
            match intersection.len() {
                1 => intersection.first().map(|c| c.to_owned()),
                _ => None,
            }
            .expect(&err_msg)
        })
        .collect();

    Some(
        common_items
            .iter()
            .map(|c| {
                char_to_priority(*c)
                    .ok_or_else(|| format!("Couldn't find priority of char {}", c))
                    .map(|c| c as i32)
            })
            .fold(Ok(0), |acc, curr| {
                acc.and_then(|inner| curr.map(|i| i + inner))
            })
            .unwrap(),
    )
}
fn part2(input: &str) -> Option<i32> {
    let lines = input.split_ascii_whitespace().collect::<Vec<&str>>(); //fixes borrow error
    let common_items = lines.chunks(3).map(|rucksacks| {
        let mut m: HashMap<char, u32> = HashMap::new();
        for r in rucksacks {
            let mut found: HashSet<char> = HashSet::new();
            for c in r.chars() {
                if !found.contains(&c) {
                    let count = m.entry(c).or_insert(0);
                    *count += 1;
                    found.insert(c);
                }
            }
        }
        *m.iter()
            .find(|(_, value)| **value == 3)
            .unwrap_or_else(|| panic!("No common item type found in line: {:?}", &rucksacks))
            .0
    });

    Some(
        common_items
            .into_iter()
            .map(|c| {
                char_to_priority(c)
                    .ok_or_else(|| format!("Couldn't find priority of char {}", c))
                    .map(|c| c as i32)
            })
            .fold(Ok(0), |acc, curr| {
                acc.and_then(|inner| curr.map(|i| i + inner))
            })
            .unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(dead_code)]
    const INPUT: &str = include_str!("./sample.input");

    #[test]
    fn sample_1() {
        let expected: i32 = 157;
        debug_assert_eq!((SOLUTION.part1)(INPUT).unwrap(), expected)
    }

    #[test]
    fn sample_2() {
        let expected: i32 = 70;
        debug_assert_eq!((SOLUTION.part2)(INPUT).unwrap(), expected)
    }
}
