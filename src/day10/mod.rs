use crate::util::*;
use itertools::Itertools;
use tap::Tap;

pub const SOLUTION: Solution<'static, i32, AsciiImage> = Solution {
    day: 10,
    title: "Cathode-Ray Tube",
    input: include_str!("./main.input"),
    part1,
    part2,
};

enum Instruction {
    NOOP,
    ADDX(i32),
}

fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            if let Some((_, value)) = line.split_once(" ") {
                Instruction::ADDX(
                    value
                        .parse()
                        .expect(&format!("Couldn't read instruction value: {value}")),
                )
            } else {
                Instruction::NOOP
            }
        })
        .collect()
}

fn evaluate(instructions: Vec<Instruction>) -> Vec<i32> {
    instructions
        .iter()
        .fold(vec![1], |acc, e| {
            let last = *acc.last().unwrap(); // start with X register value of 1
            match e {
                Instruction::NOOP => acc.clone().tap_mut(|m| m.push(last)),
                Instruction::ADDX(v) => acc
                    .clone()
                    .tap_mut(|m| m.push(last))
                    .tap_mut(|m| m.push(last + v)),
            }
        })
        .tap_mut(|m| {
            m.remove(m.len() - 1); // since we prepend 1 at the start
        })
}

fn part1(input: &str) -> Option<i32> {
    let eval = evaluate(parse(input));
    let signal_strengths = vec![
        eval[19] * 20,
        eval[59] * 60,
        eval[99] * 100,
        eval[139] * 140,
        eval[179] * 180,
        eval[219] * 220,
    ];
    println!("length: {}", eval.len());
    Some(signal_strengths.iter().sum())
}

fn part2(input: &str) -> Option<AsciiImage> {
    fn draw_sprite(i: i32, pixel: i32) -> bool {
        pixel == i - 1 || pixel == i || pixel == i + 1
    }

    fn render(eval: Vec<i32>) -> AsciiImage {
        eval.chunks(40) // rows are 40 pixels wide
            .map(|row| {
                row.iter()
                    .enumerate()
                    .map(|(i, pixel)| draw_sprite(i.try_into().unwrap(), *pixel))
                    .map(|pixel_is_lit| if pixel_is_lit { '#' } else { '.' })
                    .join("")
            })
            .join("\n")
            .into()
    }
    Some(render(evaluate(parse(input))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(dead_code)]
    const INPUT: &str = include_str!("./sample.input");

    #[test]
    fn sample_1() {
        let expected = 13140;
        debug_assert_eq!((SOLUTION.part1)(INPUT).unwrap(), expected)
    }

    #[test]
    fn sample_2() {
        let expected = vec![
            "##..##..##..##..##..##..##..##..##..##..",
            "###...###...###...###...###...###...###.",
            "####....####....####....####....####....",
            "#####.....#####.....#####.....#####.....",
            "######......######......######......####",
            "#######.......#######.......#######.....",
        ]
        .join("\n")
        .into();
        debug_assert_eq!((SOLUTION.part2)(INPUT).unwrap(), expected)
    }
}
