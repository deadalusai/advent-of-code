#![allow(unused)]

extern crate util;

use std::iter::Peekable;

use util::read_input;
use util::error::AppErr;

type Card = i32;

#[derive(Debug, Copy, Clone)]
enum HandScore {
    /// Five of a kind, where all five cards have the same label: AAAAA
    FiveOfAKind = 6,
    /// Four of a kind, where four cards have the same label and one card has a different label: AA8AA
    FourOfAKind = 5,
    /// Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
    FullHouse = 4,
    /// Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
    ThreeOfAKind = 3,
    /// Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
    TwoPair = 2,
    /// One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
    OnePair = 1,
    /// High card, where all cards' labels are distinct: 23456
    HighCard = 0,
}

impl HandScore {
    fn rank(&self) -> i32 {
        *self as i32
    }
}

#[derive(Debug)]
struct Hand {
    cards: [Card; 5],
    score: HandScore,
    bid: i32,
}

impl std::cmp::PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other)
            .map(|ord| ord == std::cmp::Ordering::Equal)
            .unwrap_or(false)
    }
}

impl std::cmp::PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering::*;
        Some(self.score.rank().cmp(&other.score.rank())
            .then(self.cards[0].cmp(&other.cards[0]))
            .then(self.cards[1].cmp(&other.cards[1]))
            .then(self.cards[2].cmp(&other.cards[2]))
            .then(self.cards[3].cmp(&other.cards[3]))
            .then(self.cards[4].cmp(&other.cards[4])))
    }
}

fn score_hand(cards: &[Card; 5]) -> HandScore {
    #[derive(Debug)]
    struct Set {
        card: Card,
        count: i32,
    }

    let mut cards = cards.clone();
    cards.sort_by(|a, b| a.cmp(b).reverse());

    // Count groups of cards in the hand
    let mut iter = cards.into_iter().peekable();
    let mut count_cards = move || {
        let card = iter.next()?;
        let mut count = 1;
        while let Some(&c) = iter.peek().filter(|&&c| c == card) {
            count += 1;
            iter.next();
        }
        Some(Set { card, count })
    };

    let mut groups = (0..5).flat_map(|_| count_cards()).collect::<Vec<_>>();
    groups.sort_by(|a, b| a.count.cmp(&b.count).reverse());

    match groups.as_slice() {
        &[Set { count: 5, .. }]                          => HandScore::FiveOfAKind,
        &[Set { count: 4, .. }, _]                       => HandScore::FourOfAKind,
        &[Set { count: 3, .. }, Set { count: 2, .. }]    => HandScore::FullHouse,
        &[Set { count: 3, .. }, _, _]                    => HandScore::ThreeOfAKind,
        &[Set { count: 2, .. }, Set { count: 2, .. }, _] => HandScore::TwoPair,
        &[Set { count: 2, .. }, _, _, _]                 => HandScore::OnePair,
        &[Set { count: 1, .. }, _, _, _, _]              => HandScore::HighCard,
        _ => panic!("Unexpected hand: {:?}", groups)
    }
}

fn parse_hand(line: &str) -> Result<Hand, AppErr> {
    fn char_to_card(char: char) -> Card {
        match char {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            '9' => 9,
            '8' => 8,
            '7' => 7,
            '6' => 6,
            '5' => 5,
            '4' => 4,
            '3' => 3,
            '2' => 2,
            c => panic!("{} not a valid card", c)
        }
    }

    let (left, right) = line.split_at(5);
    let mut cards = [0; 5];
    for (i, card) in left.chars().map(char_to_card).enumerate() {
        cards[i] = card;
    }

    let bid = right.trim().parse::<i32>()?;
    Ok(Hand {
        cards,
        score: score_hand(&cards),
        bid
    })
}

fn main() -> Result<(), AppErr> {
    /*
        --- Part One ---
        Find the rank of every hand in your set. What are the total winnings?
    */

    let mut input = read_input("input.txt")?
        .iter()
        .map(|line| parse_hand(line))
        .collect::<Result<Vec<_>, _>>()?;

    // Sort the input by hand strength
    input.sort_by(|a, b| a.partial_cmp(&b).unwrap());

    let result = input.iter()
        .enumerate()
        .map(|(rank, hand)| (rank as i32 + 1) * hand.bid)
        .sum::<i32>();

    println!("Part 1: {}", result);

    /*
        --- Part Two ---

    */

    println!("Part 2: {}", "TODO");

    Ok(())
}
