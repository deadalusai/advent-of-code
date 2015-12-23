extern crate regex;

use std::env::args;
use std::fs::File;

use std::io::{ BufRead, BufReader };

use std::collections::{ HashSet, HashMap };

use regex::Regex;

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

// Iterates through pairs in an array (first and last element are a pair)

fn pairs<'a, T>(source: &'a [T]) -> Pairs<'a, T> {
    Pairs { source: source, i: 0 }
}

struct Pairs<'a, T: 'a> {
    source: &'a [T],
    i: usize
}

impl <'a, T> Iterator for Pairs<'a, T> {
    type Item = (&'a T, &'a T);

    fn next(&mut self) -> Option<Self::Item> {

        let len = self.source.len();
        let next = match self.i {
            i if i == len - 1 => Some((&self.source[i], &self.source[0])),
            i if i >= len     => None,
            i                 => Some((&self.source[i], &self.source[i + 1]))
        };

        self.i += 1;
        next
    }
}

// Experimental iterative permutator
// (Takes ownership and requires copy)

struct Permute<T> {
    source: Vec<T>,
    index: usize,
    sub: Option<Box<Permute<T>>>,
}

impl <T: Clone> Iterator for Permute<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {

        // Simple case - source of one elements
        if self.source.len() <= 1 {
            let result = match self.index {
                0 => Some(self.source.clone()),
                _ => None
            };
            self.index += 1;
            return result;
        }

        // Simple case - source of two elements
        if self.source.len() == 2 {
            let result = match self.index {
                0 => Some(self.source.clone()),
                1 => Some(vec![self.source[1].clone(), self.source[0].clone()]),
                _ => None
            };
            self.index += 1;
            return result;
        }

        loop {
            // Done iterating?
            if self.index >= self.source.len() {
                return None;
            }

            if self.sub.is_none() {
                let source = self.source.iter()
                                 .enumerate()
                                 .filter(|&(i, _)| i != self.index)
                                 .map(|(_, v)| v.clone())
                                 .collect();

                self.sub = Some(Box::new(permute(source)));
            }

            let next = match self.sub.as_mut().unwrap().next() {
                None => None,
                Some(mut v) => {
                    v.insert(0, self.source[self.index].clone());
                    Some(v)
                }
            };

            if next.is_none() {
                //Reached the end of the sub iterator!
                self.sub = None;
                self.index += 1;
            }
            else {
                return next;
            }
        }
    }
}

fn permute<T>(arr: Vec<T>) -> Permute<T> {
    Permute { source: arr, index: 0, sub: None }
}