#![feature(convert)]

extern crate regex;

mod permute;
mod pairs;

use std::env::args;
use std::fs::File;
use std::io::{ BufRead, BufReader };
use std::collections::{ HashSet, HashMap };
use regex::Regex;
use permute::permute;
use pairs::pairs;

fn open_file() -> File {
    let filename = args().skip(1).next().expect("usage: day13 {input filename}");
    File::open(filename).expect("Error opening input")
}

#[derive(Debug)]
enum Change {
    Gain(i32),
    Lose(i32)
}

#[derive(Debug)]
struct Instruction {
    person: String,
    neighbour: String,
    change: Change,
}

fn parse_instruction(s: &str) -> Result<Instruction, String> {

    let pattern = r"^([A-Za-z]+) would (gain|lose) (\d+) happiness units by sitting next to ([A-Za-z]+).$";
    let re = Regex::new(pattern).unwrap();

    match re.captures(s) {
        None       => Err("Instruction does not match pattern".into()),
        Some(caps) => {

            let person    = caps.at(1).unwrap().into();
            let change    = caps.at(2).unwrap();
            let units     = caps.at(3).unwrap().parse().unwrap();
            let neighbour = caps.at(4).unwrap().into();

            let change = match change {
                "gain" => Change::Gain(units),
                "lose" => Change::Lose(units),
                 _     => unreachable!()
            };

            Ok(Instruction {
                person: person,
                neighbour: neighbour,
                change: change
            })
        }
    }
}

fn read_input(file: File) -> Vec<Instruction> {
    BufReader::new(file)
        .lines()
        .map(|line| line.expect("Error reading file"))
        .map(|line| parse_instruction(&line).expect("Error parsing instruction"))
        .collect()
}

fn main() {

    let instructions = read_input(open_file());

    // Collate distinct people in the instruction set
    let mut people: HashSet<_> =
        instructions.iter().map(|s| s.person.as_str()).collect();

    // Build lookup table of relationships
    let mut relationships = HashMap::new();
    for inst in instructions.iter() {
        relationships.entry(inst.person.as_str())
                     .or_insert_with(|| HashMap::new())
                     .insert(inst.neighbour.as_str(), &inst.change);
    }

    //Part two - add yourself to the mix (with no relationship baggage)
    people.insert("My own good self");

    let mut happiness_delta_max = 0;

    for peeps in permute(people.iter()) {

        let mut happiness_delta = 0;

        for (left, right) in pairs(peeps.into_iter()) {

            let mut check = |a, b| {
                match relationships.get(a).and_then(|rel| rel.get(b)) {
                    Some(&&Change::Gain(amt)) => { happiness_delta += amt; },
                    Some(&&Change::Lose(amt)) => { happiness_delta -= amt; },
                    _ => {}
                }
            };

            check(left, right);
            check(right, left);
        }

        if happiness_delta > happiness_delta_max {
            happiness_delta_max = happiness_delta;
        }
    }

    println!("Biggest happiness change: {}", happiness_delta_max);
}
