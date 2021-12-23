use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

pub mod aoc_day1_solver;
pub mod aoc_no_solver;

pub fn load_data(filename:String, day:usize, part:usize, data:&mut Vec<String>) -> io::Result<()> {
    println!("Reading input file {} to solve Day {} Part {}", filename, day, part);

    let file = File::open(filename)?;
    let file = BufReader::new(file);

    // read file into vector, password_entries
    let mut _data: Vec<String> = file
        .lines()
        .map(|line| line.unwrap())
        .collect();
    
    data.append(&mut _data);
    Ok(())
}
