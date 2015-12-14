#![feature(io, iter_arith)]
#![allow(dead_code, unused_features)]

extern crate parser_ast;

use std::env::args;
use std::fs::File;
use std::io::{ BufRead, BufReader };

use parser_ast::ast::*;
use parser_ast::parser::parse_instruction;

type PResult<'a, T> = Result<(T, &'a str), ParseError<&'a str>>;

fn open_file() -> File {
    let filename = args().skip(1).next().expect("usage: day7 {input filename}");
    File::open(filename).expect("Error opening input")
}

fn read_instructions(file: File) -> Vec<Instruction> {
    BufReader::new(file)
        .lines()
        .map(|line| line.expect("Error reading file"))
        .map(|line| {
            match parse_instruction(&line) {
                Ok((val, remainder)) => val,
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