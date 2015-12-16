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
    
fn char_to_alpha_index(c: char) -> u8 {
    c as u8 - 'a' as u8
}

fn alpha_index_to_char(i: u8) -> char {
    ('a' as u8 + i) as char
}

fn check_non_overlapping_pairs(s: &[u8]) -> bool {
    const MIN_PAIRS: usize = 2;
    
    let mut last = None;
    let mut unique_pairs = HashSet::new();
    
    for c in s.iter() {
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

fn check_increasing_straight(s: &[u8]) -> bool {
    const MIN_STRAIGHT_LEN: usize = 3;
    
    let mut last = None;
    let mut straight_length = 0;
    
    for &c in s {
        if let Some(last) = last {
            if c == (last + 1) {
                straight_length += 1;
            }
            else {
                straight_length = 1;
            }
        }
        
        if straight_length >= MIN_STRAIGHT_LEN {
            return true;
        }
        
        last = Some(c);
    }
    
    false
}

fn check_invalid_characters(s: &[u8]) -> bool {
    
    let invalid_chars = [char_to_alpha_index('i'), char_to_alpha_index('o'), char_to_alpha_index('l')];
    
    !s.iter().any(|c| invalid_chars.contains(&c))
}

fn is_valid_password(s: &[u8]) -> bool {
 
    check_non_overlapping_pairs(s) && check_increasing_straight(s) && check_invalid_characters(s)
}

fn increment_password(s: &mut [u8]) {
    
    if s.len() == 0 {
        panic!("Input is zero length")
    }
    
    // Step the last digit in the password (wrapping after 24).
    // If the digit wraps, continue and wrap the next digit too.
    // Repeat until you reach a digit that does not wrap.
    
    let digit_max = char_to_alpha_index('z');
    
    let last = s.len() - 1;
    let mut i = last;
    
    loop {
        
        let next_digit = s[i] + 1;
        
        if next_digit <= digit_max {
            s[i] = next_digit;
            
            // Incremented this digit without wrapping - halt
            break;
        }
        
        s[i] = 0;
        
        // Move to the next digit, or wrap back around
        i = if i == 0 { last } else { i - 1 }
    }
}

fn next_password(s: &str) -> String {
    
    //Convert string into a vector of alphabetic indices
    let mut working: Vec<u8> = s.chars().map(char_to_alpha_index).collect();
 
    loop {
        increment_password(&mut working);
        
        if is_valid_password(&working) {
            break;
        }
    }
    
    working.into_iter().map(alpha_index_to_char).collect()
}

fn main() {
    
    let input = read_input(open_file());
    let result = next_password(&input);
    println!("{} -> {}", input, result);
    
    let input = result;
    let result = next_password(&input);
    println!("{} -> {}", input, result);
}