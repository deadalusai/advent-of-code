extern crate util;

use std::collections::{ HashMap };

use util::{ read_input };
use util::error::{ AppErr };

fn main() -> Result<(), AppErr> {
    /*
    --- Part One ---
    
        Given a grid of trees (#) and open spaces (.) which repeats infinitely to right.
        Input is the first chunk of this repeating pattern.

        Beginning from the top-left open space, start by counting all the trees
        you would encounter for the slope right 3, down 1 - how many trees would
        you encounter?
    */

    #[derive(Debug, PartialEq, Eq)]
    enum Pos { Gap, Tree }

    // NOTE: Top left is 1, 1

    let input =
        read_input("input.txt")?.iter()
            .enumerate()
            .flat_map(|(y, line)| line.char_indices()
                .map(move |(x, c)| {
                    let p = match c {
                        '.' => Pos::Gap,
                        '#' => Pos::Tree,
                        _ => return Err(AppErr::from_debug("unexpected input", &(x, y, c))),
                    };
                    Ok(((x, y), p))
                }))
            .collect::<Result<HashMap<_, _>, AppErr>>()?;

    fn count_trees_on_slope(input: &HashMap<(usize, usize), Pos>, slope: (usize, usize)) -> u32 {
        let (x_max, y_max) = *input.keys().max().expect("zero-sized grid");
        let (x_delta, y_delta) = slope;

        let (mut x_actual, mut y_actual) = (0, 0);
        let mut tree_count = 0;
        loop {
            // count the tree
            let x = x_actual % (x_max + 1);
            let y = y_actual;
            if Pos::Tree == *input.get(&(x, y)).unwrap_or(&Pos::Gap) {
                tree_count += 1;
            }
            // move
            x_actual += x_delta;
            y_actual += y_delta;
            // check for end of slope
            if y_actual > y_max {
                break;
            }
        }

        tree_count
    }

    let part_1_result = count_trees_on_slope(&input, (3, 1));

    println!("Part 1: {} trees", part_1_result);

    /*
    --- Part Two ---
    Time to check the rest of the slopes - you need to minimize the probability of a sudden
    arboreal stop, after all.

    Determine the number of trees you would encounter if, for each of the following slopes,
    you start at the top-left corner and traverse the map all the way to the bottom:

        Right 1, down 1.
        Right 3, down 1. (This is the slope you already checked.)
        Right 5, down 1.
        Right 7, down 1.
        Right 1, down 2.

    In the above example, these slopes would find 2, 7, 3, 4, and 2 tree(s) respectively;
    multiplied together, these produce the answer 336.

    What do you get if you multiply together the number of trees encountered on each of the listed slopes?
    */

    let part_2_result =
        count_trees_on_slope(&input, (1, 1)) *
        count_trees_on_slope(&input, (3, 1)) *
        count_trees_on_slope(&input, (5, 1)) *
        count_trees_on_slope(&input, (7, 1)) *
        count_trees_on_slope(&input, (1, 2));

    println!("Part 2: {} trees", part_2_result);

    Ok(())
}
