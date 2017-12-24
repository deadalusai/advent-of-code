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
            Ok(Ingredient {
                name:       caps.at(1).unwrap().into(),
                capacity:   caps.at(2).unwrap().parse().unwrap(),
                durability: caps.at(3).unwrap().parse().unwrap(),
                flavor:     caps.at(4).unwrap().parse().unwrap(),
                texture:    caps.at(5).unwrap().parse().unwrap(),
                calories:   caps.at(6).unwrap().parse().unwrap(),
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

    for ing in &ingredients {
        println!("{:?}", ing);
    }

    // Calculate best ingredient mix by recursive brute-force solution

    let best_mix = find_best(&ingredients, 100, Mix::empty(), None);

    println!("Best mix: {:?} ({})", &best_mix, best_mix.score(None));

    let calorie_limit = Some(500);
    let best_mix = find_best(&ingredients, 100, Mix::empty(), calorie_limit);

    println!("Best mix with calorie limit: {:?} ({})", &best_mix, best_mix.score(calorie_limit));
}

#[derive(Debug)]
struct Mix {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32
}

impl Mix {
    fn empty() -> Mix {
        Mix {
            capacity: 0,
            durability: 0,
            flavor: 0,
            texture: 0,
            calories: 0
        }
    }

    fn is_poor_mix(&self, calorie_requirement: Option<i32>) -> bool {
        if let Some(required) = calorie_requirement {
            if self.calories != required {
                return true;
            }
        }
        // Operands swapped because < confuses my syntax highlighting
        1 > self.capacity || 1 > self.durability || 1 > self.flavor || 1 > self.texture
    }

    fn score(&self, calorie_requirement: Option<i32>) -> i32 {
        if self.is_poor_mix(calorie_requirement) { 0 } else { self.capacity * self.durability * self.flavor * self.texture }
    }
}

fn find_best(ingredients: &[Ingredient], max_tsp: i32, input_mix: Mix, calorie_requirement: Option<i32>) -> Mix {

    assert!(ingredients.len() >= 1);
    assert!(max_tsp >= 1);

    if ingredients.len() == 1 {
        //Return the only possible mix of a single ingredient
        let ing = &ingredients[0];
        return Mix {
            capacity:   input_mix.capacity   + (ing.capacity   * max_tsp),
            durability: input_mix.durability + (ing.durability * max_tsp),
            flavor:     input_mix.flavor     + (ing.flavor     * max_tsp),
            texture:    input_mix.texture    + (ing.texture    * max_tsp),
            calories:   input_mix.calories   + (ing.calories   * max_tsp)
        };
    }

    let mut best_mix = Mix::empty();

    for tsp in 0..max_tsp + 1 {
        let ing = &ingredients[0];
        let mix = Mix {
            capacity:   input_mix.capacity   + (ing.capacity   * tsp),
            durability: input_mix.durability + (ing.durability * tsp),
            flavor:     input_mix.flavor     + (ing.flavor     * tsp),
            texture:    input_mix.texture    + (ing.texture    * tsp),
            calories:   input_mix.calories   + (ing.calories   * tsp)
        };

        let mix =
            if max_tsp - tsp <= 0 { mix }
            else                  { find_best(&ingredients[1..], max_tsp - tsp, mix, calorie_requirement) };

        //println!("{}x{} => {} {:?}", tsp, max_tsp - tsp, mix.score(), mix);

        if best_mix.score(calorie_requirement) < mix.score(calorie_requirement) {
            best_mix = mix;
        }
    }

    best_mix
}
