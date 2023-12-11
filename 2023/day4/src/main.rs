#![allow(unused)]

extern crate util;

use std::rc::Rc;

use util::read_input;
use util::error::AppErr;
use util::parse::{ParseErr, Input, ParseResultEx};

fn parse_card(input: Input) -> Result<Card, ParseErr> {
    let (input, _) = input.parse_token("Card")?;
    let (input, id) = input.parse_i32()?;
    let (input, _) = input.parse_token(":")?;
    let (input, winning_numbers) = input.parse_repeated(|inp| inp.parse_i32())?;
    let (input, _) = input.parse_token("|")?;
    let (input, chosen_numbers) = input.parse_repeated(|inp| inp.parse_i32())?;
    input.parse_end()?;
    Ok(Card { id, winning_numbers, chosen_numbers })
}

#[derive(Clone, Debug)]
struct Card {
    id: i32,
    winning_numbers: Vec<i32>,
    chosen_numbers: Vec<i32>,
}

impl Card {
    fn win_count(&self) -> u32 {
        self.chosen_numbers.iter()
            .filter(|n| self.winning_numbers.contains(n))
            .count() as u32
    }
}

fn main() -> Result<(), AppErr> {
    /*
        --- Part One ---
        Figure out which of the numbers you have appear in the list of winning numbers.
        The first match makes the card worth one point and each match after the first doubles the point value of that card.
        
        How many points are they worth in total?
    */

    let input = read_input("input.txt")?
        .iter()
        .map(|row| parse_card(Input::new(row)))
        .collect::<Result<Vec<_>, _>>()?;

    fn card_point_value(card: &Card) -> u32 {
        match card.win_count() {
            0 => 0,
            1 => 1,
            n => 2_u32.pow(n - 1)
        }
    }

    let sum = input.iter()
        .map(card_point_value)
        .sum::<u32>();

    println!("Part 1: {}", sum);

    /*
        --- Part Two ---
        ** SNIP: Complex rules about winning copies of cards **

        Process all of the original and copied scratchcards until no more scratchcards are won.
        Including the original set of scratchcards, how many total scratchcards do you end up with?
    */

    #[derive(Clone)]
    struct CardScore {
        id: i32,
        score: usize,
    }

    let source = input.iter()
        .map(|c| CardScore { id: c.id, score: c.win_count() as usize })
        .collect::<Vec<_>>();

    let mut result = source.clone(); 
    let mut i = 0;
    while i < result.len() {
        let CardScore { id, score } = result[i];
        if score > 0 {
            let from = id as usize;
            let to = from + score;
            result.extend_from_slice(&source[from..to]);
        }
        i += 1;
    }

    println!("Part 2: {}", result.len());

    Ok(())
}
