#![feature(io, iter_arith)]
#![allow(dead_code, unused_features)]

use std::env::args;
use std::fs::File;
use std::io::{ BufRead, BufReader };
use std::str::FromStr;
use std::mem::swap;

fn open_file() -> File {
    let filename = args().skip(1).next().expect("usage: day7 {input filename}");
    File::open(filename).expect("Error opening input")
}

fn main() {
    
}