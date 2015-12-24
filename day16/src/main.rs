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

    let re = Regex::new(r"^Sue (\d+):").unwrap();
    let number = match re.captures(s) {
        None => return Err("Unable to parse memory".into()),
        Some(caps) => caps.at(1).unwrap().parse().unwrap()
    };

    let mut things = HashMap::new();

    // Can't seem to access repeated sub-patterns with a single regex?
    // Need to break this out into two steps

    let re = Regex::new(r"([a-z]+): (\d+)(,|$)").unwrap();
    for caps in re.captures_iter(s) {
        let name = caps.at(1).unwrap().into();
        let count = caps.at(2).unwrap().parse().unwrap();
        things.insert(name, count);
    }

    Ok(Memory { number: number, things: things })
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
