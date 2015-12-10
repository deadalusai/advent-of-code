#![feature(io)]

use std::env::args;
use std::io::{ Read };
use std::fs::File;

fn open_file() -> File {
    let filename = args().skip(1).next().expect("usage: day1 {input filename}");
    File::open(filename).expect("Error opening input")
}

fn main() {
    let level = find_final_level(&mut open_file());
    println!("Level {}", level);
    
    let first = find_first_step_which_descends_into_the_basement(&mut open_file());
    println!("First step below floor level: {:?}", first);
}

fn find_final_level(input: &mut File) -> i32 {
    
    let mut level = 0;
    
    for c in input.chars() {
        match c {
            Ok('(')    => level += 1, //Step up
            Ok(')')    => level -= 1, //Step down
            Ok( _ )    => continue,   //Ignore character
            Err(ref e) => panic!("Unexpected error: {:?}", e)
        }
    }
    
    level
}

fn find_first_step_which_descends_into_the_basement(input: &mut File) -> Option<u32> {
    
    let mut level = 0;
    let mut step = 0;
    
    for c in input.chars() {
        match c {
            Ok('(')    => level += 1, //Step up
            Ok(')')    => level -= 1, //Step down
            Ok( _ )    => continue,   //Ignore character
            Err(ref e) => panic!("Unexpected error: {:?}", e)
        }
        
        step += 1;
        if level < 0 {
            return Some(step);
        }
    }
    
    None
}
