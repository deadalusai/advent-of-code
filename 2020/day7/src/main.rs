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
        rules: Vec<(BagName, u32)>,
    }

    // Could have done this with just string splits,
    // but it's more fun to write a parser combinator right?

    #[derive(Debug)]
    enum ConsumeErr {
        NothingConsumed,
        ExpectedToken,
        InvalidNumber,
    }

    type ConsumeResult<'a, T> = Result<(&'a str, T), ConsumeErr>;

    fn consume<'a>(s: &'a str, mut m: impl FnMut(&char) -> bool) -> ConsumeResult<&'a str> {
        let last = s.char_indices().take_while(|(_, c)| m(c)).last();
        match last {
            None => Err(ConsumeErr::NothingConsumed),
            Some((i, _)) => {
                let (a, b) = s.split_at(i + 1);
                // NOTE: always swallow whitespace
                Ok((b.trim(), a.trim()))
            },
        }
    }

    fn consume_term(input: &str) -> ConsumeResult<&str> {
        consume(input, |c| *c == ',' || *c == '.')
    }

    fn consume_numeric(input: &str) -> ConsumeResult<&str> {
        consume(input, |c| c.is_numeric())
    }

    fn consume_token(input: &str) -> ConsumeResult<&str> {
        consume(input, |c| c.is_alphabetic())
    }

    fn consume_exact<'a>(input: &'a str, tokens: &[&str]) -> ConsumeResult<'a, ()> {
        let mut input = input;
        for t in tokens.iter() {
            let (input_, token) = consume_token(input)?;
            if *t != token {
                return Err(ConsumeErr::ExpectedToken);
            }
            input = input_;
        }
        Ok((input, ()))
    }

    fn consume_name(input: &str) -> ConsumeResult<String> {
        let (input, word1) = consume_token(input)?;
        let (input, word2) = consume_token(input)?;
        Ok((input, format!("{} {}", word1, word2)))
    }

    // `a b` bags contain n `c d` bags, 1 `e f` bag, no `g h` bags.

    fn consume_rule<'a>(input: &'a str) -> ConsumeResult<Option<(String, u32)>> {
        if let Ok((input, num)) = consume_numeric(input) {
            let num = num.parse::<u32>().map_err(|_| ConsumeErr::InvalidNumber)?;
            let (input, name) = consume_name(input)?;
            let (input, _) = consume_exact(input, &[if num == 1 { "bag" } else { "bags" }])?;
            let (input, _) = consume_term(input)?;
            return Ok((input, Some((name, num))));
        }
        else {
            let (input, _) = consume_exact(input, &["no", "other", "bags"])?;
            let (input, _) = consume_term(input)?;
            assert_eq!(input, "");
            return Ok((input, None));
        }
    }

    fn consume_bag<'a>(input: &'a str) -> Result<BagRule, ConsumeErr> {
        let (input, name) = consume_name(input)?;
        let (input, _) = consume_exact(input, &["bags", "contain"])?;
        let mut rules = Vec::new();
        let mut input = input;
        while let Ok((input_, rule)) = consume_rule(input) {
            input = input_;
            if let Some(rule) = rule {
                rules.push(rule);
            }
        }
        assert_eq!(input, "");
        Ok(BagRule { name, rules })
    }

    let input = read_input("input.txt")?
        .iter()
        .map(|line| consume_bag(line))
        .collect::<Vec<_>>();

    for bag in &input {
        println!("{:?}", bag);
    }

    Ok(())
}
