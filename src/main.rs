use crate::util::Solutions;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day10;
mod util;

fn main() {
    let solved_days = Solutions {
        all: vec![
            day01::SOLUTION.into(),
            day02::SOLUTION.into(),
            day03::SOLUTION.into(),
            day04::SOLUTION.into(),
            day05::SOLUTION.into(),
            day06::SOLUTION.into(),
            day07::SOLUTION.into(),
            day10::SOLUTION.into(),
        ],
    };
    println!("{}", solved_days);
}
