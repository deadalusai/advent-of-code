#![feature(io, iter_arith)]
#![allow(dead_code, unused_features)]

use std::env::args;
use std::io::{ BufRead, BufReader };
use std::fs::File;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{ Occupied, Vacant };

fn open_file() -> File {
    let filename = args().skip(1).next().expect("usage: day5 {input filename}");
    File::open(filename).expect("Error opening input")
}

//
// A nice string is one with all of the following properties:
// * It contains at least three vowels (aeiou only), like aei, xazegov, or aeiouaeiouaeiou.
// * It contains at least one letter that appears twice in a row, like xx, abcdde (dd), or aabbccdd (aa, bb, cc, or dd).
// * It does not contain the strings ab, cd, pq, or xy, even if they are part of one of the other requirements.
//
fn is_str_nice_old_rules(s: &str) -> bool {
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

//
// Now, a nice string is one with all of the following properties:
// * It contains a pair of any two letters that appears at least twice in the string without overlapping,
//   like xyxy (xy), aabcdefgaa (aa) or aaaa (aa), but not like aaa (aa, but it overlaps).
// * It contains at least one letter which repeats with exactly one letter between them, like xyx, 
//   abcdefeghi (efe), or even aaa.
//
fn is_str_nice_new_rules(s: &str) -> bool {
    
    //Track the first position of every pair found within the string
    let mut double_indices = HashMap::new();
    
    //Track the last two characters (for counting anagrams)
    let mut last1 = None; // index - 1
    let mut last2 = None; // index - 2
    
    let mut anagram_count = 0;
    let mut non_overlapping_double_count = 0;
    
    for (index, c) in s.chars().enumerate() {
        
        if let Some(a) = last1 {
            let pair = (a, c);
            
            //Check for the first place this pair occured
            match double_indices.entry(pair) {
                Occupied(e) => {
                    //Was it more than one position away from this character?
                    let last_index = *e.get();
                    if index - last_index > 1 {
                        //Count it as a non-overlapping double
                        non_overlapping_double_count += 1;
                    }
                },
                Vacant(e) => {
                    //First time we've seen this pair - record the position within the string
                    e.insert(index);
                }
            }
        }
        
        if let Some(a) = last2 {
            if a == c {
                //Found a double seperated by one character
                anagram_count += 1;
            }
        }
        
        last2 = last1;
        last1 = Some(c);
    }
    
    anagram_count > 0 && non_overlapping_double_count > 0
}

fn count_nice_strings(file: File, rule: fn(&str) -> bool) -> usize {
    BufReader::new(file)
        .lines()
        .map(|line| line.expect("Error reading input"))
        .filter(|s| (rule)(&s))
        .count()
}

fn main() {
    
    let nice_strings = count_nice_strings(open_file(), is_str_nice_old_rules);
    
    println!("Nice strings (old rules): {}", nice_strings);
    
    let nice_strings = count_nice_strings(open_file(), is_str_nice_new_rules);
    
    println!("Nice strings (new rules): {}", nice_strings);
}