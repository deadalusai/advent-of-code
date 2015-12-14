#![feature(io, iter_arith)]
#![allow(dead_code, unused_features)]

extern crate parser_ast;

use std::env::args;
use std::fs::File;
use std::io::{ BufRead, BufReader };

use parser_ast::ast::*;
use parser_ast::parser::parse_instruction;

fn open_file() -> File {
    let filename = args().skip(1).next().expect("usage: day7 {input filename}");
    File::open(filename).expect("Error opening input")
}

fn read_instructions(file: File) -> Vec<Result<Instruction, String>> {
    BufReader::new(file)
        .lines()
        .map(|line| line.expect("Error reading file"))
        .map(|line| {
            match parse_instruction(&line) {
                Ok(val) => Ok(val),
                Err(e)  => Err(format!("Error parsing line `{}`: {}", line, e))
            }
        })
        .collect()
}

fn main() {
    
    let instructions = read_instructions(open_file());
    
    for inst in instructions {
        match inst {
            Ok(inst) => println!("{:?}", inst),
            Err(msg) => println!("Error: {}", msg) 
        }
    }
}