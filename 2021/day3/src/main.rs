extern crate util;
extern crate itertools;

use util::{ read_input };
use util::error::{ AppErr };

fn main() -> Result<(), AppErr> {

    let input = read_input("input.txt")?;

    /*
        --- Part One ---
        Gamma rate: For each column, select the most common bit
        Epsilon rate: For each column, select the least common bit

        E.g.
        010   G = 110
        110   E = 001
        101
        
        Use the binary numbers in your diagnostic report to calculate the gamma rate and epsilon rate, then multiply them together.
        What is the power consumption of the submarine? (Be sure to represent your answer in decimal, not binary.)
    */
    
    fn count_bits_at_index(input: &[String], at_bit_index: usize) -> (usize, usize) {
        let mut ones_count = 0;
        let mut zeroes_count = 0;
        for line in input.iter() {
            match &line[at_bit_index..=at_bit_index] {
                "1" => ones_count += 1,
                "0" => zeroes_count += 1,
                _   => unreachable!(),
            }
        }
        (ones_count, zeroes_count)
    }

    let column_count = input.iter().next().unwrap().len();

    let mut gamma = 0;
    let mut epsilon = 0;

    for i in 0..column_count {
        let (ones, zeroes) = count_bits_at_index(&input, i);
        // Note: bits in input are LEAST SIGNIFICANT first
        //  input("10110") => usize(01101)
        let bit_index = column_count - i - 1;
        gamma   |= if ones > zeroes { 1 << bit_index } else { 0 };
        epsilon |= if zeroes > ones { 1 << bit_index } else { 0 };
    }

    println!("g: {:b}, e: {:b}", gamma, epsilon);

    let result = gamma * epsilon;
    println!("Part 1: {}", result);

    /*
        --- Part Two ---
        https://adventofcode.com/2021/day/3#part2

        Use the binary numbers in your diagnostic report to calculate the oxygen generator rating and CO2 scrubber rating,
        then multiply them together. What is the life support rating of the submarine?
        (Be sure to represent your answer in decimal, not binary.)
    */

    enum FindBehavior { MostCommonPreferOnes, LeastCommonPreferZeroes }

    fn find_entry_with_most_common_bits(mut input: Vec<String>, behavior: FindBehavior) -> Option<String> {
        let column_count = input.iter().next().unwrap().trim().len();
        for i in 0..column_count {
            let (ones, zeroes) = count_bits_at_index(&input, i);
            let filter_char = match behavior {
                FindBehavior::MostCommonPreferOnes    => if ones >= zeroes { "1" } else { "0" },
                FindBehavior::LeastCommonPreferZeroes => if zeroes <= ones { "0" } else { "1" },
            };
            // Filter to strings which match the character at this position
            input.retain(|line| &line[i..=i] == filter_char);
            // Got one? Found our result
            if input.len() == 1 {
                return input.into_iter().next();
            }
        }
        None
    }

    fn input_binary_to_usize(line: &str) -> usize {
        let len = line.len();
        line.char_indices()
            .fold(0_usize, |r, (i, c)| {
                r | match c {
                    '1' => 1 << len - i - 1,
                    '0' => 0,
                    _   => unreachable!(),
                }
            })
    }

    let oxy_generator_rating = find_entry_with_most_common_bits(input.clone(), FindBehavior::MostCommonPreferOnes).map(|s| input_binary_to_usize(&s)).expect("oxy");
    let co2_scrubber_rating = find_entry_with_most_common_bits(input, FindBehavior::LeastCommonPreferZeroes).map(|s| input_binary_to_usize(&s)).expect("co2");

    println!("oxy: {}, co2: {}", oxy_generator_rating, co2_scrubber_rating);

    let result = oxy_generator_rating * co2_scrubber_rating;

    println!("Part 2: {}", result);

    Ok(())
}
