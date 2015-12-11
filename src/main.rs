#![feature(io, iter_arith)]
#![allow(dead_code, unused_features)]

use std::env::args;
use std::fs::File;
use std::io::{ BufRead, BufReader };
use std::str::FromStr;
use std::mem::swap;

fn open_file() -> File {
    let filename = args().skip(1).next().expect("usage: day6 {input filename}");
    File::open(filename).expect("Error opening input")
}

// Coord(x, y)
#[derive(Clone, Copy, Debug)]
struct Coord(usize, usize);

impl Coord {
    fn to_index(&self, stride: usize) -> usize {
        self.1 * stride + self.0
    }
}

impl FromStr for Coord {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Coord, Self::Err> {
        //Consume a string of the form '489,959'
        let mut parts = s.split(',').map(|s| s.parse());
        let x = match parts.next() { Some(Ok(x)) => x, _ => return Err("Error parsing x coord") };
        let y = match parts.next() { Some(Ok(y)) => y, _ => return Err("Error parsing y coord") };
        if parts.next().is_some() {
            return Err("Too many parts");
        }
        Ok(Coord(x, y))
    }
}

#[derive(Debug)]
enum Action {
    Toggle,
    TurnOn,
    TurnOff
}

#[derive(Debug)]
struct Command {
    action: Action,
    from: Coord,
    to: Coord
}

impl FromStr for Command {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Command, Self::Err> {
        // Consume a string of the form 'turn on 489,959 through 759,964'
        // e.g. turn on 489,959 through 759,964
        //      turn off 820,516 through 871,914
        let mut parts = s.split(" ");
        
        //Expect `turn`
        let action =
            match parts.next() {
                Some("turn") => {
                    match parts.next() {
                        Some("on") => Action::TurnOn,
                        Some("off") => Action::TurnOff,
                        _ => return Err("Expected `on` or `off`") 
                    }
                },
                Some("toggle") => {
                    Action::Toggle
                }
                _ => {
                    return Err("Expected `turn` or `toggle`")
                }
            };
            
        //Expect range
        let from = match parts.next().map(|s| s.parse::<Coord>()) { 
            Some(Ok(r))  => r, 
            Some(Err(e)) => return Err(e),
            _            => return Err("Expected range start")
        };
            
        //Expect `through`
        match parts.next() {
            Some("through") => (), 
            _ => return Err("Expected `through`")
        };
            
        //Expect range
        let to = match parts.next().map(|s| s.parse::<Coord>()) { 
            Some(Ok(r))  => r,
            Some(Err(e)) => return Err(e),
            _            => return Err("Expected range to")
        };
        
        if parts.next().is_some() {
            return Err("Expected end of input");
        }
        
        Ok(Command {
            action: action,
            to: to,
            from: from
        })
    }
}

struct LightGrid {
    stride: usize,
    lights: Vec<bool>
}

impl LightGrid {
    fn new(w: usize, h: usize) -> LightGrid {
        LightGrid { stride: w, lights: vec![false; w * h] }
    }
    
    fn get_light_mut(&mut self, coord: Coord) -> &mut bool {
        let index = coord.to_index(self.stride);
        &mut self.lights[index]
    }
    
    fn get_all(&self) -> &[bool] {
        &self.lights[..]
    }
}

fn parse_commands(file: File) -> Vec<Command> {
    BufReader::new(file)
        .lines()
        .map(|line| line.expect("Error reading file"))
        .map(|line| line.parse().expect("Error parsing command"))
        .collect()
}

fn main() {
    
    let commands = parse_commands(open_file());
    
    let mut lights = LightGrid::new(1000, 1000);
    
    for command in &commands {
        
        //Command from, to describes a rectangle
        let Coord(mut x1, mut y1) = command.from;
        let Coord(mut x2, mut y2) = command.to;
        
        if x2 < x1  { swap(&mut x1, &mut x2); }
        if y2 < y1  { swap(&mut y1, &mut y2); }
        
        for y in y1..y2 + 1 {
            for x in x1..x2 + 1 {
                let light = lights.get_light_mut(Coord(x, y));
                match command.action {
                    Action::TurnOn => *light = true,
                    Action::TurnOff => *light = false,
                    Action::Toggle => *light = !*light
                };
            } 
        }
    }
    
    let lights_on_count =
        lights.get_all()
              .iter()
              .filter(|light| **light) // How do we avoid this double de-reference?
              .count();
              
    println!("There are {} lights on", lights_on_count);
    
    /*
    let mut buf = String::new();
    
    for row in 0..10 {
        
        buf.clear();
        
        for &light in lights.get_range(Coord(0, row), Coord(9, row)) {
            if light {
                buf.push('x');
            }
            else {
                buf.push('.');
            }
        }
        
        println!("{}", &buf);
    }*/
}