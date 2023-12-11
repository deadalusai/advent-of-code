#![allow(unused)]

extern crate util;

use util::read_input;
use util::error::AppErr;
use util::parse::{ParseErr, Input};

fn parse_card(input: Input) -> Result<Card, ParseErr> {
    let (input, _) = input.parse_token("Card")?;
    let (input, id) = input.parse_i32()?;
    let (input, _) = input.parse_token(":")?;
    let (input, winning_numbers) = input.parse_repeated(|next| next.parse_i32())?;
    let (input, _) = input.parse_token("|")?;
    let (input, chosen_numbers) = input.parse_repeated(|next| next.parse_i32())?;
    input.parse_end()?;
    Ok(Card { id, winning_numbers, chosen_numbers })
}

#[derive(Clone, Debug)]
struct Card {
    id: i32,
    winning_numbers: Vec<i32>,
    chosen_numbers: Vec<i32>,
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

    fn card_point_value(card: &Card) -> i32 {
        let mut sum = 0;
        for n in card.chosen_numbers.iter() {
            if card.winning_numbers.contains(n) {
                sum = match sum { 0 => 1, n => n * 2 };
            }
        }
        sum
    }

    let sum = input.iter()
        .map(card_point_value)
        .sum::<i32>();

    println!("Part 1: {}", sum);

    Ok(())
}
