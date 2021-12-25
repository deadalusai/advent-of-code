extern crate util;
extern crate itertools;

use util::{ read_input };
use util::error::{ AppErr };

fn main() -> Result<(), AppErr> {

    fn parse_instruction(s: &str) -> Result<(&str, i32), AppErr> {
        let mut parts = s.split(" ");
        Ok((parts.next().ok_or("Expected direction")?,
            parts.next().and_then(|s| s.parse::<i32>().ok()).ok_or("Invalid distance")?))
    }

    let input = read_input("input.txt")?;

    /*
        --- Part One ---
        Calculate the horizontal position and depth you would have after following the planned course.
        What do you get if you multiply your final horizontal position by your final depth?
    */

    let mut h = 0;
    let mut d = 0;

    for input in input.iter() {
        let (inst, dist) = parse_instruction(input)?;
        match inst {
            "forward" => h += dist,
            "up"      => d -= dist,
            "down"    => d += dist,
            _         => return Err(AppErr::from("Invalid instruction")),
        }
    }

    let result = h * d;

    println!("Part 1: {}", result);

    /*
        --- Part Two ---

        In addition to horizontal position and depth, you'll also need to track a third value, aim, which also starts at 0. The commands also mean something entirely different than you first thought:

        - down X increases your aim by X units.
        - up X decreases your aim by X units.
        - forward X does two things:
            - It increases your horizontal position by X units.
            - It increases your depth by your aim multiplied by X.

        Using this new interpretation of the commands, calculate the horizontal position and depth you would have after
        following the planned course. What do you get if you multiply your final horizontal position by your final depth?
    */

    let mut h = 0;
    let mut d = 0;
    let mut aim = 0;

    for input in input.iter() {
        let (inst, dist) = parse_instruction(input)?;
        match inst {
            "forward" => {
                h += dist;
                d += aim * dist;
            }
            "up"      => aim -= dist,
            "down"    => aim += dist,
            _         => return Err(AppErr::from("Invalid instruction")),
        }
    }

    let result = h * d;

    println!("Part 1: {}", result);

    Ok(())
}
