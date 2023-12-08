extern crate util;

use util::read_input;
use util::error::AppErr;

use winnow::{ Parser, PResult };

#[derive(Debug)]
struct GameRound {
    red: usize,
    green: usize,
    blue: usize,
}

#[derive(Debug)]
struct GameRecord {
    id: usize,
    rounds: Vec<GameRound>,
}

// Let's give winnow a go

fn parse_game(input: &mut &str) -> PResult<GameRecord> {
    use winnow::ascii::{digit1, space0, space1, alpha1};
    use winnow::token::tag;
    use winnow::combinator::{preceded, separated};

    // E.g.
    // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green

    fn parse_score<'a>(input: &mut &'a str) -> PResult<(&'a str, usize)> {
        let (_, digit, _, label, _) = (space0, digit1, space1, alpha1, space0).parse_next(input)?;
        Ok((label, digit.parse().unwrap()))
    }

    fn parse_round(input: &mut &str) -> PResult<GameRound> {

        let things: Vec<_> =
            separated(1..=3, parse_score, tag(","))
                .parse_next(input)?;

        let (mut red, mut green, mut blue) = (0, 0, 0);
        for (tag, score) in things {
            match tag {
                "red"   => red += score,
                "green" => green += score,
                "blue"  => blue += score,
                _ => {}
            }
        }

        Ok(GameRound { red, green, blue })
    }

    let id =
        preceded(
            (tag("Game"), space1),
            digit1
        )
        .parse_next(input)?
        .parse()
        .unwrap();

    let rounds: Vec<_> =
        preceded(
            (space0, tag(":"), space0),
            separated(1.., parse_round, tag(";"))
        )
        .parse_next(input)?;

    Ok(GameRecord { id, rounds })
}

fn main() -> Result<(), AppErr> {
    /*
        --- Part One ---
        The Elf would first like to know which games would have been possible if the bag contained only 12 red cubes, 13 green cubes, and 14 blue cubes?
    */

    let input = read_input("input.txt")?
        .iter()
        .map(|s| parse_game.parse(&s).map_err(|err| AppErr::new("ParseError", &err.to_string())))
        .collect::<Result<Vec<_>, _>>()?;

    let result: usize = input.iter()
        .filter(|g| g.rounds.iter().filter(|r| r.red > 12 || r.green > 13 || r.blue > 14).next().is_none())
        .map(|g| g.id)
        .sum();

    println!("Part 1: {:?}", result);

    Ok(())
}
