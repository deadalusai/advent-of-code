extern crate util;

use util::{ read_input };
use util::error::{ AppErr };

fn main() -> Result<(), AppErr> {
    /*
    --- Part One ---
    Consider your entire calibration document. What is the sum of all of the calibration values?
    */

    fn parse_item(s: &str) -> Result<u32, AppErr> {
        let numbers = s.chars()
            .filter_map(|c| c.to_digit(10))
            .collect::<Vec<_>>();
        let first = numbers.iter().next().ok_or("No calibration numbers in input")?;
        let last = numbers.iter().rev().next().unwrap();
        Ok((first * 10) + last)
    }

    let input = 
        read_input("input.txt")?
            .iter()
            .map(|s| parse_item(s))
            .collect::<Result<Vec<_>, AppErr>>()?;

    let sum = input.iter().sum::<u32>();
    println!("Day 1: {}", sum);

    Ok(())
}
