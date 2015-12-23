extern crate regex;

use std::env::args;
use std::fs::File;
use std::io::{ BufRead, BufReader };
use regex::Regex;

fn open_file() -> File {
    let filename = args().skip(1).next().expect("usage: day14 {input filename}");
    File::open(filename).expect("Error opening input")
}

#[derive(Debug)]
struct Instruction {
    name: String,
    velocity_kms: u32,
    flight_time_s: u32,
    rest_time_s: u32
}

fn parse_instruction(s: &str) -> Result<Instruction, String> {

    let pattern = r"^([A-Za-z]+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.$";
    let re = Regex::new(pattern).unwrap();

    match re.captures(s) {
        None => Err("Invalid instruciton".into()),
        Some(caps) => {

            let name   = caps.at(1).unwrap().into();
            let vel    = caps.at(2).unwrap().parse().unwrap();
            let f_time = caps.at(3).unwrap().parse().unwrap();
            let r_time = caps.at(4).unwrap().parse().unwrap();

            Ok(Instruction {
                name: name,
                velocity_kms: vel,
                flight_time_s: f_time,
                rest_time_s: r_time
            })
        }
    }
}

fn read_instructions(file: File) -> Vec<Instruction> {
    BufReader::new(file)
        .lines()
        .map(|line| line.expect("Error reading line"))
        .map(|line| parse_instruction(&line).expect("Error parsing instruciton"))
        .collect()
}

fn main() {

    let instructions = read_instructions(open_file());

    println!("{:?}", instructions);

}
