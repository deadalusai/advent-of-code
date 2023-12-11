extern crate util;

use std::collections::HashSet;

use util::read_input;
use util::error::AppErr;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Pos { y: usize, x: usize }

impl Pos {
    fn up(&self) -> Option<Pos> {
        match self.y {
            0 => None,
            y => Some(Pos { x: self.x, y: y - 1 }),
        }
    }

    fn down(&self) -> Pos {
        Pos { x: self.x, y: self.y + 1 }
    }

    fn left(&self) -> Option<Pos> {
        match self.x {
            0 => None,
            x => Some(Pos { x: x - 1, y: self.y }),
        }
    }

    fn right(self) -> Pos {
        Pos { x: self.x + 1, y: self.y }
    }

    fn surrounding(&self) -> impl Iterator<Item=Pos> {
        let candidates = [
            self.up().and_then(|p| p.left()),
            self.up(),
            self.up().map(|p| p.right()),
            self.left(),
            Some(self.right()),
            self.down().left(),
            Some(self.down()),
            Some(self.down().right()),
        ];
        candidates.into_iter().flatten()
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

    fn surrounding_part_numbers<'a>(pos: Pos, grid: &'a Grid) -> impl Iterator<Item=u32> + 'a {
        let mut seen = HashSet::with_capacity(8);
        pos.surrounding().filter_map(move |p| {
            // Is this part of a number?
            if digit_at(grid, p).is_none() {
                return None;
            }
            // Find the start of the number
            let mut p1 = p;
            while let Some(n) = p1.left().filter(|&n| digit_at(grid, n).is_some()) {
                p1 = n
            }
            // Have we seen this number?
            if !seen.insert(p1) {
                return None;
            }
            // Parse it
            let mut num = 0;
            let mut p2 = p1;
            while let Some(d) = digit_at(grid, p2) {
                num *= 10;
                num += d;
                p2 = p2.right();
            }
            Some(num)
        })
    }

    let mut sum = 0_u32;

    for (y, row) in grid.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c != '.' && !c.is_numeric() {
                // Found a symbol.
                // Sum up any surrounding part numbers
                sum += surrounding_part_numbers(Pos { x, y }, &grid).sum::<u32>();
            }
        }
    }

    println!("Part 1: {}", sum);

    /*
        --- Part 2 ---
        A gear is any * symbol that is adjacent to exactly two part numbers.
        Its gear ratio is the result of multiplying those two numbers together.

        What is the sum of all of the gear ratios in your engine schematic?
    */

    let mut sum = 0_u32;

    for (y, row) in grid.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == '*' {
                // Found a gear
                let mut surr = surrounding_part_numbers(Pos { x, y }, &grid);
                match (surr.next(), surr.next(), surr.next()) {
                    // Sum up gears with exactly two part numbers only
                    (Some(a), Some(b), None) => sum += a * b,
                    _ => continue,
                }
            }
        }
    }

    println!("Part 2: {}", sum);

    Ok(())
}
