use std::env::args;
use std::fs::File;

fn open_file() -> File {
    let filename = args().skip(1).next().expect("usage: day9 {input filename}");
    File::open(filename).expect("Error opening input")
}

fn main() {
    
}