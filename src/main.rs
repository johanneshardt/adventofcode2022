use crate::util::Solutions;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod util;

fn main() {
    let solved_days = Solutions {
        all: vec![
            Box::new(day01::SOLUTION),
            Box::new(day02::SOLUTION),
            Box::new(day03::SOLUTION),
            Box::new(day04::SOLUTION),
            Box::new(day05::SOLUTION),
            Box::new(day06::SOLUTION),
        ],
    };
    println!("{}", solved_days);
}
