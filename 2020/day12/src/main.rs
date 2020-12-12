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
    enum Step {
        Move(Facing, i32),
        Left(i32),
        Right(i32),
        Forward(i32),
    }

    fn parse_step(line: &str) -> Result<Step, AppErr> {
        let c = line.chars().next().ok_or(AppErr::new("parse fail", "expected direction instruction"))?;
        let num = line[1..].parse::<i32>()?;
        match c {
            'N' => Ok(Step::Move(Facing::North, num)),
            'S' => Ok(Step::Move(Facing::South, num)),
            'E' => Ok(Step::Move(Facing::East, num)),
            'W' => Ok(Step::Move(Facing::West, num)),
            'L' => Ok(Step::Left(num)),
            'R' => Ok(Step::Right(num)),
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

    fn d_move(facing: Facing, distance: i32) -> (i32, i32) {
        match facing {
            Facing::North => (0, distance),
            Facing::South => (0, -distance),
            Facing::East => (distance, 0),
            Facing::West => (-distance, 0),
        }
    }

    fn d_facing(facing: Facing, angle: i32) -> Facing {
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
            Step::Left(angle) => {
                facing = d_facing(facing, -angle);
            },
            Step::Right(angle) => {
                facing = d_facing(facing, *angle);
            },
        }
    }

    println!("Part 1: position is {:?}, sum is {}", (x, y), (x.abs() + y.abs()));

    Ok(())
}
