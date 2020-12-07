extern crate util;

use std::collections::{ HashSet };

use util::{ read_input };
use util::parse::{ parse_i32, parse_alpha, parse_token, parse_end, ParseResult, ParseResultEx };
use util::error::{ AppErr };

fn main() -> Result<(), AppErr> {
    /*
    --- Part One ---
    
    The input is a list of rules which describe what kinds of bags can
    contain what other kinds of bags, in what quantities.

    How many bag colors can eventually contain at least one shiny gold bag?
    (The list of rules is quite long; make sure you get all of it.)
    
    */

    #[derive(Debug)]
    struct BagRule {
        name: String,
        rules: Vec<(String, i32)>,
    }

    impl BagRule {
        fn bag_can_hold(&self, name: &str) -> bool {
            self.rules.iter().any(|r| r.0 == name)
        }
    }

    // Could have done this with just string splits,
    // but it's more fun to write a parser combinator right?

    // `a b` bags contain n `c d` bags, 1 `e f` bag, no `g h` bags, no other bags.

    fn parse_name(input: &str) -> ParseResult<String> {
        let (input, word1) = parse_alpha(input)?;
        let (input, word2) = parse_alpha(input)?;
        Ok((input, format!("{} {}", word1, word2)))
    }

    fn parse_rule(input: &str) -> ParseResult<Option<(String, i32)>> {
        
        fn no_bags(input: &str) -> ParseResult<Option<(String, i32)>> {
            let (input, _) = parse_token(input, "no")?;
            let (input, _) = parse_token(input, "other")?;
            let (input, _) = parse_token(input, "bags")?;
            Ok((input, None))
        }
        
        fn some_bags(input: &str) -> ParseResult<Option<(String, i32)>> {
            let (input, num) = parse_i32(input)?;
            let (input, name) = parse_name(input)?;
            let (input, _) = parse_token(input, if num == 1 { "bag" } else { "bags" })?;
            Ok((input, Some((name, num))))
        }

        no_bags(input).or_try(|| some_bags(input))
    }

    fn parse_bag(input: &str) -> ParseResult<BagRule> {
        let (input, name) = parse_name(input)?;
        let (input, _) = parse_token(input, "bags")?;
        let (input, _) = parse_token(input, "contain")?;
        let mut rules = Vec::new();
        let mut input = input;
        loop {
            let (next, rule) = parse_rule(input)?;
            if let Some(rule) = rule {
                rules.push(rule);
            }
            // Continue scanning for more rules?
            let (next, term) = parse_token(next, ",").or_try(|| parse_token(next, "."))?;
            input = next;
            if term == "." {
                break;
            }
        }
        let (input, _) = parse_end(input)?;
        Ok((input, BagRule { name, rules }))
    }

    let bags = read_input("input.txt")?
        .iter()
        .map(|line| {
            parse_bag(line)
                .map(|r| r.1)
                .map_err(|e| AppErr::from_debug("parse error", &e))
        })
        .collect::<Result<Vec<_>, _>>()?;

    fn can_hold_count<'a>(bag_name: &'a str, bags: &'a [BagRule], seen: &mut HashSet<&'a str>) -> u32 {
        let mut count = 0;
        for bag in bags {
            if bag.bag_can_hold(bag_name) && seen.insert(&bag.name) {
                count += 1;
                count += can_hold_count(&bag.name, bags, seen);
            }
        }
        count
    }

    println!("Part 1: {} bags can hold the shiny gold bag", can_hold_count("shiny gold", &bags, &mut HashSet::new()));

    /*
    --- Part Two ---

    Consider again your shiny gold bag and the rules from the above example:

        faded blue bags contain 0 other bags.
        dotted black bags contain 0 other bags.
        vibrant plum bags contain 11 other bags: 5 faded blue bags and 6 dotted black bags.
        dark olive bags contain 7 other bags: 3 faded blue bags and 4 dotted black bags.

    So, a single shiny gold bag must contain 1 dark olive bag (and the 7 bags within it)
    plus 2 vibrant plum bags (and the 11 bags within each of those):
        1 + 1*7 + 2 + 2*11 = 32 bags!

    Of course, the actual rules have a small chance of going several levels deeper than this example;
    be sure to count all of the bags, even if the nesting becomes topologically impractical!

    How many individual bags are required inside your single shiny gold bag?
    */

    let mut bags = bags;
    bags.sort_by(|a, b| a.name.cmp(&b.name));

    fn should_hold_count<'a>(bag_name: &'a str, bags: &'a [BagRule]) -> i32 {
        let bag_index = bags.binary_search_by_key(&bag_name, |b| &b.name).unwrap();
        bags[bag_index].rules.iter()
            .map(|(bag_name, count)| {
                (1 + should_hold_count(bag_name, bags)) * count
            })
            .sum::<i32>()
    }

    println!("Part 2: {} bags can be held by the shiny gold bag", should_hold_count("shiny gold", &bags));

    Ok(())
}