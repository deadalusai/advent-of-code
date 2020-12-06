extern crate util;

use util::{ read_input, ConsumeIterator };
use util::error::{ AppErr };

fn main() -> Result<(), AppErr> {
    /*
    --- Part One ---
    
    Find your seat on the plane by binary search.
        - 128 rows
        - 8 columns

    Instead of zones or groups, this airline uses binary space partitioning to seat people.
    A seat might be specified like FBFBBFFRLR, where F means "front", B means "back", L means "left", and R means "right".

    The first 7 characters will either be F or B;
    The last three characters will be either L or R;

    So, decoding FBFBBFFRLR reveals that it is the seat at row 44, column 5.

    Every seat also has a unique seat ID: multiply the row by 8, then add the column.
    In this example, the seat has ID 44 * 8 + 5 = 357.

    As a sanity check, look through your list of boarding passes.
    What is the highest seat ID on a boarding pass?
    */

    #[derive(Debug, PartialEq, Eq)]
    enum PartitionDesc { Upper, Lower }
    #[derive(Debug)]
    struct SeatLocationDesc {
        row: [PartitionDesc; 7],
        col: [PartitionDesc; 3],
    }
    #[derive(Debug)]
    struct SeatLocation {
        row: u32,
        col: u32,
        id: u32,
    }

    fn try_parse_seat_location_desc(s: &str) -> Result<SeatLocationDesc, AppErr> {
        fn parse_rowpar(c: char) -> Result<PartitionDesc, AppErr> {
            match c {
                'F' => Ok(PartitionDesc::Lower),
                'B' => Ok(PartitionDesc::Upper),
                 x  => Err(AppErr::from_debug("invalid row partition", &x)),
            }
        }
        fn parse_colpar(c: char) -> Result<PartitionDesc, AppErr> {
            match c {
                'L' => Ok(PartitionDesc::Lower),
                'R' => Ok(PartitionDesc::Upper),
                 x  => Err(AppErr::from_debug("invalid col partition", &x)),
            }
        }
        let mut chars = s.chars();
        Ok(SeatLocationDesc {
            row: [
                parse_rowpar(chars.take_next()?)?,
                parse_rowpar(chars.take_next()?)?,
                parse_rowpar(chars.take_next()?)?,
                parse_rowpar(chars.take_next()?)?,
                parse_rowpar(chars.take_next()?)?,
                parse_rowpar(chars.take_next()?)?,
                parse_rowpar(chars.take_next()?)?,
            ],
            col: [
                parse_colpar(chars.take_next()?)?,
                parse_colpar(chars.take_next()?)?,
                parse_colpar(chars.take_last()?)?,
            ],
        })
    }

    fn calculate_seat_location(loc: &SeatLocationDesc) -> SeatLocation {
        fn partition(min: u32, max: u32, take_upper: bool) -> (u32, u32) {
            let range = max - min;
            let mid = min + (range / 2);
            if take_upper { (mid + 1, max) } else { (min, mid) }
        }
        fn reduce(mut min: u32, mut max: u32, path: &[PartitionDesc]) -> u32 {
            for part in path {
                let (new_min, new_max) = partition(min, max, *part == PartitionDesc::Upper);
                min = new_min;
                max = new_max;
            }
            assert_eq!(min, max);
            min
        }
        let row = reduce(0, 127, &loc.row);
        let col = reduce(0, 7, &loc.col);
        SeatLocation {
            row,
            col,
            id: (row * 8) + col
        }
    }

    let seat_locations =
        read_input("input.txt")?
            .iter()
            .map(|line| {
                let desc = try_parse_seat_location_desc(line)?;
                Ok(calculate_seat_location(&desc))
            })
            .collect::<Result<Vec<_>, AppErr>>()?;


    let highest_seat_id = seat_locations.iter()
        .map(|loc| loc.id)
        .max();

    println!("Part 1: highest seat id = {:?}", highest_seat_id);

    /*
    --- Part Two ---

    Ding! The "fasten seat belt" signs have turned on.
    Time to find your seat.

    It's a completely full flight, so your seat should be the only missing
    boarding pass in your list. However, there's a catch: some of the seats
    at the very front and back of the plane don't exist on this aircraft,
    so they'll be missing from your list as well.

    Your seat wasn't at the very front or back, though; the seats
    with IDs +1 and -1 from yours will be in your list.

    What is the ID of your seat?
    */

    // Sort the collection by ID and scan for non-contiguous IDs to identify the empty seat.
    let mut seat_locations = seat_locations;
    seat_locations.sort_by_key(|loc| loc.id);

    let boundary_seats =
            Iterator::zip(
                seat_locations.iter(),
                seat_locations.iter().skip(1)
            )
            .find(|(a, b)| a.id != (b.id - 1));

    if let Some((a, b)) = boundary_seats {
        println!("Part 2: found a gap in the IDs at {:?}", (a.id, b.id));
    }
    else {
        println!("Part 2: no gap found");
    }

    Ok(())
}
