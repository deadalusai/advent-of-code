extern crate util;
extern crate itertools;

use util::{ read_input };
use util::error::{ AppErr };

use itertools::{ Itertools };

fn main() -> Result<(), AppErr> {
    /*
    --- Part One ---
    Before you leave, the Elves in accounting just need you to fix your expense report (your puzzle input); apparently, something isn't quite adding up.

    Specifically, they need you to find the two entries that sum to 2020 and then multiply those two numbers together.

    For example, suppose your expense report contained the following:

        1721
        979
        366
        299
        675
        1456

    In this list, the two entries that sum to 2020 are 1721 and 299. Multiplying them together produces 1721 * 299 = 514579, so the correct answer is 514579.

    Of course, your expense report is much larger. Find the two entries that sum to 2020; what do you get if you multiply them together?
    */

    fn parse_item(s: &str) -> Result<i32, AppErr> {
        let i = s.parse::<i32>()?;
        Ok(i)
    }

    let input = 
        read_input("input.txt")?
            .iter()
            .map(|s| parse_item(s))
            .collect::<Result<Vec<i32>, AppErr>>()?;

    // permute inputs and find the two which sum to 2020
    let result = input.iter()
        .permutations(2)
        .map(|v| (*v[0], *v[1]))
        .find(|(a, b)| a + b == 2020);
    
    if let Some((a, b)) = result {
        println!("Part 1: {} * {} == {}", a, b, a * b);
    }
    else {
        println!("Part 1: No items which sum to 2020 found!");
    }

    /*
    --- Part Two ---
    Find three numbers in your expense report that meet the same criteria.
    
    Using the above example again, the three entries that sum to 2020 are 979, 366, and 675. Multiplying them together produces the answer, 241861950.

    In your expense report, what is the product of the three entries that sum to 2020?
    */

    // permute inputs and find the three which sum to 2020
    let result = input.iter()
        .permutations(3)
        .map(|v| (*v[0], *v[1], *v[2]))
        .find(|(a, b, c)| a + b + c == 2020);
    
    if let Some((a, b, c)) = result {
        println!("Part 2: {} * {} * {} == {}", a, b, c, a * b * c);
    }
    else {
        println!("Part 2: No items which sum to 2020 found!");
    }
    

    Ok(())
}
