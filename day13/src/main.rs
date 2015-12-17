use std::env::args;
use std::fs::File;

use std::io::{ BufRead, BufReader };

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
    
    for inst in instructions {
        
        println!("{:?}", inst);
    }    

}