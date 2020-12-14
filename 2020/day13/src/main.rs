extern crate util;

use util::{ read_input };
use util::error::{ AppErr };

fn main() -> Result<(), AppErr> {
    /*
    --- Part One ---

    What is the ID of the earliest bus you can take to the airport multiplied
    by the number of minutes you'll need to wait for that bus?
    */

    let input = read_input("input.txt")?;

    let earliest_time_to_leave = input[0].parse::<u32>()?;
    let bus_ids = input[1].split(',')
        .filter_map(|id| match id {
            "x" => None,
            id  => Some(id.parse::<u32>()),
        })
        .collect::<Result<Vec<_>, _>>()?;

    fn bus_timetable(bus_id: u32) -> impl Iterator<Item=(u32, u32)> {
        (0..).map(move |i| (bus_id, i * bus_id))
    }

    let earliest_bus = bus_ids
        .iter()
        .filter_map(|id| {
            bus_timetable(*id)
                .take(1_000_000)
                .find(|(_, time)| *time >= earliest_time_to_leave)
        })
        .fold(None, |best, (id, time)| match best {
            Some((_, best)) if time < best => Some((id, time)),
            Some(b) => Some(b),
            None => Some((id, time)),
        })
        .ok_or_else(|| AppErr::new("error", "unable to find earliest bus"))?;

    let (id, time) = earliest_bus;
    let time_to_wait = time - earliest_time_to_leave;
    let puzzle_solution = time_to_wait * id;
    println!("Part 1: earliest bus {} at {} ({} minute wait). puzzle solution is {}", id, time, time_to_wait, puzzle_solution);

    Ok(())
}
