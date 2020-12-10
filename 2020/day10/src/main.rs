extern crate util;

use std::collections::{ HashMap };

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
        .map(|line| line.parse::<i64>())
        .collect::<Result<Vec<_>, _>>()?;

    // add outlet and device ratings
    let outlet_rating = 0;
    let device_rating = *adapter_ratings.iter().max().unwrap() + 3;

    adapter_ratings.push(outlet_rating);
    adapter_ratings.push(device_rating);
    adapter_ratings.sort();

    #[derive(PartialEq, Eq, Debug)]
    enum Delta { One, Three }

    let deltas = adapter_ratings
        .windows(2)
        .map(|w| match w[1] - w[0] {
            1 => Ok(Delta::One),
            3 => Ok(Delta::Three),
            x => Err(AppErr::new("fail", &format!("found a delta of {}", x))),
        })
        .collect::<Result<Vec<_>, _>>()?;

    // println!("{:?}", deltas);

    let count1 = deltas.iter().filter(|d| **d == Delta::One).count();
    let count3 = deltas.iter().filter(|d| **d == Delta::Three).count();

    println!("Part 1: {} 1-jolt differences multiplied by {} 3-jolt differences is {}", count1, count3, count1 * count3);

    /*
    --- Part Two --- (cheater, cheater)

    What is the total number of distinct ways you can arrange the adapters
    to legally connect the charging outlet to your device?
    */

    fn count_arrangements(adapters: &Vec<i64>, value: i64, memo: &mut HashMap<i64, i64>) -> i64 {
        if let Some(&ret) = memo.get(&value) {
            return ret;
        };
        if value == 0 {
            memo.insert(value, 1);
            return 1;
        }
        if value < 0 || !adapters.contains(&value) {
            memo.insert(value, 0);
            return 0;
        };
        let result = count_arrangements(adapters, value - 1, memo) +
                     count_arrangements(adapters, value - 2, memo) +
                     count_arrangements(adapters, value - 3, memo);
        memo.insert(value, result);
        result
    }

    let mut memo = HashMap::new();
    let arrangements = count_arrangements(&adapter_ratings, device_rating, &mut memo);

    println!("Part 2: {} valid permutations of adapters", arrangements);

    Ok(())
}
