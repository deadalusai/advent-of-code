extern crate util;
extern crate itertools;

use util::{ read_input };
use util::error::{ AppErr };

fn main() -> Result<(), AppErr> {

    fn parse_isntruction(s: &str) -> Result<(String, i32), AppErr> {
        let mut parts = s.split(" ");
        Ok((parts.next().map(|s| s.to_string()).ok_or("Expected direction")?,
            parts.next().and_then(|s| s.parse::<i32>().ok()).ok_or("Invalid distance")?))
    }

    let input = 
        read_input("input.txt")?
            .iter()
            .map(|s| parse_isntruction(s))
            .collect::<Result<Vec<_>, AppErr>>()?;
    /*
        --- Part One ---
        Calculate the horizontal position and depth you would have after following the planned course.
        What do you get if you multiply your final horizontal position by your final depth?
    */

    let mut h = 0;
    let mut d = 0;

    for (inst, dist) in input.iter() {
        match inst.as_str() {
            "forward" => h += dist,
            "up"      => d -= dist,
            "down"    => d += dist,
            _         => return Err(AppErr::from("Invalid instruction")),
        }
    }

    let result = h * d;

    println!("Part 1: {}", result);

    Ok(())
}
