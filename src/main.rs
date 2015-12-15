use std::env::args;
use std::fs::File;

use std::io::{ BufRead, BufReader };

fn open_file() -> File {
    let filename = args().skip(1).next().expect("usage: day11 {input filename}");
    File::open(filename).expect("Error opening input")
}

fn read_input(file: File) -> String {
    BufReader::new(file).lines().map(|line| line.expect("Error reading file"))
                        .next().expect("No lines in input")
}

fn main() {
    
    let mut input = read_input(open_file());
    
}