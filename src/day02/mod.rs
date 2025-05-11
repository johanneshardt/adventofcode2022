use crate::util::Solution;
use itertools::Itertools;

pub const SOLUTION: Solution<'static, i32, i32> = Solution {
    day: 02,
    title: "Rock Paper Scissors",
    input: include_str!("./main.input"),
    part1,
    part2,
};

fn parse(input: &str) -> Vec<(Move, Move)> {
    input
        .lines()
        .map(|line| {
            if let Some((fst, snd)) = line
                .trim()
                .split(' ')
                .map(|c| c.try_into().unwrap())
                .collect_tuple()
            {
                (fst, snd)
            } else {
                panic!("Couldn't parse line: '{}'", line)
            }
        })
        .collect()
}

fn part1(input: &str) -> Option<i32> {
    Some(score(&parse(input)))
}

fn part2(input: &str) -> Option<i32> {
    let part2: Vec<(Move, Move)> = parse(input)
        .iter()
        .map(|(fst, snd)| {
            let wins = fst.winner_over();
            let loses = fst.loser_over();
            let snd = match snd {
                Move::Rock => loses,    // you lose
                Move::Paper => *fst,    // you draw
                Move::Scissors => wins, // you win
            };
            (*fst, snd)
        })
        .collect();
    Some(score(&part2))
}

fn score(pairs: &[(Move, Move)]) -> i32 {
    pairs
        .iter()
        .map(|(fst, snd)| {
            *snd as i32
                + 1
                + match fst.winner_of(snd) {
                    Some(v) if v == *fst => 0,
                    Some(v) if v == *snd => 6,
                    Some(_) => unreachable!("Shouldn't happen"),
                    None => 3,
                }
        })
        .sum()
}

#[derive(PartialEq, Copy, Clone)]
enum Move {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}

impl Move {
    fn winner_of(&self, other: &Self) -> Option<Move> {
        match (*self as isize - *other as isize).rem_euclid(3) {
            0 => None,
            1 => Some(*self),
            2 => Some(*other),
            _ => unreachable!("Nope"),
        }
    }

    fn loser_over(&self) -> Move {
        match (*self as isize - 1).rem_euclid(3) {
            0 => Move::Rock,
            1 => Move::Paper,
            2 => Move::Scissors,
            _ => unreachable!("Guaranteed by mod 3"),
        }
    }

    fn winner_over(&self) -> Move {
        match (*self as isize + 1).rem_euclid(3) {
            0 => Move::Rock,
            1 => Move::Paper,
            2 => Move::Scissors,
            _ => unreachable!("Guaranteed by mod 3"),
        }
    }
}

impl TryFrom<&str> for Move {
    type Error = String;
    fn try_from(v: &str) -> Result<Self, Self::Error> {
        match v {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            _ => Err(format!("Input '{}' can't be parsed", v)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(dead_code)]
    const INPUT: &str = include_str!("./sample.input");

    #[test]
    fn sample_1() {
        let expected: i32 = 15;
        debug_assert_eq!((SOLUTION.part1)(INPUT).unwrap(), expected)
    }

    #[test]
    fn sample_2() {
        let expected: i32 = 12;
        debug_assert_eq!((SOLUTION.part2)(INPUT).unwrap(), expected)
    }
}
