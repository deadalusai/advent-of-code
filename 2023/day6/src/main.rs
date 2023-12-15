#![allow(unused)]

extern crate util;

use util::read_input_to_string;
use util::error::AppErr;
use util::parse::{ParseErr, Input, ParseResultEx};

struct Record {
    race_time_ms: i32,
    distance_mm: i32,
}

fn parse_records(input: &str) -> Result<Vec<Record>, AppErr> {
    let input = Input::new(input);
    let (input, ()) = input.parse_token_sequence(["Time", ":"])?;
    let (input, times) = input.parse_repeated(|input| input.parse_i32())?;
    let (input, ()) = input.parse_newline()?;
    let (input, ()) = input.parse_token_sequence(["Distance", ":"])?;
    let (input, distances) = input.parse_repeated(|input| input.parse_i32())?;
    let (input, ()) = input.parse_end()?;

    let results = times.into_iter()
        .zip(distances)
        .map(|(race_time_ms, distance_mm)| Record { race_time_ms, distance_mm })
        .collect::<Vec<_>>();

    Ok(results)
}

struct Race {
    time_held_ms: i32,
    time_limit_ms: i32
}

impl Race {
    fn distance_travelled_mm(&self) -> Option<i32> {
        if self.time_held_ms == 0 && self.time_held_ms >= self.time_limit_ms {
            // Can't win if you never start the race
            return None;
        }
        let rate_of_acceleration_mm_ms2 = self.time_held_ms; // 1mm/ms per ms button is held
        let remaining_ms = self.time_limit_ms - self.time_held_ms;
        Some(remaining_ms * rate_of_acceleration_mm_ms2)
    }
}

fn main() -> Result<(), AppErr> {
    /*
        --- Part One ---

        Determine the number of ways you could beat the record in each race. What do you get if you multiply these numbers together?
    */

    let input = read_input_to_string("input.txt")?;
    let records = parse_records(&input)?;

    fn enumerate_winning_strategies(record: &Record) -> i32 {
        (1..record.race_time_ms)
            .into_iter()
            .map(|t| Race { time_held_ms: t, time_limit_ms: record.race_time_ms })
            .filter_map(|r| r.distance_travelled_mm())
            .filter(|&d| d > record.distance_mm)
            .count() as i32
    }

    let result = records
        .iter()
        .map(enumerate_winning_strategies)
        .product::<i32>();

    println!("Part 1: {}", result);

    /*
        --- Part Two ---

    */

    println!("Part 2: {}", "TODO");

    Ok(())
}
