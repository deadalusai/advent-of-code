extern crate util;

use std::collections::HashSet;

use util::read_input;
use util::error::AppErr;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Pos { y: usize, x: usize }

impl Pos {
    fn surrounding(&self) -> impl Iterator<Item=Pos> {
        let pos = self.clone();
        let offsets = [-1, 0, 1].into_iter().flat_map(|x| [-1, 0, 1].into_iter().map(move |y| (x, y)));
        offsets.filter_map(move |(x, y)| {
            match (x, y) {
                (0,  0)  => None,
                (ox, _)  if ox < 0 && pos.x == 0 => None,
                (_,  oy) if oy < 0 && pos.y == 0 => None,
                (ox, oy) => Some(Pos {
                    x: (pos.x as isize + ox) as usize,
                    y: (pos.y as isize + oy) as usize,
                })
            }
        })
    }

    fn left(&self) -> Option<Pos> {
        match self.x {
            0 => { None },
            x => Some(Pos { x: x - 1, y: self.y }),
        }
    }

    fn right(self) -> Pos {
        Pos { x: self.x + 1, y: self.y }
    }
}

fn main() -> Result<(), AppErr> {
    /*
        --- Part One ---
        Any number adjacent to a symbol, even diagonally, is a "part number" and should be included in your sum.
        Periods (.) do not count as a symbol.

        What is the sum of all of the part numbers in the engine schematic?
    */

    type Grid = Vec<Vec<char>>;

    let grid = read_input("input.txt")?
        .iter()
        .enumerate()
        .map(|(_, row)| {
            row.chars()
                .enumerate()
                .map(|(_, c)| c)
                .collect::<Vec<_>>()
        })
        .collect::<Grid>();

    // Iterate through the grid, find symbols and sum up their surrounding numbers

    fn digit_at(grid: &Grid, pos: Pos) -> Option<u32> {
        grid.get(pos.y)
            .and_then(|row| row.get(pos.x))
            .and_then(|c| c.to_digit(10))
    }

    fn surrounding_part_numbers<'a> (pos: Pos, grid: &'a Grid) -> impl Iterator<Item=(Pos, u32)> + 'a {
        pos.surrounding()
            .filter_map(|p| {
                // Is this part of a number?
                if digit_at(grid, p).is_none() {
                    return None;
                }
                // Find the start of the number
                let mut p1 = p;
                while let Some(n) = p1.left().filter(|&n| digit_at(grid, n).is_some()) {
                    p1 = n
                }
                // Parse it
                let mut num = 0;
                let mut p2 = p1;
                while let Some(d) = digit_at(grid, p2) {
                    num *= 10;
                    num += d;
                    p2 = p2.right();
                }
                Some((p1, num))
            })
    }

    let mut seen = HashSet::new();
    let mut sum = 0_u32;

    for (y, row) in grid.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c != '.' && !c.is_numeric() {
                // Found a symbol.
                // Sum up any surrounding part numbers
                let pos = Pos { x, y };
                sum += surrounding_part_numbers(pos, &grid)
                    // Don't double-count
                    .filter_map(|(pos, n)| if seen.insert(pos) { Some(n) } else { None })
                    .sum::<u32>();
            }
        }
    }

    println!("Part 1: {}", sum);

    Ok(())
}
