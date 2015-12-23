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

    let mut people = HashSet::new();
    let mut relationships = HashMap::new();

    for inst in instructions {
        people.insert(inst.person.clone());
        relationships.insert((inst.person, inst.neighbour), inst.change);
    }

    people.insert("My own good self".into());

    let people: Vec<String> = people.into_iter().collect();

    let mut happiness_delta_max = 0;

    for p in permute(people) {

        let mut happiness_delta = 0;

        for (left, right) in pairs(&p) {

            let keys = [
                (left.clone(), right.clone()),
                (right.clone(), left.clone())
            ];

            for key in keys.iter() {
                match relationships.get(key) {
                    Some(&Change::Gain(ref amt)) => { happiness_delta += *amt; },
                    Some(&Change::Lose(ref amt)) => { happiness_delta -= *amt; },
                    _ => {}
                }
            }
        }

        if happiness_delta > happiness_delta_max {
            happiness_delta_max = happiness_delta;
        }
    }

    println!("Biggest happiness change: {}", happiness_delta_max);
}
