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
    type WH = (isize, isize);
    type XY = (isize, isize);

    let input = read_input("input.txt")?;
    let w = input[0].len() as isize;
    let h = input.len() as isize;
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

    fn cardinal_offsets() -> impl Iterator<Item=XY> {
        (-1..=1).flat_map(|x| (-1..=1).map(move |y| (x, y))).filter(|xy| *xy != (0, 0))
    }

    fn grid_positions((w, h): WH) -> impl Iterator<Item=XY> {
        (0..w).flat_map(move |x| (0..h).map(move |y| (x, y)))
    }

    fn iterate_until_stable(grid: &[Pos], mut calc_next: impl FnMut(&[Pos], &mut [Pos])) -> (usize, usize) {
        let mut iteration_count = 0;
        let mut curr = grid.to_vec();
        let mut next = curr.clone();
        loop {
            calc_next(&curr, &mut next);
            if next == curr {
                // Grid is stable
                break;
            }
            std::mem::swap(&mut curr, &mut next);
            iteration_count += 1;
        }
        let occupied_count = curr.iter().filter(|p| **p == Pos::Occupied).count();
        (iteration_count, occupied_count)
    }

    fn get_pos_and_neighbors_part1(grid: &[Pos], (w, h): WH, (x, y): XY) -> (Pos, usize) {
        let mut count = 0;
        for (off_x, off_y) in cardinal_offsets() {
            let x = x + off_x;
            let y = y + off_y;
            if x < 0 || x >= w || y < 0 || y >= h {
                continue;
            }
            let p = grid[(y * w + x) as usize];
            if p == Pos::Occupied {
                count += 1;
            }
        }
        let p = grid[(y * w + x) as usize];
        (p, count)
    }

    let (iteration_count, occupied_count) = iterate_until_stable(&grid, |curr, next| {
        for (x, y) in grid_positions((w, h)) {
            let b = match get_pos_and_neighbors_part1(&curr, (w, h), (x, y)) {
                (Pos::Empty, o) if o == 0 => Pos::Occupied,
                (Pos::Occupied, o) if o >= 4 => Pos::Empty,
                (p, _) => p
            };
            next[(y * w + x) as usize] = b;
        }
    });

    println!("Part 1: {} iterations until stable, {} occupied seats", iteration_count, occupied_count);

    fn get_pos_and_neighbors_part2(grid: &[Pos], (w, h): WH, (x, y): XY) -> (Pos, usize) {
        let mut count = 0;
        for (off_x, off_y) in cardinal_offsets() {
            let mut x = x;
            let mut y = y;
            loop {
                x += off_x;
                y += off_y;
                if x < 0 || x >= w || y < 0 || y >= h {
                    break;
                }
                let p = grid[(y * w + x) as usize];
                if p == Pos::Occupied {
                    count += 1;
                    break;
                }
                if p == Pos::Empty {
                    break;
                }
            }
        }
        let p = grid[(y * w + x) as usize];
        (p, count)
    }

    let (iteration_count, occupied_count) = iterate_until_stable(&grid, |curr, next| {
        for (x, y) in grid_positions((w, h)) {
            let b = match get_pos_and_neighbors_part2(&curr, (w, h), (x, y)) {
                (Pos::Empty, o) if o == 0 => Pos::Occupied,
                (Pos::Occupied, o) if o >= 5 => Pos::Empty,
                (p, _) => p
            };
            next[(y * w + x) as usize] = b;
        }
    });

    println!("Part 2: {} iterations until stable, {} occupied seats", iteration_count, occupied_count);

    Ok(())
}
