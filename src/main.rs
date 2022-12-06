use crate::util::Solutions;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod util;

fn main() {
    let solved_days = Solutions {
        all: vec![
            Box::new(day01::solve::SOLUTION),
            Box::new(day02::solve::SOLUTION),
            Box::new(day03::solve::SOLUTION),
            Box::new(day04::solve::SOLUTION),
            Box::new(day05::solve::SOLUTION),
        ],
    };
    println!("{}", solved_days);
}
