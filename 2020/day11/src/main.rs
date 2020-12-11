extern crate util;

use util::{ read_input };
use util::error::{ AppErr };

fn main() -> Result<(), AppErr> {
    /*
    --- Part One ---

    It's conway's game of life.
    Rules:
    - If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
    - If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat becomes empty.
    - Otherwise, the seat's state does not change.

    Simulate your seating area by applying the seating rules repeatedly until no seats change state.
    How many seats end up occupied?
    */

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum Pos { Empty, Occupied, Floor }

    let input = read_input("input.txt")?;
    let width = input[0].len() as isize;
    let height = input.len() as isize;
    let grid = input
        .iter()
        .flat_map(|line| line.chars().map(|c|
            match c {
                '#' => Pos::Occupied,
                'L' => Pos::Empty,
                 _  => Pos::Floor
            }
        ))
        .collect::<Vec<_>>();

    fn get_pos_and_neighbors((x, y): (isize, isize), (w, h): (isize, isize), grid: &[Pos]) -> (Pos, usize) {
        let mut count = 0;
        for &off_x in &[-1, 0, 1] {
            for &off_y in &[-1, 0, 1] {
                if off_x == 0 && off_y == 0 {
                    continue;
                }
                let x = x + off_x;
                let y = y + off_y;
                if x < 0 || x >= w || y < 0 || y >= h {
                    continue;
                }
                if let Pos::Occupied = grid[(y * w + x) as usize] {
                    count += 1;
                }
            }
        }
        let p = grid[(y * w + x) as usize];
        (p, count)
    }

    fn get_next_state((w, h): (isize, isize), grid: &Vec<Pos>) -> Vec<Pos> {
        let mut next = grid.clone();
        for x in 0..w {
            for y in 0..h {
                let b = match get_pos_and_neighbors((x, y), (w, h), grid) {
                    (Pos::Empty, o) if o == 0 => Pos::Occupied,
                    (Pos::Occupied, o) if o >= 4 => Pos::Empty,
                    (p, _) => p
                };
                next[(y * w + x) as usize] = b;
            }
        }
        next
    }

    let mut count = 0;
    let mut grid = grid;
    loop {
        let next = get_next_state((width, height), &grid);
        if next == grid {
            break;
        }
        grid = next;
        count += 1;
    }
    let occupied_count = grid.iter().filter(|p| **p == Pos::Occupied).count();

    println!("Part 1: {} iterations until stable, {} occupied seats", count, occupied_count);

    Ok(())
}
