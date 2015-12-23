use std::env::args;
use std::fs::File;

use std::io::{ BufRead, BufReader };

use std::collections::{ HashSet, HashMap };

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

macro_rules! part {
    ( $e:expr, $message:expr ) => {
        match $e.next() {
            Some(s) => s,
            None => return Err(format!("Expected {}", $message))
        }
    }
}

macro_rules! expect {
    ( $e:expr, $expected:expr ) => {
        match $e.next() {
            Some(s) if s != $expected => return Err(format!("Expected '{}'", $expected)),
            None                      => return Err(format!("Expected '{}'", $expected)),
            _                         => ()
        }
    }
}

fn parse_instruction(s: &str) -> Result<Instruction, String> {
    
    let mut parts = s.split(' ');
    
    let person = part!(parts, "person name");
    
    expect!(parts, "would");
    
    let change = part!(parts, "change type");
    let change_amount = part!(parts, "change amount");
    
    let change_amount = match change_amount.parse() {
        Ok(n) => n,
        Err(e) => return Err(format!("Error parsing change amount: {}", e))
    };
    
    let change = match change {
        "gain" => Change::Gain(change_amount),
        "lose" => Change::Lose(change_amount),
        other  => return Err(format!("Invalid change amount: {}", other))
    };
    
    expect!(parts, "happiness");
    expect!(parts, "units");
    expect!(parts, "by");
    expect!(parts, "sitting");
    expect!(parts, "next");
    expect!(parts, "to");
    
    let neighbour = part!(parts, "person name");
    
    match neighbour.chars().rev().next() {
        Some('.') => (),
        _         => return Err("Expected .".into())       
    }
    
    let neighbour = &neighbour[..neighbour.len() - 1];
    
    if parts.next().is_some() {
        return Err("Expected end of input".into());
    }
    
    Ok(Instruction { person: person.into(), change: change, neighbour: neighbour.into() })
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