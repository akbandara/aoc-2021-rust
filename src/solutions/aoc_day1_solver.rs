use crate::solutions::load_data;
use std::io;

pub fn solve(filename: String, day : usize, part : usize) -> io::Result<()>  {
    println!("Generating Solution for Advent of Code 2021 - Day {} / Part {}", day, part);

    let mut data: Vec<String> = Vec::new();
    load_data(filename, day, part, &mut data)?;

    let readings: Vec<i64> = data.iter()
        .map(|line| line.parse::<i64>().unwrap())
        .collect();

    if part == 1 {
        let mut current_reading:i64 = readings[0];
        let mut num_increases:i64 = 0;
        for reading in &readings {
            if reading > &current_reading {
                num_increases = num_increases + 1;
            }
            current_reading = *reading;
        }

        println!("Number of depth increases {}", num_increases);
    }

    if part == 2 {
        let mut current_reading:i64 = readings[0]+readings[1]+readings[2];
        let mut num_increases:i64 = 0;
        for index in 0..(readings.len()-2) {
            if (readings[index]+readings[index+1]+readings[index+2]) > current_reading {
                num_increases = num_increases + 1;
            }
            current_reading = readings[index]+readings[index+1]+readings[index+2];
        }

        println!("Number of depth increases {}", num_increases);
        
    }

    Ok(())

}
