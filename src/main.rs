#![feature(io, iter_arith)]
#![allow(dead_code, unused_features)]

use std::env::args;
use std::io::{ BufRead, BufReader };
use std::fs::File;

fn open_file() -> File {
    let filename = args().skip(1).next().expect("usage: day4 {input filename}");
    File::open(filename).expect("Error opening input")
}

// A nice string is one with all of the following properties:
// * It contains at least three vowels (aeiou only), like aei, xazegov, or aeiouaeiouaeiou.
// * It contains at least one letter that appears twice in a row, like xx, abcdde (dd), or aabbccdd (aa, bb, cc, or dd).
// * It does not contain the strings ab, cd, pq, or xy, even if they are part of one of the other requirements.
fn is_str_nice(s: &str) -> bool {
    const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    
    let mut last = None;
    let mut vowel_count = 0;
    let mut has_double = false;
    
    for c in s.chars() {
        
        if VOWELS.contains(&c) {
            vowel_count += 1;
        }
        
        if let Some(last) = last {
            match (last, c) {
                ('a', 'b') |
                ('c', 'd') |
                ('p', 'q') |
                ('x', 'y') => {
                    //Found naughty pair - bail immediately
                    return false;
                },
                (o, c) => {
                    if o == c {
                        //Found a nice double 
                        has_double = true;
                    }
                }
            };
        }
        
        last = Some(c);
    }
        
    has_double && vowel_count >= 3
}

fn main() {
    
    let nice_strings =
        BufReader::new(open_file())
            .lines()
            .map(|line| line.expect("Error reading input"))
            .filter(|s| is_str_nice(&s))
            .count();
    
    println!("Nice strings: {}", nice_strings);
}