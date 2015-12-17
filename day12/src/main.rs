extern crate serde_json;

use std::env::args;
use std::fs::File;

use std::io::{ Read };

use serde_json::{ Value };

fn open_file() -> File {
    let filename = args().skip(1).next().expect("usage: day11 {input filename}");
    File::open(filename).expect("Error opening input")
}

fn read_input(mut file: File) -> Value {
    let mut json = String::new();
    file.read_to_string(&mut json);

    serde_json::from_str(&json).expect("Error parsing input")
}

fn main() {

    let json = read_input(open_file());

    println!("{:?}", json);
}