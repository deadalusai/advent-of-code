extern crate regex;

use std::env::args;
use std::fs::File;
use std::io::{ BufRead, BufReader };
use regex::Regex;

fn open_file() -> File {
    let filename = args().skip(1).next().expect("usage: day15 {input filename}");
    File::open(filename).expect("Error opening input")
}

#[derive(Debug)]
struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32
}

fn parse_ingredient(s: &str) -> Result<Ingredient, String> {

    let pattern = r"([A-Za-z]+): capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)";
    let re = Regex::new(pattern).unwrap();

    match re.captures(s) {
        None => Err("Unable to parse ingredient".into()),
        Some(caps) => {
            let name       = caps.at(1).unwrap().into();
            let capacity   = caps.at(2).unwrap().parse().unwrap();
            let durability = caps.at(3).unwrap().parse().unwrap();
            let flavor     = caps.at(4).unwrap().parse().unwrap();
            let texture    = caps.at(5).unwrap().parse().unwrap();
            let calories   = caps.at(6).unwrap().parse().unwrap();

            Ok(Ingredient {
                name: name,
                capacity: capacity,
                durability: calories,
                flavor: texture,
                texture: flavor,
                calories: durability,
            })
        }
    }
}

fn read_input(file: File) -> Vec<Ingredient> {
    BufReader::new(file)
        .lines()
        .map(|line| line.expect("Error reading line"))
        .map(|line| parse_ingredient(&line).expect("Error parsing ingredient"))
        .collect()
}

fn main() {

    let ingredients = read_input(open_file());

    println!("Ingredients: {:?}", ingredients);
}
