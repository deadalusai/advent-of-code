#![feature(iter_arith, io)]

extern crate gol;

use std::env::args;
use std::fs::File;
use std::io::{ Read };

use gol::grid::{ Grid, Cell };
use gol::world::{ World };
use gol::rules::terminal_neighbours;

fn open_file() -> File {
    let filename = args().skip(1).next().expect("usage: day18 {input filename}");
    File::open(filename).expect("Error opening input")
}

fn read_input(file: File) -> Grid {

    let char_to_cell = |c| {
        match c {
            '#' => Some(Cell::Live),
            '.' => Some(Cell::Dead),
             _  => None
        }
    };

    let cells: Vec<Cell> =
        file.chars()
            .map(|c| c.expect("Error reading line"))
            .filter_map(|c| char_to_cell(c))
            .collect();

    //Expecting a 100x100 grid
    Grid::from_raw(100, 100, cells)
}

fn main() {

    let iterations = 100;
    let initial_grid = read_input(open_file());

    let mut world = World::new(initial_grid);

    // Simulating a terminal world
    world.set_neighbours(terminal_neighbours);

    for _ in 0 .. iterations {
        world.step_mut();
    }

    let live_cell_count: usize =
        world.iter_rows()
             .map(|row| row.iter().filter(|c| c.is_live()).count())
             .sum();

    println!("Lights on after {} iterations: {}", iterations, live_cell_count);
}
