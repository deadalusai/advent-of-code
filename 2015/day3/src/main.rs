#![feature(io, iter_arith)]
#![allow(dead_code, unused_features)]

use std::env::args;
use std::io::{ Read };
use std::fs::File;
use std::collections::HashMap;
use std::mem::swap;

fn open_file() -> File {
    let filename = args().skip(1).next().expect("usage: day2 {input filename}");
    File::open(filename).expect("Error opening input")
}

#[derive(Clone, Copy)]
enum Instruction {
    Up, Down, Left, Right
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Coord(i32, i32);

impl Coord {
    fn update_with_instruction(&mut self, inst: Instruction) {
        match inst {
            Up    => self.1 += 1,
            Down  => self.1 -= 1,
            Left  => self.0 -= 1,
            Right => self.0 += 1,
        }
    }
}

use Instruction::{ Up, Down, Left, Right };

fn read_instructions(file: File) -> Vec<Instruction> {
    file.chars()
        .map(|c| c.expect("Error reading file"))
        .filter_map(|c| match c {
            '^' => Some(Up),
            'v' => Some(Down),
            '<' => Some(Left),
            '>' => Some(Right),
             _  => None
        })
        .collect()
}

fn main() {
    
    let instructions = read_instructions(open_file());
    
    // Count houses visited by santa alone
    let mut houses = HashMap::new();
    let mut santa_pos = Coord(0, 0);
    
    for inst in &instructions {
        *houses.entry(santa_pos).or_insert(0) += 1;
        santa_pos.update_with_instruction(*inst);
    }
    
    let count_of_houses_visited = houses.keys().count();
    println!("Houses visited at least once by santa: {}", count_of_houses_visited);
    
    // Count houses visited by santa and robo santa
    let mut houses = HashMap::new();
    let mut pos = Coord(0, 0);
    
    //First house is always visited twice
    houses.insert(pos, 2);
    
    // pos and other track the positions of santa and robo santa
    let mut other = pos;
    
    for inst in instructions {
        // Give the instruction to the first worker
        pos.update_with_instruction(inst);
        *houses.entry(pos).or_insert(0) += 1;
        
        // And swap the workers
        swap(&mut pos, &mut other);
    }
    
    let count_of_houses_visited = houses.keys().count();
    println!("Houses visited at least once by santa and robo santa: {}", count_of_houses_visited);
}