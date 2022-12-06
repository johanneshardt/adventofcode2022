use crate::util::Solutions;
mod day01;
mod day02;
mod day03;
mod day04;
mod util;

fn main() {
    let solved_days = Solutions {
        all: vec![
            day01::solve::SOLUTION,
            day02::solve::SOLUTION,
            day03::solve::SOLUTION,
            day04::solve::SOLUTION,
        ],
    };
    println!("{}", solved_days);
}
