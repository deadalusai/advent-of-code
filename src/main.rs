#![feature(io, iter_arith)]
#![allow(dead_code, unused_features)]

extern crate combine;

use std::env::args;
use std::fs::File;
use std::io::{ BufRead, BufReader };

use combine::{ spaces, many1, digit, lower, string, choice, try, Parser, ParserExt, ParseError };

type PResult<'a, T> = Result<(T, &'a str), ParseError<&'a str>>;

fn open_file() -> File {
    let filename = args().skip(1).next().expect("usage: day7 {input filename}");
    File::open(filename).expect("Error opening input")
}

type Signal = u16;
type Label = String;

#[derive(Debug)]
enum Source {
    Wire(Label),
    Const(Signal)
}

#[derive(Debug)]
enum Gate1 {
    NOT
}

#[derive(Debug)]
enum Gate2 {
    AND, OR, LSHIFT, RSHIFT
}

#[derive(Debug)]
enum Expr {
    Input(Source),
    Gate1(Gate1, Source),
    Gate2(Gate2, Source, Source)
}

#[derive(Debug)]
struct Instruction {
    expr: Expr,
    target: Label
}

fn parse_instruction(s: &str) -> PResult<Instruction> {
    
    // A wire "label" is a sequence of lowercase letters
    let p_wire_label = || many1(lower()).message("Label");

    // A raw signal is an integer                         
    let p_signal = || many1(digit()).map(|val: String| val.parse::<Signal>().unwrap()).message("Raw Signal");
    
    let p_source = || {
        let signal = p_signal().map(|l| Source::Const(l));
        let wire = p_wire_label().map(|s| Source::Wire(s));
        spaces().with(
            signal.or(wire)
        )
    };
        
    let p_gate_2 = || {
        let choice =
            choice([
                string("AND"),
                string("OR"),
                string("LSHIFT"),
                string("RSHIFT"),
            ])
            .map(|label| match label {
                "AND"    => Gate2::AND,
                "OR"     => Gate2::OR,
                "LSHIFT" => Gate2::LSHIFT,
                "RSHIFT" => Gate2::RSHIFT,
                _        => panic!("p_gate_2")
            });
            
        spaces().with(
            choice
        )
    };
        
    let p_gate_1 = || {
        let choice = string("NOT").map(|_| Gate1::NOT);
        
        spaces().with(
            choice
        )
    };
        

    let p_expr = || {
        let gate2 = p_source().and(p_gate_2()).and(p_source()).map(|((source1, gate), source2)| Expr::Gate2(gate, source1, source2)).message("Two-input Gate");
        let gate1 = p_gate_1().and(p_source())                .map(|(gate, source)|             Expr::Gate1(gate, source))          .message("One-input Gate");
        let input = p_source()                                .map(|source|                     Expr::Input(source));
        
        spaces().with(
            try(gate2).or(try(gate1)).or(input)
        )
    };
    
    let p_inst = || {
        let arrow = spaces().with(string("->"));
        let target = spaces().with(p_wire_label());
        
        p_expr().skip(arrow).and(target)
            .map(|(expr, target)| Instruction { expr: expr, target: target })
    };
    
    p_inst().parse(s)
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