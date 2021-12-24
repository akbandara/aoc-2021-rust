use crate::solutions::load_data;
use std::io;

pub fn solve(filename: String, day : usize, part : usize) -> io::Result<()>  {
    println!("Generating Solution for Advent of Code 2021 - Day {} / Part {}", day, part);

    let mut data: Vec<String> = Vec::new();
    load_data(filename, day, part, &mut data)?;

    if part == 1 {

        const REPORT_ENTRY_LEN: usize = 12;
        let mut bit_freq: [i64;REPORT_ENTRY_LEN] = [0; REPORT_ENTRY_LEN];
        let mut epsilon: [i64;REPORT_ENTRY_LEN] = [0; REPORT_ENTRY_LEN];
        let mut gamma: [i64;REPORT_ENTRY_LEN] = [0; REPORT_ENTRY_LEN];
        let mut epsilon_val = 0;
        let mut gamma_val = 0;

        for diagnostic_report in &data {

            let diagnostic_report_entry: Vec<char> = diagnostic_report.chars().collect();

            for index in 0..diagnostic_report_entry.len() {
                match diagnostic_report_entry[index] {
                    '0' => bit_freq[index] = bit_freq[index]-1,
                    '1' => bit_freq[index] = bit_freq[index]+1,
                    _ => println!("Unexpected report char: {}", diagnostic_report_entry[index]),
                };
            }

        }

        for index in 0..bit_freq.len() {
            if bit_freq[index] > 0 {
                gamma[index] = 1;
                epsilon[index] = 0;
            } else {
                gamma[index] = 0;
                epsilon[index] = 1;
            }
            let base : i64 = 2;
            let place_value : i64 = base.pow((REPORT_ENTRY_LEN as u32)-(index as u32)-1);
            epsilon_val = epsilon_val + epsilon[index]*place_value;
            gamma_val = gamma_val + gamma[index]*place_value;
        }


        println!("Calculated Gamma = {:?}, {}; Epsilon = {:?}, {} | Gamma x Epsilon = {}", gamma, gamma_val, epsilon, epsilon_val, epsilon_val*gamma_val);
        
    }

    if part == 2 {
       
        
    }

    Ok(())

}
