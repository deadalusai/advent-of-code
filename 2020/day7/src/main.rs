extern crate util;

use util::{ read_input };
use util::error::{ AppErr };

fn main() -> Result<(), AppErr> {
    /*
    --- Part One ---
    
    The input is a list of rules which describe what kinds of bags can
    contain what other kinds of bags, in what quantities.

    How many bag colors can eventually contain at least one shiny gold bag?
    (The list of rules is quite long; make sure you get all of it.)
    
    */

    type BagName = String;

    #[derive(Debug)]
    struct BagRule {
        name: BagName,
        rules: Vec<(BagName, i32)>,
    }

    // Could have done this with just string splits,
    // but it's more fun to write a parser combinator right?

    #[derive(Debug)]
    enum ParseErr {
        NoInput,
        Unexpected(char),
        Expected(String),
        InvalidNumber(String),
    }

    type ParseResult<'a, T> = Result<(&'a str, T), ParseErr>;

    fn consume<'a>(s: &'a str) -> ParseResult<&'a str> {
        let first = s.chars().next()
            .ok_or(ParseErr::NoInput)?;
        // Special characters
        if first == '.' || first == ',' {
            let (a, b) = s.split_at(1);
            return Ok((b.trim(), a.trim()));
        }
        // Tokens are made up of characters of the same type as the first character.
        let allowed = match first {
            c if c.is_alphabetic() => 'a'..='z',
            c if c.is_numeric() => '0'..='9',
            c => return Err(ParseErr::Unexpected(c)),
        };
        let (last, _) = s.char_indices()
            .take_while(|(_, c)| allowed.contains(c))
            .last()
            .unwrap();
        let (a, b) = s.split_at(last + 1);
        Ok((b.trim(), a.trim()))
    }

    fn parse_i32(input: &str) -> ParseResult<i32> {
        let (input, num) = consume(input)?;
        let num = num.parse::<i32>().map_err(|_| ParseErr::InvalidNumber(num.to_string()))?;
        Ok((input, num))
    }

    fn parse_token<'a>(input: &'a str, token: &str) -> ParseResult<'a, &'a str> {
        let (input, actual) = consume(input)?;
        if actual != token {
            return Err(ParseErr::Expected(token.to_string()));
        }
        Ok((input, actual))
    }

    fn parse_name(input: &str) -> ParseResult<String> {
        let (input, word1) = consume(input)?;
        let (input, word2) = consume(input)?;
        Ok((input, format!("{} {}", word1, word2)))
    }

    // `a b` bags contain n `c d` bags, 1 `e f` bag, no `g h` bags, no other bags.

    fn parse_rule<'a>(input: &'a str) -> ParseResult<Option<(String, i32)>> {
        // Attempt to consume the "no rules" rule.
        if let Ok((input, _)) = parse_token(input, "no") {
            let (input, _) = parse_token(input, "other")?;
            let (input, _) = parse_token(input, "bags")?;
            return Ok((input, None));
        }

        let (input, num) = parse_i32(input)?;
        let (input, name) = parse_name(input)?;
        let (input, _) = parse_token(input, if num == 1 { "bag" } else { "bags" })?;
        Ok((input, Some((name, num))))
    }

    fn parse_bag<'a>(input: &'a str) -> Result<BagRule, ParseErr> {
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
            let (next, term) = parse_token(next, ",").or_else(|_| parse_token(next, "."))?;
            input = next;
            if term == "." {
                break;
            }
        }
        assert_eq!(input, "");
        Ok(BagRule { name, rules })
    }

    let input = read_input("input.txt")?
        .iter()
        .map(|line| {
            parse_bag(line)
                .map_err(|e| AppErr::from_debug("parse error", &e))
        })
        .collect::<Result<Vec<_>, _>>()?;

    for bag in &input {
        println!("{:?}", bag);
    }

    Ok(())
}
