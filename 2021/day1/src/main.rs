extern crate util;
extern crate itertools;

use util::{ read_input };
use util::error::{ AppErr };

fn main() -> Result<(), AppErr> {
    /*
    --- Part One ---
    count the number of times a depth measurement increases from the previous measurement.
    (There is no measurement before the first measurement.)
    In the example above, the changes are as follows:
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

    let mut seq = input.iter();

    let mut curr = seq.next().unwrap();
    let mut count = 0;

    for v in seq {
        if v > curr {
            count += 1;
        }
        curr = v;
    }

    println!("Result: {}", count);

    Ok(())
}
