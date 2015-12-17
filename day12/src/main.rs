#![feature(iter_arith)]

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
    file.read_to_string(&mut json).expect("Error reading file");
    serde_json::from_str(&json).expect("Error parsing input")
}

fn recursive_sum_numbers(json: &Value) -> i64 {
    match json {
        &Value::Object(ref map) => {
            map.iter()
               .map(|(_, val)| recursive_sum_numbers(val))
               .sum()
        },
        &Value::Array(ref arr) => {
            arr.iter()
               .map(|val| recursive_sum_numbers(val))
               .sum()
        },
        &Value::U64(ref num) => *num as i64,
        &Value::F64(ref num) => *num as i64,
        &Value::I64(ref num) => *num,
        _                    => 0,
    }
}

fn main() {

    let json = read_input(open_file());

    let result = recursive_sum_numbers(&json);

    println!("Sum of all numbers in the input: {:?}", result);
}