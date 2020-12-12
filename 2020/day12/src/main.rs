extern crate util;

use util::{ read_input };
use util::error::{ AppErr };

fn main() -> Result<(), AppErr> {
    /*
    --- Part One ---



    Figure out where the navigation instructions lead.
    What is the Manhattan distance between that location and the ship's starting position?
    */

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum Facing { North, South, East, West };

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum Dir { Left, Right }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum Step {
        Move(Facing, i64),
        Turn(Dir, i64),
        Forward(i64),
    }

    fn parse_step(line: &str) -> Result<Step, AppErr> {
        let c = line.chars().next().ok_or(AppErr::new("parse err", "expected direction instruction"))?;
        let num = line[1..].parse::<i64>()?;
        match c {
            'N' => Ok(Step::Move(Facing::North, num)),
            'S' => Ok(Step::Move(Facing::South, num)),
            'E' => Ok(Step::Move(Facing::East, num)),
            'W' => Ok(Step::Move(Facing::West, num)),
            'L' => Ok(Step::Turn(Dir::Left, num)),
            'R' => Ok(Step::Turn(Dir::Right, num)),
            'F' => Ok(Step::Forward(num)),
             c  => Err(AppErr::new("parse error", &format!("unexpected step {}", c))),
        }
    }

    let steps = read_input("input.txt")?
        .iter()
        .map(|line| parse_step(line))
        .collect::<Result<Vec<_>, _>>()?;
    
    let mut facing = Facing::East;
    let mut x = 0;
    let mut y = 0;

    fn d_move(facing: Facing, distance: i64) -> (i64, i64) {
        match facing {
            Facing::North => (0, distance),
            Facing::South => (0, -distance),
            Facing::East => (distance, 0),
            Facing::West => (-distance, 0),
        }
    }

    fn d_facing(facing: Facing, angle: i64) -> Facing {
        let turn = match facing {
            Facing::North => 0,
            Facing::East  => 90,
            Facing::South => 180,
            Facing::West  => 270,
        };
        match (angle + turn) % 360 {
            0          => Facing::North,
            180 | -180 => Facing::South,
            90  | -270 => Facing::East,
            -90 |  270 => Facing::West,
            a   => panic!("unexpected angle {}", a),
        }
    }

    for step in &steps {
        match step {
            Step::Move(f, d) => {
                let (dx, dy) = d_move(*f, *d);
                x += dx;
                y += dy;
            },
            Step::Forward(d) => {
                let (dx, dy) = d_move(facing, *d);
                x += dx;
                y += dy;
            },
            Step::Turn(direction, angle) => {
                facing = d_facing(facing, match direction {
                    Dir::Left  => -angle,
                    Dir::Right => *angle,
                });
            },
        }
    }

    println!("Part 1: position is {:?}, sum is {}", (x, y), (x.abs() + y.abs()));

    /*
    --- Part Two ---

    Figure out where the navigation instructions actually lead.
    What is the Manhattan distance between that location and the ship's starting position?
    */

    let mut waypoint_x = 10;
    let mut waypoint_y = 1;
    let mut ship_x = 0;
    let mut ship_y = 0;

    for step in &steps {
        match step {
            Step::Move(f, d) => {
                // Move the waypoint
                let (dx, dy) = d_move(*f, *d);
                waypoint_x += dx;
                waypoint_y += dy;
            },
            Step::Turn(direction, angle) => {
                match (direction, angle) {
                    (_, 180) => {
                        // Rotate 180 - flip both coordinates
                        waypoint_x *= -1;
                        waypoint_y *= -1;
                    },
                    (Dir::Left,  90) | (Dir::Right, 270) => {
                        // Rotate left - swap the coordinates and flip x
                        std::mem::swap(&mut waypoint_x, &mut waypoint_y);
                        waypoint_x *= -1;
                    },
                    (Dir::Right, 90) | (Dir::Left,  270) => {
                        // Rotate right - swap the coordinates and flip y
                        std::mem::swap(&mut waypoint_x, &mut waypoint_y);
                        waypoint_y *= -1;
                    },
                    a => panic!("unexpected turn {:?}", a),
                }
            },
            Step::Forward(d) => {
                // Move towards the waypoint
                ship_x += waypoint_x * d;
                ship_y += waypoint_y * d;
            },
        }
    }

    println!("Part 2: position is {:?}, sum is {}", (ship_x, ship_y), (ship_x.abs() + ship_y.abs()));

    Ok(())
}
