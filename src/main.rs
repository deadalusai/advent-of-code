#![feature(io, iter_arith)]
#![allow(dead_code, unused_features)]

extern crate parser_ast;

use std::env::args;
use std::fs::File;
use std::io::{ BufRead, BufReader };
use std::collections::HashMap;

use parser_ast::ast::*;
use parser_ast::parser::parse_instruction;

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
                Ok(val) => val,
                Err(e)  => panic!("Error parsing line `{}`: {}", line, e)
            }
        })
        .collect()
}

fn main() {
    
    let instructions = read_instructions(open_file());
    
    // Map of wire labels to signals
    let mut map = HashMap::new();
    
    for inst in instructions {
        
        let signal: Signal = {
            match inst.expr {
                Expr::Input(src) => resolve(&map, src),
                Expr::Gate2(gate, a, b) => {
                    let a = resolve(&map, a);
                    let b = resolve(&map, b);
                    match gate {
                        Gate2::AND    => a & b, 
                        Gate2::OR     => a | b,
                        Gate2::RSHIFT => a >> b,
                        Gate2::LSHIFT => a << b
                    }
                },
                Expr::Gate1(gate, a) => {
                    let a = resolve(&map, a);
                    match gate {
                        Gate1::NOT => !a
                    }
                }
            }
        };
        
        map.insert(inst.target, signal);
    }
    
    for (key, value) in map.iter() {
        println!("{}: {}", key, value);
    }
}

fn resolve(map: &HashMap<Label, Signal>, source: Source) -> Signal { 
    match source {
        Source::Const(s) => s,
        Source::Wire(label) => *map.get(&label).unwrap_or_else(|| panic!("No signal value for {}", label))
    }
}
