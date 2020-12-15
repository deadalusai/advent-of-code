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

    let earliest_time_to_leave = input[0].parse::<u64>()?;
    let bus_ids = input[1].split(',')
        .map(|id| match id {
            "x" => Ok(None),
            id => id.parse::<u64>().map(|id| Some(id)),
        })
        .collect::<Result<Vec<_>, _>>()?;

    fn bus_timetable(bus_id: u64) -> impl Iterator<Item=u64> {
        (0..).map(move |i| i * bus_id)
    }

    let earliest_bus = bus_ids
        .iter()
        .filter_map(|o| *o)
        .filter_map(|id| {
            let first_time =
                bus_timetable(id)
                    .take(1_000_000)
                    .find(|time| *time >= earliest_time_to_leave)?;
            Some((id, first_time))
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

    /*
    --- Part Two ---

    What is the earliest timestamp such that all of the listed bus IDs
    depart at offsets matching their positions in the list?
    */

    struct Timetable {
        id: u64,
        offset: u64,
        time: u64,
    }

    impl Timetable {
        fn next(&mut self) {
            self.time += self.id;
        }
    }

    let mut bus_timetables = bus_ids
        .iter()
        .enumerate()
        .filter_map(|(offset, &id)| {
            let id = id?;
            Some(Timetable { id, offset: offset as u64, time: 0 })
        })
        .collect::<Vec<_>>();

    // hack - hint indicates the result will likely be higher than 100_000_000_000_000
    {
        let hint = 100_000_000_000_000;
        let tt = &mut bus_timetables[0];
        tt.time = (hint / tt.id) * tt.id;
    }

    'outer: loop {
        let base_time = {
            let tt = &mut bus_timetables[0];
            tt.next();
            tt.time
        };
        for i in 1..bus_timetables.len() {
            let tt = &mut bus_timetables[i];
            let target_time = base_time + tt.offset; 
            while tt.time < target_time {
                tt.next();
            }
            if tt.time != target_time {
                continue 'outer;
            }
        }
        break;
    }

    println!("{:?}", bus_timetables.iter().map(|t| (t.id, t.time)).collect::<Vec<_>>());

    let earliest = &bus_timetables[0];
    println!("Part 2: earliest bus {} at {}", earliest.id, earliest.time);

    Ok(())
}
