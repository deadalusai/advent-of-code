use std::env::args;
use std::fs::File;

use std::io::{ BufRead, BufReader };
use std::collections::HashSet;

fn open_file() -> File {
    let filename = args().skip(1).next().expect("usage: day11 {input filename}");
    File::open(filename).expect("Error opening input")
}

fn read_input(file: File) -> String {
    BufReader::new(file).lines().map(|line| line.expect("Error reading file"))
                        .next().expect("No lines in input")
}

fn check_non_overlapping_pairs(s: &str) -> bool {
    const MIN_PAIRS: usize = 2;
    
    let mut last = None;
    let mut unique_pairs = HashSet::new();
    
    for c in s.chars() {
        last = match last {
            Some(last) if last == c => {
                unique_pairs.insert(c);
                None
            },
            Some(_) | None => {
                Some(c)
            }
        };
        
        if unique_pairs.len() >= MIN_PAIRS {
            return true;
        }
    }
    
    false
}

fn check_increasing_straight(s: &str) -> bool {
    const MIN_STRAIGHT_LEN: usize = 3;
    
    fn next_char(c: char) -> char {
        ((c as u8) + 1) as char
    }
    
    let mut last = None;
    let mut straight_length = 0;
    
    for c in s.chars() {
        last = match last {
            Some(last) if c == next_char(last) => {
                straight_length += 1;
                Some(c)
            },
            Some(_) | None => {
                straight_length = 1;
                Some(c)
            }
        };
        
        if straight_length >= MIN_STRAIGHT_LEN {
            return true;
        }
    }
    
    false
}

fn check_invalid_characters(s: &str) -> bool {
    const INVALID_CHARS: [char; 3] = ['i', 'o', 'l'];
    
    !s.chars().any(|c| INVALID_CHARS.contains(&c))
}

fn is_valid_password(s: &str) -> bool {
 
    check_non_overlapping_pairs(s) && check_increasing_straight(s) && check_invalid_characters(s)
}

fn increment_string(s: &mut String) {
    
    unimplemented!();
    
}

fn main() {
    
    let input = read_input(open_file());
    let mut new = input.clone();
 
    loop {
        if is_valid_password(&new) {
            break;
        }
        
        increment_string(&mut new);
    }
    
    println!("{} -> {}", input, new);
}