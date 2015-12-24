extern crate regex;

use std::env::args;
use std::fs::File;
use std::io::{ BufRead, BufReader };
use std::collections::HashMap;
use regex::Regex;

fn open_file() -> File {
    let filename = args().skip(1).next().expect("usage: day16 {input filename}");
    File::open(filename).expect("Error opening input")
}

#[derive(Debug)]
struct Memory {
    number: u32,
    things: HashMap<String, u32>
}

fn parse_memory(s: &str) -> Result<Memory, String> {

    let pattern = r"([A-Za-z]+) (\d+):(?:\s+([a-z]+): (\d+),?)*";
    let re = Regex::new(pattern).unwrap();

    match re.captures(s) {
        None => Err("Unable to parse memory".into()),
        Some(caps) => {
            unimplemented!()
        }
    }
}

fn read_input(file: File) -> Vec<Memory> {
    BufReader::new(file)
        .lines()
        .map(|line| line.expect("Error reading line"))
        .map(|line| parse_memory(&line).expect("Error parsing memory"))
        .collect()
}

fn main() {

    let memories = read_input(open_file());

    for mem in &memories {
        println!("{:?}", mem);
    }
}
