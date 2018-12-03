extern crate util;

use util::{ read_input };
use util::error::{ AppErr, fail };

fn main() -> Result<(), AppErr> {
    /*
    --- Part One ---
    The whole piece of fabric they're working on is a very large square - at least 1000 inches on each side.

    Each Elf has made a claim about which area of fabric would be ideal for Santa's suit. All claims have an ID and consist of a single rectangle with edges parallel to the edges of the fabric. Each claim's rectangle is defined as follows:

    The number of inches between the left edge of the fabric and the left edge of the rectangle.
    The number of inches between the top edge of the fabric and the top edge of the rectangle.
    The width of the rectangle in inches.
    The height of the rectangle in inches.
    A claim like #123 @ 3,2: 5x4 means that claim ID 123 specifies a rectangle 3 inches from the left edge, 2 inches from the top edge, 5 inches wide, and 4 inches tall. Visually, it claims the square inches of fabric represented by # (and ignores the square inches of fabric represented by .) in the diagram below:

    ...........
    ...........
    ...#####...
    ...#####...
    ...#####...
    ...#####...
    ...........
    ...........
    ...........

    The problem is that many of the claims overlap, causing two or more claims to cover part of the same areas. For example, consider the following claims:

    #1 @ 1,3: 4x4
    #2 @ 3,1: 4x4
    #3 @ 5,5: 2x2
    Visually, these claim the following areas:

    ........
    ...2222.
    ...2222.
    .11XX22.
    .11XX22.
    .111133.
    .111133.
    ........
    The four square inches marked with X are claimed by both 1 and 2. (Claim 3, while adjacent to the others, does not overlap either of them.)

    If the Elves all proceed with their own plans, none of them will have enough fabric. How many square inches of fabric are within two or more claims?
    */
    #[derive(Debug)]
    struct Claim {
        pub id: String,
        pub top_left: (usize, usize),
        pub size: (usize, usize),
    }

    fn parse_claim(line: &str) -> Result<Claim, AppErr> {
        // #1 @ 45,64: 22x22
        let _fail = || fail("Unable to parse claim");
        let mut read = {
            let mut parts = line.split(' ');
            move || parts.next().ok_or_else(_fail)
        };
        let id = read()?.trim_start_matches('#');
        read()?; // @
        let mut top_left = read()?.trim_end_matches(':').split(',').map(str::parse);
        let mut size = read()?.split('x').map(str::parse);
        Ok(Claim {
            id: id.to_string(),
            top_left: (
                top_left.next().ok_or_else(_fail)??,
                top_left.next().ok_or_else(_fail)??
            ),
            size: (
                size.next().ok_or_else(_fail)??,
                size.next().ok_or_else(_fail)??
            )
        })
    }

    let claims =
        read_input("input.txt")?
            .into_iter()
            .map(|line| parse_claim(&line))
            .collect::<Result<Vec<_>, _>>()?;

    const SIZE: usize = 1000;
    let mut grid = vec![0_u32; SIZE * SIZE];

    for claim in &claims {
        let (x, y) = claim.top_left;
        let (w, h) = claim.size;
        for _x in x..(x + w) {
            for _y in y..(y + h) {
                let index = (_y * SIZE) + _x;
                grid[index] += 1;
            }
        }
    }

    let result = grid.iter().filter(|i| **i > 1).count();

    println!("Part 1 result: {}", result);

    /*
    --- Part Two ---
    Amidst the chaos, you notice that exactly one claim doesn't overlap by even a single square inch of fabric with any other claim. If you can somehow draw attention to it, maybe the Elves will be able to make Santa's suit after all!

    For example, in the claims above, only claim 3 is intact after all claims are made.

    What is the ID of the only claim that doesn't overlap?
    */

    let mut id_of_lonely_claim = None;

    'outer: for claim in &claims {
        let (x, y) = claim.top_left;
        let (w, h) = claim.size;
        for _x in x..(x + w) {
            for _y in y..(y + h) {
                let index = (_y * SIZE) + _x;
                if grid[index] != 1 {
                    // Not lonely :(
                    continue 'outer;
                }
            }
        }
        id_of_lonely_claim = Some(&claim.id);
    }

    let result = id_of_lonely_claim.ok_or(fail("Couldn't find the lonely claim"))?;

    println!("Part 2 result: {}", result);

    Ok(())
}
