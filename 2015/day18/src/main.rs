#![feature(io)]

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

    // Part 2 - Keep the four corners turned on

    for _ in 0 .. iterations {
        reset_corners(&mut world);
        world.step_mut();
        reset_corners(&mut world);
    }

    let live_cell_count: usize =
        world.grid()
             .iter_cells()
             .filter(|&(_, _, cell)| cell.is_live())
             .count();

    println!("Lights on after {} iterations: {}", iterations, live_cell_count);
}

fn reset_corners(world: &mut World) {
    // Find the x, y coordinates of the bottom right corner
    let x_last = world.width() - 1;
    let y_last = world.height() - 1;

    let g = world.grid_mut();

    // Resurrect the four corner cells
    g.set_cell(0,      0,      Cell::Live);
    g.set_cell(x_last, 0,      Cell::Live);
    g.set_cell(0,      y_last, Cell::Live);
    g.set_cell(x_last, y_last, Cell::Live);
}
