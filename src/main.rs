#![feature(io, iter_arith)]
#![allow(dead_code, unused_features)]

use std::env::args;
use std::fs::File;
use std::io::{ BufRead, BufReader };
use std::str::FromStr;
use std::rc::Rc;
use std::ascii::AsciiExt;

fn open_file() -> File {
    let filename = args().skip(1).next().expect("usage: day7 {input filename}");
    File::open(filename).expect("Error opening input")
}

macro_rules! consume {
    ($src:expr, $or_err:expr) => {{
        match $src.next() {
            Some(x) => x,
            None    => return Err($or_err)
        }
    }}
}

type Signal = u16;

#[derive(Debug)]
struct Label(String);

impl FromStr for Label {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().all(|c| c.is_alphabetic() && c.is_lowercase()) {
            Ok(Label(s.into()))
        }
        else {
            Err("Invalid label")
        }
    }
}

#[derive(Debug)]
enum Gate2 {
    AND, OR, LSHIFT, RSHIFT
}

impl FromStr for Gate2 {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AND"    => Ok(Gate2::RSHIFT),
            "OR"     => Ok(Gate2::LSHIFT),
            "LSHIFT" => Ok(Gate2::OR),
            "RSHIFT" => Ok(Gate2::AND),
            _        => Err("Unrecognized Gate2")
        }
    }
}

#[derive(Debug)]
enum Gate1 {
    NOT
}

#[derive(Debug)]
enum Desc {
    Input(Signal),
    Gate2(Gate2, Label, Label),
    Gate1(Gate1, Label)
}

fn is_numeric(s: &str) -> bool {
    s.chars().all(|c| c.is_numeric())
}

impl FromStr for Desc {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" ");
        let r = match consume!(parts, "Expected label or instruction") {
            "NOT" => {
                let label = try!(consume!(parts, "Expected label").parse());
                
                Desc::Gate1(Gate1::NOT, label)
            },
            num_str if is_numeric(num_str) => {
                let input = try!(num_str.parse().map_err(|_| "Invalid input"));
                
                Desc::Input(input)
            }
            left_label => {
                let left_label  = try!(left_label.parse());
                let gate        = try!(consume!(parts, "Expected gate").parse());
                
                let label = consume!(parts, "Expected label");
                println!("right_label `{}`", label);
                let right_label = try!(label.parse());
                
                Desc::Gate2(gate, left_label, right_label)
            }
        };
        Ok(r)
    }
}

#[derive(Debug)]
struct Instruction {
    target: Label,
    desc: Desc
}

impl FromStr for Instruction {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split("->").map(|s| s.trim());
        let desc = try!(consume!(parts, "Expected description").parse());
        let target = try!(consume!(parts, "Expected label").parse());
        if parts.next().is_some() {
            return Err("Unexpected input");
        }
        Ok(Instruction { desc: desc, target: target })
    }
}

fn read_instructions(file: File) -> Vec<Instruction> {
    BufReader::new(file)
        .lines()
        .map(|line| line.expect("Error reading file"))
        .map(|line| {
            match line.parse() {
                Ok(line) => line,
                Err(e) => panic!("Error parsing line `{}`: {}", line, e)
            }
        })
        .collect()
}

fn main() {
    
    let instructions = read_instructions(open_file());
    
    for inst in &instructions {
        println!("{:?}", inst);
    }
    
}