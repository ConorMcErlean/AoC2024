mod challenges;

use challenges::{day_1, day_2, day_3};
use std::fs;

pub fn read_file(filename: &str) -> String {
    let contents =
        fs::read_to_string(filename).expect("Should have been able to read the input file");
    contents
}

pub fn run(day: u32, input: String) {
    match day {
        1 => day_1::solve(input),
        2 => day_2::solve(input),
        3 => day_3::solve(input),
        _ => println!("Day not yet complete!"),
    }
}
