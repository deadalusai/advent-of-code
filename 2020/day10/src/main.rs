extern crate util;

use util::{ read_input };
use util::error::{ AppErr };

fn main() -> Result<(), AppErr> {
    /*
    --- Part One ---

    Each of your joltage adapters is rated for a specific output joltage (your
    puzzle input). Any given adapter can take an input 1, 2, or 3 jolts lower
    than its rating and still produce its rated output joltage.

    In addition, your device has a built-in joltage adapter rated for 3 jolts
    higher than the highest-rated adapter in your bag.

    Find a chain that uses all of your adapters to connect the charging outlet
    to your device's built-in adapter and count the joltage differences between
    the charging outlet, the adapters, and your device. What is the number of
    1-jolt differences multiplied by the number of 3-jolt differences?
    */

    let mut adapter_ratings = read_input("input.txt")?
        .iter()
        .map(|line| line.parse::<u32>())
        .collect::<Result<Vec<_>, _>>()?;

    // add outlet and device ratings
    let outlet_rating = 0;
    let device_rating = *adapter_ratings.iter().max().unwrap() + 3;

    adapter_ratings.push(outlet_rating);
    adapter_ratings.push(device_rating);
    adapter_ratings.sort();

    let mut count1 = 0;
    let mut count3 = 0;

    for w in adapter_ratings.windows(2) {
        let a = w[0];
        let b = w[1];
        match b - a {
            1 => count1 += 1,
            3 => count3 += 1,
            x => return Err(AppErr::new("fail", &format!("found a delta of {}", x))),
        }
    }

    println!("Part 1: {} 1-jolt differences multiplied by {} 3-jolt differences is {}", count1, count3, count1 * count3);

    Ok(())
}
