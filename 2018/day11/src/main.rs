#![allow(unused)]

struct Grid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl <T> Grid<T> where T: Default + Clone {
    fn new(w: usize, h: usize) -> Grid<T> {
        Grid {
            data: vec![Default::default(); (w * h) as usize],
            width: w,
            height: h,
        }
    }
}

impl <T> Grid<T> {
    fn get(&self, x: usize, y: usize) -> &T {
        let pos = (y * self.width) + x;
        &self.data[pos as usize]
    }

    fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
        let pos = (y * self.width) + x;
        &mut self.data[pos as usize]
    }
}

/*
    Each fuel cell has a coordinate ranging from 1 to 300 in both the X (horizontal) and Y (vertical) direction.
    In X,Y notation, the top-left cell is 1,1, and the top-right cell is 300,1.

    The interface lets you select any 3x3 square of fuel cells. To increase your chances of getting to your
    destination, you decide to choose the 3x3 square with the largest total power.

    The power level in a given fuel cell can be found through the following process:
    - Find the fuel cell's rack ID, which is its X coordinate plus 10.
    - Begin with a power level of the rack ID times the Y coordinate.
    - Increase the power level by the value of the grid serial number (your puzzle input).
    - Set the power level to itself multiplied by the rack ID.
    - Keep only the hundreds digit of the power level (so 12345 becomes 3; numbers with no hundreds digit become 0).
    - Subtract 5 from the power level.
*/
fn calculate_power_grid(serial_number: i32) -> Grid<i32> {
    let (width, height) = (300, 300);
    let mut grid = Grid::new(width, height);
    for x in 0..width {
        for y in 0..height {
            let x_coord = (x + 1) as i32;
            let y_coord = (y + 1) as i32;
            let rack_id = x_coord + 10; 
            let power = ((rack_id * y_coord) + serial_number) * rack_id;
            let hundreds_digit = (power.abs() / 100) % 100 % 10;
            let power_2 = hundreds_digit - 5;
            *grid.get_mut(x as usize, y as usize) = power_2;
        }
    }
    grid
}

fn main() {
    fn test_power_grid(serial_number: i32, coords: (usize, usize)) {
        let grid = calculate_power_grid(serial_number);
        let (x, y) = coords;
        let power = *grid.get(x - 1, y - 1);
        println!("Grid {} at position {:?}: {}", serial_number, coords, power);
    }

    test_power_grid(8, (3, 5));
    test_power_grid(57, (122, 79));
    test_power_grid(39, (217, 196));
    test_power_grid(71, (101, 153));

    /*
    --- Part One ---
    What is the X,Y coordinate of the top-left fuel cell of the 3x3 square with the largest total power?

    Your puzzle input is 1723.
    */
    fn test_cell_max_power(grid: &Grid<i32>, cell_size: usize) -> ((usize, usize), i32) {
        let mut highest_power = 0;
        let mut highest_power_coords = (0, 0);
        for x in 0..(grid.width - (cell_size - 1)) {
            for y in 0..(grid.height - (cell_size - 1)) {
                // Total up this NxN grid
                let mut total = 0;
                for sx in 0..cell_size {
                    for sy in 0..cell_size {
                        total += grid.get(x + sx, y + sy);
                    }
                }
                if total > highest_power {
                    highest_power = total;
                    highest_power_coords = (x + 1, y + 1);
                }
            }
        }
        (highest_power_coords, highest_power)
    }
    
    let grid = calculate_power_grid(1723);
    let cell_size = 3;
    let (coords, power) = test_cell_max_power(&grid, cell_size);

    println!("Part 1 result: {:?} with total power {}", coords, power);

    /*
    --- Part Two ---
    You discover a dial on the side of the device; it seems to let you select a square
    of any size, not just 3x3. Sizes from 1x1 to 300x300 are supported.

    Realizing this, you now must find the square of any size with the largest total power.
    Identify this square by including its size as a third parameter after the top-left
    coordinate: a 9x9 square with a top-left corner of 3,5 is identified as 3,5,9.

    For example:
    - For grid serial number 18, the largest total square (with a total power of 113) is 16x16 and has a top-left corner of 90,269, so its identifier is 90,269,16.
    - For grid serial number 42, the largest total square (with a total power of 119) is 12x12 and has a top-left corner of 232,251, so its identifier is 232,251,12.

    What is the X,Y,size identifier of the square with the largest total power?
    */

    let mut highest_power = 0;
    let mut highest_power_coords = (0, 0);
    let mut highest_power_cell_size = 0;

    for cell_size in 1..=30 {
        let (coords, power) = test_cell_max_power(&grid, cell_size);
        if power > highest_power {
            highest_power = power;
            highest_power_coords = coords;
            highest_power_cell_size = cell_size;
        }
    }

    println!("Part 2 result: {:?} with total power {} and cell size {}", highest_power_coords, highest_power, highest_power_cell_size);
}
