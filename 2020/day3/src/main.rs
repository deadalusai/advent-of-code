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

    let (x_max, y_max) = *input.keys().max().expect("zero-sized grid");
    let (mut x_actual, mut y_actual) = (0, 0);
    let mut tree_count = 0;

    loop {
        // count the trees
        let x = x_actual % (x_max + 1);
        let y = y_actual;
        let pos = input.get(&(x, y)).unwrap_or(&Pos::Gap);
        if pos == &Pos::Tree {
            tree_count += 1;
        }
        // move right 3, down 1
        x_actual += 3;
        y_actual += 1;
        // check for end of slope
        if y_actual > y_max {
            break;
        }
    }

    println!("Part 1: {} trees", tree_count);

    Ok(())
}
