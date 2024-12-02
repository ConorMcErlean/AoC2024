use core::panic;
use std::env;

use advent_of_code_2024::read_file;
use advent_of_code_2024::run;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Need to provide two arguements! A day, then a path to input file.")
    }
    let path = &args[2];
    let day: u32 = args[1].parse().expect("Day should be a number");
    let input = read_file(path);

    run(day, input)
}
