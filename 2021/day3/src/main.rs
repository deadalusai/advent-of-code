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

    let column_count = input.iter().next().unwrap().trim().len();
    let row_count = input.len();

    let mut ones_counts = vec![0; column_count];

    for row in input.iter() {
        for (i, count) in ones_counts.iter_mut().enumerate() {
            if &row[i..=i] == "1" {
                *count += 1;
            }
        }
    }

    let mut gamma = 0;
    let mut epsilon = 0;
    let half = row_count / 2;

    for (i, &count) in ones_counts.iter().enumerate() {
        // Note: bits in input are LEAST SIGNIFICANT first
        //  input("10110") => usize(01101)
        let bit_index = column_count - i - 1;
        gamma   |= if count > half { 1 << bit_index } else { 0 };
        epsilon |= if count < half { 1 << bit_index } else { 0 };
        // println!("g: {:b}, e: {:b}", gamma, epsilon);
    }

    println!("g: {}, e: {}", gamma, epsilon);

    let result = gamma * epsilon;

    println!("Part 1: {}", result);

    Ok(())
}
