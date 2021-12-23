use crate::solutions::load_data;
use std::io;

pub fn solve(filename: String, day : usize, part : usize) -> io::Result<()>  {
    println!("Generating Solution for Advent of Code 2021 - Day {} / Part {}", day, part);

    let mut data: Vec<String> = Vec::new();
    load_data(filename, day, part, &mut data)?;

    if part == 1 {

        
    }

    if part == 2 {
       
        
    }

    Ok(())

}
