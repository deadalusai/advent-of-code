#![feature(io, iter_arith)]
#![allow(dead_code, unused_features)]

use std::env::args;
use std::io::{ BufReader, BufRead };
use std::fs::File;
use std::str::FromStr;
use std::cmp::min;

fn open_file() -> File {
    let filename = args().skip(1).next().expect("usage: day2 {input filename}");
    File::open(filename).expect("Error opening input")
}

struct Box { l: u32, w: u32, h: u32 }

impl Box {
    fn top(&self)             -> u32 { self.l *     self.w     }
    fn top_perimeter(&self)   -> u32 { self.l * 2 + self.w * 2 }
    
    fn side(&self)            -> u32 { self.l *     self.h     }
    fn side_perimeter(&self)  -> u32 { self.l * 2 + self.h * 2 }
    
    fn front(&self)           -> u32 { self.w *     self.h     }
    fn front_perimeter(&self) -> u32 { self.w * 2 + self.h * 2 }
    
    fn perimeter_of_smallest_side(&self) -> u32 {
        min(min(self.top_perimeter(), self.front_perimeter()), self.side_perimeter())
    }
    
    fn surface_area_of_smallest_side(&self) -> u32 {
        min(min(self.top(), self.front()), self.side())
    }
    
    fn surface_area(&self) -> u32 {
        2 * self.top() + 
        2 * self.side() + 
        2 * self.front()
    }
    
    fn volume(&self) -> u32 {
        self.l * self.w * self.h
    }
}

impl FromStr for Box {
    type Err = ();
    fn from_str(s: &str) -> Result<Box, Self::Err> {
        let mut iter = s.split('x').map(|i| i.parse());
        
        let l = match iter.next() { Some(Ok(c)) => c, _ => return Err(()) };
        let w = match iter.next() { Some(Ok(c)) => c, _ => return Err(()) };
        let h = match iter.next() { Some(Ok(c)) => c, _ => return Err(()) };
        
        if iter.next().is_some() {
            return Err(()); //Too many parts
        }
        
        Ok(Box { l: l, w: w, h: h })
    }
}

fn read_boxes(file: File) -> Vec<Box> {
    BufReader::new(file)
        .lines()
        .map(|line| line.expect("Error reading file"))
        .filter_map(|line| line.parse().ok())
        .collect()
}

fn main() {
    
    let boxes = read_boxes(open_file());
    
    //Find the surface area of the box, which is 2*l*w + 2*w*h + 2*h*l.
    //The elves also need a little extra paper for each present: the area of the smallest side.
    let total_square_footage_needed: u32 =
        boxes.iter()
             .map(|b| b.surface_area() + b.surface_area_of_smallest_side())
             .sum();
    
    //Each present also requires a bow made out of ribbon as well;
    //The ribbon required to wrap a present is the shortest distance around its sides, or the smallest perimeter of any one face. 
    //The feet of ribbon required for the perfect bow is equal to the cubic feet of volume of the present.
    let total_feet_of_ribbon_needed: u32 =
        boxes.iter()
             .map(|b| b.perimeter_of_smallest_side() + b.volume())
             .sum();
             
    println!("Square footage needed: {}", total_square_footage_needed);
    println!("Feet of ribbon required: {}", total_feet_of_ribbon_needed);
}