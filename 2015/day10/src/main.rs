use std::env::args;
use std::fs::File;

use std::fmt::Write;
use std::io::{ BufRead, BufReader };

fn open_file() -> File {
    let filename = args().skip(1).next().expect("usage: day10 {input filename}");
    File::open(filename).expect("Error opening input")
}

fn read_input(file: File) -> String {
    BufReader::new(file).lines().map(|line| line.expect("Error reading file"))
                        .next().expect("No lines in input")
}

fn count_digits(s: &str) -> Vec<(u32, u32)> {
    
    fn char_to_u32(c: char) -> u32 {
        match c {
            '0'...'9' => (c as u8 - '0' as u8) as u32,
             _        => panic!("Unexpected character `{}`", c) 
        }
    }
    
    let mut data = Vec::new();
    let mut current = None;
    
    for c in s.chars() {
        let n = char_to_u32(c);
        current = match current {
            None => {
                Some((n, 1))
            },
            Some((num, count)) if num != n => {
                data.push((num, count));
                Some((n, 1))
            },
            Some((num, count)) => {
                Some((num, count + 1))
            }
        };
    }
    
    if let Some(pair) = current {
        data.push(pair);
    }
    
    data
}

fn build_look_and_say_string(counts: &Vec<(u32, u32)>) -> String {
    
    let mut buf = String::with_capacity(counts.len() * 2);
    
    for &(num, count) in counts {
        write!(&mut buf, "{}{}", count, num).unwrap();
    }
    
    buf
}

fn main() {
    
    let mut input = read_input(open_file());
    
    println!("iteration 0 -> {}", input);
    
    for it in 0..50 {
        
        let counts = count_digits(&input);
        
        input = build_look_and_say_string(&counts);
        
        println!("iteration {} -> {}", it + 1, input.chars().count());
    }
}