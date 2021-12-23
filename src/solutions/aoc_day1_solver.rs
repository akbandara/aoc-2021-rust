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
        let mut average_readings: Vec<i64> = [].to_vec();
        for index in 0..(readings.len()-2) {
            average_readings.push(readings[index]+readings[index+1]+readings[index+2]);
        }

        let mut current_reading:i64 = average_readings[0];
        let mut num_increases:i64 = 0;
        for avg_reading in &average_readings {
            if avg_reading > &current_reading {
                num_increases = num_increases + 1;
            }
            current_reading = *avg_reading;
        }

        println!("Number of depth increases {}", num_increases);
        
    }

    Ok(())

}
