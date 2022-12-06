use crate::util::Solution;
use itertools::Itertools;

pub const SOLUTION: Solution<'static> = Solution {
    day: 02,
    title: "Rock Paper Scissors",
    input: include_str!("./in"),
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
    let part2 = &parse(input)
        .iter()
        .map(|(fst, snd)| {
            let wins = fst.winner_over();
            let loses = fst.loser_over();
            let snd = match snd {
                Move::ROCK => loses,    // you lose
                Move::PAPER => *fst,    // you draw
                Move::SCISSORS => wins, // you win
            };
            (*fst, snd)
        })
        .collect();
    Some(score(part2))
}

fn score(pairs: &Vec<(Move, Move)>) -> i32 {
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
    ROCK = 0,
    PAPER = 1,
    SCISSORS = 2,
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
            0 => Move::ROCK,
            1 => Move::PAPER,
            2 => Move::SCISSORS,
            _ => unreachable!("Guaranteed by mod 3"),
        }
    }

    fn winner_over(&self) -> Move {
        match (*self as isize + 1).rem_euclid(3) {
            0 => Move::ROCK,
            1 => Move::PAPER,
            2 => Move::SCISSORS,
            _ => unreachable!("Guaranteed by mod 3"),
        }
    }
}

impl TryFrom<&str> for Move {
    type Error = String;
    fn try_from(v: &str) -> Result<Self, Self::Error> {
        match v {
            "A" | "X" => Ok(Move::ROCK),
            "B" | "Y" => Ok(Move::PAPER),
            "C" | "Z" => Ok(Move::SCISSORS),
            _ => Err(format!("Input '{}' can't be parsed", v)),
        }
    }
}
