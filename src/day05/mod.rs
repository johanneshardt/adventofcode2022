use std::fmt::Display;

use itertools::Itertools;

use crate::util::Solution;

pub const SOLUTION: Solution<'static, String, String> = Solution {
    day: 05,
    title: "Supply Stacks",
    input: include_str!("./main.input"),
    part1,
    part2,
};

pub(super) mod parsing {

    use nom::{
        IResult, Parser,
        branch::alt,
        character::complete::{alpha1, char, line_ending, not_line_ending, satisfy, u8},
        combinator::{all_consuming, map, value},
        multi::{count, many1, separated_list1},
        sequence::{delimited, preceded, terminated},
    };

    use super::{Crate, Instruction};

    fn match_block(input: &str) -> IResult<&str, Option<Crate>> {
        let parse_crate = map(
            delimited(char('['), satisfy(|c| c.is_alphabetic()), char(']')),
            |c: char| Some(Crate { c }),
        );
        let parse_empty = value(None, count(space, 3));
        alt((parse_crate, parse_empty)).parse(input) // match either a crate '[<char>]' or a space '   '
    }

    fn line_blocks(input: &str) -> IResult<&str, Vec<Option<Crate>>> {
        separated_list1(space, match_block).parse(input)
    }

    fn space(input: &str) -> IResult<&str, char> {
        satisfy(|c| c == ' ')(input)
    }

    fn line_instruction(input: &str) -> IResult<&str, Instruction> {
        fn word(input: &str) -> IResult<&str, &str> {
            alt((
                delimited(space, alpha1, space), // " word "
                terminated(alpha1, space),       // "word "
            ))
            .parse(input)
        }

        fn match_num(input: &str) -> IResult<&str, u8> {
            map(preceded(word, u8), |i| i).parse(input)
        }

        map((match_num, match_num, match_num), |(count, from, to)| {
            Instruction {
                count,
                from: from - 1,
                to: to - 1,
            }
        })
        .parse(input)
    }

    pub(super) fn parse(input: &str) -> (Vec<Vec<Option<Crate>>>, Vec<Instruction>) {
        let term_blocks = terminated(line_blocks, line_ending);
        let (input, towers) = match many1(term_blocks).parse(input) {
            Ok((input, towers)) => (input, towers),
            Err(e) => panic!("{}", e),
        };

        fn skip(input: &str) -> IResult<&str, &str> {
            terminated(terminated(not_line_ending, line_ending), line_ending).parse(input)
        }

        // There are better ways to skip lines
        let (input, _) = match skip(input) {
            Ok((input, nums_and_line)) => (input, nums_and_line),
            Err(e) => panic!("{}", e),
        };

        let instructions =
            match all_consuming(separated_list1(line_ending, line_instruction)).parse(input) {
                Ok((_, is)) => is,
                Err(e) => panic!("{}", e),
            };
        (towers, instructions)
    }
}

fn part1(input: &str) -> Option<String> {
    let (stacks, instructions) = parsing::parse(input);
    let mut stacks = transpose(stacks);
    for i in instructions {
        let (count, from, to) = i.into();
        for _ in 0..count {
            stacks.transfer(1, from as usize, to as usize);
        }
    }
    Some(
        stacks
            .s
            .iter()
            .filter_map(|s| s.s.last())
            .map(|c| c.c)
            .collect(),
    )
}

fn part2(input: &str) -> Option<String> {
    let (stacks, instructions) = parsing::parse(input);
    let mut stacks = transpose(stacks);
    for i in instructions {
        let (count, from, to) = i.into();
        stacks.transfer(count as usize, from as usize, to as usize);
    }
    Some(
        stacks
            .s
            .iter()
            .filter_map(|s| s.s.last())
            .map(|c| c.c)
            .collect(),
    )
}

#[derive(Clone, Copy, Debug)]
pub(self) struct Crate {
    c: char,
}

impl Display for Crate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.c)
    }
}

pub(self) struct Stack {
    s: Vec<Crate>,
}

impl Stack {
    fn pop(&mut self, count: usize) -> Vec<Crate> {
        self.s.drain(self.s.len() - count..).collect()
    }

    fn push(&mut self, elems: &mut Vec<Crate>) {
        self.s.append(elems)
    }
}

impl FromIterator<Crate> for Stack {
    fn from_iter<T: IntoIterator<Item = Crate>>(iter: T) -> Self {
        Stack {
            s: iter.into_iter().collect(),
        }
    }
}

impl Display for Stack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.s.iter().map(|c| c.to_string()).collect_vec().join(" ")
        )
    }
}

pub(self) struct Stacks {
    s: Vec<Stack>,
}

impl Stacks {
    fn transfer(&mut self, count: usize, from: usize, to: usize) {
        let mut crates = self.s[from].pop(count);
        self.s[to].push(&mut crates);
    }
}

impl FromIterator<Stack> for Stacks {
    fn from_iter<T: IntoIterator<Item = Stack>>(iter: T) -> Self {
        Stacks {
            s: iter.into_iter().collect(),
        }
    }
}

impl Display for Stacks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.s
                .iter()
                .enumerate()
                .map(|(i, c)| format!("{i}: {c}"))
                .collect_vec()
                .join("\n")
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub(self) struct Instruction {
    count: u8,
    from: u8,
    to: u8,
}

impl From<Instruction> for (u8, u8, u8) {
    fn from(i: Instruction) -> Self {
        (i.count, i.from, i.to)
    }
}

// Rows to stacks
fn transpose(mat: Vec<Vec<Option<Crate>>>) -> Stacks {
    (0..mat.iter().map(|s| s.len()).max().unwrap())
        .into_iter()
        .map(|col| {
            mat.iter()
                .flat_map(|row| row.get(col))
                .copied()
                .rev()
                .flatten() // cringe
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("./sample.input");

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
}
