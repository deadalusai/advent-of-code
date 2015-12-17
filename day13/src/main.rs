use std::env::args;
use std::fs::File;

use std::io::{ Read };

fn open_file() -> File {
    let filename = args().skip(1).next().expect("usage: day13 {input filename}");
    File::open(filename).expect("Error opening input")
}

fn read_input(mut file: File) -> String {
    let mut json = String::new();
    file.read_to_string(&mut json);
    json
}

fn main() {

}