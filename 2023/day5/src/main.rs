#![allow(unused)]

extern crate util;

use util::{read_input_to_string};
use util::error::AppErr;
use util::parse::{ParseErr, Input, ParseResultEx, ParseResult};

use rayon::iter::{ParallelIterator, IntoParallelRefIterator};

fn parse_almanac(input: Input) -> Result<Almanac, ParseErr> {

    fn parse_mapping(input: Input) -> ParseResult<Mapping> {
        let (input, dest_start) = input.parse_i64()?;
        let (input, source_start) = input.parse_i64()?;
        let (input, length) = input.parse_i64()?;
        let (input, ()) = input.parse_newline()?;
        Ok((input, Mapping { source_start, dest_start, length }))
    }

    fn parse_mapping_set<'a>(input: Input<'a>) -> ParseResult<'a, MappingSet> {
        let (input, label_from) = input.parse_alpha()?;
        let (input, ()) = input.parse_token_sequence([ "-", "to", "-" ])?;
        let (input, label_to) = input.parse_alpha()?;
        let (input, ()) = input.parse_token_sequence([ "map", ":" ])?;
        let (input, ()) = input.parse_newline()?;
        let (input, ranges) = input.parse_repeated(parse_mapping)?;
        Ok((input, MappingSet {
            name: format!("{} to {}", label_from, label_to),
            ranges
        }))
    }
    
    let (input, ())    = input.parse_token_sequence([ "seeds", ":" ])?;
    let (input, seeds) = input.parse_repeated(|next| next.parse_i64())?;
    let (input, ())    = input.parse_newline()?;
    let (input, mappings) = input.parse_repeated(parse_mapping_set)?;
    input.parse_end()?;

    Ok(Almanac {
        seeds,
        mappings
    })
}

#[derive(Debug)]
struct Mapping {
    pub source_start: i64,
    pub dest_start: i64,
    pub length: i64,
}

impl Mapping {
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
struct MappingSet {
    name: String,
    ranges: Vec<Mapping>,
}

impl MappingSet {
    fn map(&self, input: i64) -> i64 {
        self.ranges
            .iter().filter_map(|r| r.map(input))
            .next().unwrap_or(input)
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<i64>,
    mappings: Vec<MappingSet>,
}

impl Almanac {
    fn map(&self, input: i64) -> i64 {
        self.mappings
            .iter()
            .fold(input, |input, el| el.map(input))
    }
    
    fn seed_ranges(&self) -> Vec<std::ops::Range<i64>> {
        self.seeds.chunks(2)
            .map(|w| w[0]..(w[0] + w[1]))
            .collect()
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
        The input seed numbers are actually pairs of numbers describing [start, length] ranges.

        What is the lowest location number that corresponds to any of the initial seed numbers?
    */

    let result = almanac
        .seed_ranges()
        .par_iter()
        .flat_map(|range| range.clone().into_iter())
        .map(|input| almanac.map(input))
        .min()
        .ok_or("Expected minimum value")?;

    println!("Part 2: {}", result);

    Ok(())
}
