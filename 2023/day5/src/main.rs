#![allow(unused)]

extern crate util;

use std::ops::RangeInclusive;
use std::rc::Rc;

use util::{read_input_to_string};
use util::error::AppErr;
use util::parse::{ParseErr, Input, ParseResultEx, ParseResult};

fn parse_almanac(input: Input) -> Result<Almanac, ParseErr> {

    fn parse_mapping<'a>(input: Input<'a>) -> ParseResult<'a, Mapping> {
        let (input, label_from) = input.parse_alpha()?;
        let (input, ()) = input.parse_token_sequence([ "-", "to", "-" ])?;
        let (input, label_to) = input.parse_alpha()?;
        let (input, ()) = input.parse_token_sequence([ "map", ":" ])?;
        let (input, _) = input.parse_newline()?;
        let (input, ranges) = input.parse_repeated(|next| {
            let (next, dest_start) = next.parse_i64()?;
            let (next, source_start) = next.parse_i64()?;
            let (next, length) = next.parse_i64()?;
            let (next, ()) = next.parse_newline()?;
            Ok((next, Range { source_start, dest_start, length }))
        })?;
        Ok((input, Mapping {
            name: format!("{} to {}", label_from, label_to),
            ranges
        }))
    }
    
    let (input, ())    = input.parse_token_sequence([ "seeds", ":" ])?;
    let (input, seeds) = input.parse_repeated(|next| next.parse_i64())?;
    let (input, ())    = input.parse_newline()?;
    let (input, mappings) = input.parse_repeated(parse_mapping)?;
    input.parse_end()?;

    Ok(Almanac {
        seeds,
        mappings
    })
}

#[derive(Debug)]
struct Range {
    pub source_start: i64,
    pub dest_start: i64,
    pub length: i64,
}

impl Range {
    /// Translates {input} from the source range to the destination range
    fn map(&self, input: i64) -> Option<i64> {
        if input < self.source_start {
            return None;
        }
        let shifted = input - self.source_start;
        if shifted >= self.length {
            return None;
        }
        Some(self.dest_start + shifted)
    }
}

#[derive(Debug)]
struct Mapping {
    name: String,
    ranges: Vec<Range>,
}

impl Mapping {
    fn map(&self, input: i64) -> i64 {
        self.ranges
            .iter().filter_map(|r| r.map(input))
            .next().unwrap_or(input)
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<i64>,
    mappings: Vec<Mapping>,
}

impl Almanac {
    fn map(&self, input: i64) -> i64 {
        self.mappings
            .iter()
            .fold(input, |input, el| el.map(input))
    }
}

fn main() -> Result<(), AppErr> {
    /*
        --- Part One ---
        What is the lowest location number that corresponds to any of the initial seed numbers?
    */

    let input = read_input_to_string("input.txt")?;
    let almanac = parse_almanac(Input::new(&input))?;

    let result = almanac.seeds
        .iter()
        .map(|&input| almanac.map(input))
        .min()
        .ok_or("Expected minimum value")?;

    println!("Part 1: {}", result);

    /*
        --- Part Two ---
        
    */

    println!("Part 2: {}", "TODO");

    Ok(())
}
