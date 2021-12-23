use crate::solutions::load_data;
use std::io;

pub fn solve(filename: String, day : usize, part : usize) -> io::Result<()>  {
    println!("Generating Solution for Advent of Code 2021 - Day {} / Part {}", day, part);

    let mut data: Vec<String> = Vec::new();
    load_data(filename, day, part, &mut data)?;

    if part == 1 {

        let mut depth:i64 = 0;
        let mut horizontal:i64 = 0;

        for command in &data {
            let command_detail: Vec<&str> = command.split_whitespace().collect();
            let instruction:&str = command_detail[0];
            let value:i64 = command_detail[1].parse().unwrap();

            match instruction {
                "down" => depth = depth + value,
                "up" => depth = depth - value,
                "forward" => horizontal = horizontal + value,
                _ => println!("Unexpected instruction: {}", instruction),
            };
        }

        println!("Final position - depth: {}, horizontal: {} | result = {}", depth, horizontal, depth*horizontal);
    }

    if part == 2 {
        let mut depth:i64 = 0;
        let mut horizontal:i64 = 0;
        let mut aim:i64 = 0;

        for command in &data {
            let command_detail: Vec<&str> = command.split_whitespace().collect();
            let instruction:&str = command_detail[0];
            let value:i64 = command_detail[1].parse().unwrap();

            match instruction {
                "down" => aim = aim + value,
                "up" => aim = aim - value,
                "forward" => {horizontal = horizontal + value; depth = depth+(aim * value);},
                _ => println!("Unexpected instruction: {}", instruction),
            };
        }

        println!("Final position - depth: {}, horizontal: {} | result = {}", depth, horizontal, depth*horizontal);
        
    }

    Ok(())

}
