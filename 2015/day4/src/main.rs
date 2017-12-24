#![feature(io, iter_arith)]
#![allow(dead_code, unused_features)]

extern crate md5;

use std::env::args;
use std::io::{ BufRead, BufReader };
use std::fs::File;
use std::io::Write;

fn open_file() -> File {
    let filename = args().skip(1).next().expect("usage: day4 {input filename}");
    File::open(filename).expect("Error opening input")
}

fn read_secret_key(file: File) -> String {
    BufReader::new(file)
        .lines()
        .map(|line| line.expect("Error reading input"))
        .nth(0)
        .expect("No lines in input")
}

fn to_hex(input: &[u8]) -> String {
    let mut s = String::new();
    for i in input {
        let octet = format!("{:0>2x}", i);
        s.push_str(&octet);
    }
    s
}

fn main() {
    
    let secret_key = read_secret_key(open_file());
    
    let make_sequence = |secret_key| {
        let mut buf = Vec::new();
        (0..).map(move |number| {
            
            //write! writes utf8 bytes into the buffer
            //Assumption: bytes will be in the ASCII range only
            buf.clear();
            write!(&mut buf, "{}{}", secret_key, number).expect("Error writing input");
            
            let hash = md5::compute(&buf);
            
            (number, to_hex(&hash))
        })
    };
    
    let number =
        make_sequence(&secret_key)
            .filter_map(|(number, hash)| if hash.starts_with("00000") { Some(number) } else { None })
            .nth(0)
            .unwrap();
    
    println!("Smallest number whose md5 starts with five zeroes: {}", number);
    
    let number =
        make_sequence(&secret_key)
            .filter_map(|(number, hash)| if hash.starts_with("000000") { Some(number) } else { None })
            .nth(0)
            .unwrap();
    
    println!("Smallest number whose md5 starts with six zeroes: {}", number);
}