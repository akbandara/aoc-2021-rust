use std::env;
use aoc_2021_rust::solutions::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let data_dir = &args[1];
    let aoc_day = &args[2].parse::<usize>().unwrap_or(0);
    let aoc_day_part = &args[3].parse::<usize>().unwrap_or(0);
    

    match aoc_day {
        1 => aoc_day1_solver::solve(data_dir.to_string(), *aoc_day, *aoc_day_part).ok(),
        _ => aoc_no_solver::solve(data_dir.to_string(), *aoc_day, *aoc_day_part).ok(),
    };
    
}