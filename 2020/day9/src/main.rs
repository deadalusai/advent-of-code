extern crate util;
extern crate itertools;

use itertools::Itertools;

use util::{ read_input };
use util::error::{ AppErr };

fn main() -> Result<(), AppErr> {
    /*
    --- Part One ---

    XMAS starts by transmitting a preamble of 25 numbers.
    After that, each number you receive should be the sum of any two of the 25 immediately previous numbers.
    The two numbers will have different values, and there might be more than one such pair.

    The first step of attacking the weakness in the XMAS data is to find the first number in the list
    (after the preamble) which is not the sum of two of the 25 numbers before it.
    What is the first number that does not have this property?
    */

    let input = read_input("input.txt")?
        .iter()
        .map(|line| line.parse::<u64>())
        .collect::<Result<Vec<_>, _>>()?;

    fn find_first_number_without_sum_in_preamble(input: &[u64], preamble_size: usize) -> Result<u64, AppErr> {
        for i in preamble_size..input.len() {
            let v = input[i];
            let preamble = &input[i-preamble_size..i];
            let x = preamble.iter()
                .permutations(2)
                .map(|p| (*p[0], *p[1]))
                .find(|&(a, b)| a + b == v);
            if x.is_none() {
                return Ok(v);
            }
        }
        Err(AppErr::new("fail", "Unable to find number without sum in preamble"))
    }

    let part1 = find_first_number_without_sum_in_preamble(&input, 25)?;

    println!("Part 1: could not find sum for {:?}", part1);

    /*
    --- Part Two ---
    
    The final step in breaking the XMAS encryption relies on the invalid number you just
    found: you must find a contiguous set of at least two numbers in your list which sum
    to the invalid number from step 1.

    To find the encryption weakness, add together the smallest and largest number in this
    contiguous range. What is the encryption weakness in your XMAS-encrypted list of numbers?
    */

    fn find_range_which_sums_to_invalid_number(input: &[u64], num: u64) -> Result<(u64, u64), AppErr> {
        for a in 0..(input.len() - 1) {
            let mut sum = 0;
            for b in (a + 1)..input.len() {
                sum += input[b];
                if sum == num {
                    let range = &input[a..=b];
                    let min = *range.iter().min().unwrap();
                    let max = *range.iter().max().unwrap();
                    return Ok((min, max));
                }
            }
        }
        Err(AppErr::new("fail", "Unable to find range which sums to input"))
    }

    let part2 = find_range_which_sums_to_invalid_number(&input, part1)?;

    println!("Part 2: min and max of range which sums to {} are {:?} which sum to {}", part1, part2, part2.0 + part2.1);

    Ok(())
}
