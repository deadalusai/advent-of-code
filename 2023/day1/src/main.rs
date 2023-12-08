extern crate util;

use util::read_input;
use util::error::AppErr;

fn main() -> Result<(), AppErr> {
    /*
    --- Part One ---
    Consider your entire calibration document. What is the sum of all of the calibration values?
    */

    fn parse_item_part_1(s: &str) -> Result<u32, AppErr> {
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
            .map(|s| parse_item_part_1(s))
            .collect::<Result<Vec<_>, AppErr>>()?;

    let sum = input.iter().sum::<u32>();
    println!("Part 1: {}", sum);

    /*
    --- Part Two ---
    Your calculation isn't quite right. It looks like some of the digits are actually spelled out with letters: one, two, three, four, five, six, seven, eight, and nine also count as valid "digits".
    What is the sum of all of the calibration values?
    */

    const DIGIT_WORDS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    fn extract_digit(s: &str) -> Option<u32> {
        // Scan for 0-9
        if let Some(d) = s.chars().next().and_then(|c| c.to_digit(10)) {
            return Some(d);
        }
        // Scan for word
        for (i, n) in DIGIT_WORDS.iter().enumerate() {
            if s.starts_with(n) {
                return Some((i + 1) as u32);
            }
        }
        // No digit
        None
    }

    fn parse_item_part_2(s: &str) -> Result<u32, AppErr> {
        // Need to find overlapping matches
        let numbers = (0..s.len())
            .filter_map(|offset| extract_digit(&s[offset..]))
            .collect::<Vec<_>>();

        let first = numbers.iter().next().ok_or("No calibration numbers in input")?;
        let last = numbers.iter().rev().next().unwrap();
        Ok((first * 10) + last)
    }

    let input = 
        read_input("input.txt")?
            .iter()
            .map(|s| parse_item_part_2(s))
            .collect::<Result<Vec<_>, AppErr>>()?;

    let sum = input.iter().sum::<u32>();
    println!("Part 2: {}", sum);

    Ok(())
}
