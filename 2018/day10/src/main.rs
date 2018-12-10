extern crate util;
extern crate regex;

use util::{ read_input };

#[derive(Copy, Clone, Debug)]
enum State { On, Off }

struct Grid {
    data: Vec<State>,
    width: isize,
    height: isize,
}

impl Grid {
    fn new(w: isize, h: isize) -> Grid {
        Grid {
            data: vec![State::Off; (w * h) as usize],
            width: w,
            height: h,
        }
    }

    fn get(&self, x: isize, y: isize) -> &State {
        let pos = (y * self.width) + x;
        &self.data[pos as usize]
    }

    fn get_mut(&mut self, x: isize, y: isize) -> &mut State {
        let pos = (y * self.width) + x;
        &mut self.data[pos as usize]
    }
}

use std::fmt;
impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        writeln!(f, "+{}+", str::repeat("-", self.width as usize));
        for y in 0..self.height {
            write!(f, "|");
            for x in 0..self.width {
                write!(f, "{}", match self.get(x, y) { State::On => '#', State::Off => ' ' });
            }
            writeln!(f, "|");
        }
        writeln!(f, "+{}+", str::repeat("-", self.width as usize));
        Ok(())
    }
}

fn main() {

    struct Vector {
        position: (isize, isize),
        velocity: (isize, isize),
    }

    let input_matcher = regex::Regex::new(r"position=<\s*(-?\d+),\s*(-?\d+)> velocity=<\s*(-?\d+),\s*(-?\d+)>").unwrap();
    let parse_input = move |s: &str| {
        let captures = input_matcher.captures(s).unwrap();
        Vector {
            position: (
                captures.get(1).unwrap().as_str().parse().unwrap(),
                captures.get(2).unwrap().as_str().parse().unwrap()
            ),
            velocity: (
                captures.get(3).unwrap().as_str().parse().unwrap(),
                captures.get(4).unwrap().as_str().parse().unwrap()
            ),
        }
    };

    let mut vectors =
        read_input("input.txt").unwrap().iter()
            .map(|s| parse_input(&s))
            .collect::<Vec<_>>();

    fn tick_and_find_bounds(vectors: &mut Vec<Vector>) -> ((isize, isize), (isize, isize)) {
        let mut min_x = isize::max_value();
        let mut max_x = isize::min_value();
        let mut min_y = isize::max_value();
        let mut max_y = isize::min_value();
        for v in vectors.iter_mut() {
            let (x, y) = v.position;
            let (dx, dy) = v.velocity;
            min_x = x.min(min_x);
            max_x = x.max(max_x);
            min_y = y.min(min_y);
            max_y = y.max(max_y);
            v.position = (x + dx, y + dy);
        }
        ((min_x, max_x),
         (min_y, max_y))
    }

    let (width, height) = (200, 30);
    let mut grid = Grid::new(width, height);
    let mut iteration = 0;

    loop {
        let ((min_x, max_x), (min_y, max_y)) = tick_and_find_bounds(&mut vectors);
        iteration += 1;

        // Are all the stars within our bounding box?
        if (max_x - min_x).abs() > width || (max_y - min_y).abs() > height {
            continue;
        }

        // Update and render the grid
        for x in min_x..max_x {
            for y in min_y..max_y {
                let v = vectors.iter().filter(|v| v.position == (x, y)).next();
                *grid.get_mut(x - min_x, y - min_y) = if v.is_some() { State::On } else { State::Off };
            }
        }

        println!("Iteration: {}", iteration);
        println!("{:?}", grid);
        
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}
