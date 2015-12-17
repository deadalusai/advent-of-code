extern crate parser_ast;

use std::env::args;
use std::fs::File;
use std::io::{ BufRead, BufReader };

use std::collections::HashMap;
use std::collections::hash_map::Entry::*;

use parser_ast::ast::*;
use parser_ast::parse_instruction;

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

#[derive(Debug, Clone)]
struct Wire {
    source: Expr,
    value: Option<Signal>
}

fn main() {
    
    let instructions = read_instructions(open_file());
    
    // Map of wire to their labels
    let mut map = HashMap::new();
    
    for inst in instructions {
        match map.entry(inst.target.clone()) {
            Occupied(_) => panic!("Attempted to redefine wire {}", &inst.target),
            Vacant(e) => {
                e.insert(Wire {
                    source: inst.expr,
                    value: None
                });
            }
        };
    }
    
    let signal = resolve_recusive(&mut map, "a");
    println!("Signal on wire a: {:?}", signal);
    
    // Part two:
    // Now, take the signal you got on wire a, override wire b to that signal,
    // and reset the other wires (including wire a).
    // What new signal is ultimately provided to wire a?

    for (key, wire) in map.iter_mut() {
        wire.value = match &key[..] {
           "b" => Some(signal),
            _  => None 
        }     
    }
    
    let signal = resolve_recusive(&mut map, "a");
    println!("Signal on wire a (take two): {:?}", signal);
}

fn resolve_recusive(map: &mut HashMap<Label, Wire>, label: &str) -> Signal {
    
    let expr = {
        
        let wire = map.get(label).expect("Cannot find wire");
        
        //Already worked out the value of this wire?
        if let Some(ref val) = wire.value {
            return *val;
        }
        
        wire.source.clone()
    };
    
    // Resolve the value of the expression attached to this wire.
    let signal = match expr {
        Expr::Input(src) => {
            resolve_source(map, &src)
        },
        Expr::Gate2(gate, a, b) => {
            let a = resolve_source(map, &a);
            let b = resolve_source(map, &b);
            match gate {
                Gate2::AND    => a & b, 
                Gate2::OR     => a | b,
                Gate2::RSHIFT => a >> b,
                Gate2::LSHIFT => a << b
            }
        },
        Expr::Gate1(gate, a) => {
            let a = resolve_source(map, &a);
            match gate {
                Gate1::NOT => !a
            }
        }
    };
    
    // Cache it
    map.get_mut(label).unwrap().value = Some(signal);
    
    // Done
    signal
}

fn resolve_source(map: &mut HashMap<Label, Wire>, source: &Source) -> Signal { 
    match source {
        &Source::Const(ref s) => *s,
        &Source::Wire(ref label) => resolve_recusive(map, label)
    }
}