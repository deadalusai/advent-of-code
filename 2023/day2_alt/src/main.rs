extern crate util;

use util::read_input;
use util::error::AppErr;
use util::parse::{Input, ParseResult, ParseResultEx};

#[derive(Debug, Default)]
struct GameRound {
    red: i32,
    green: i32,
    blue: i32,
}

#[derive(Debug)]
struct GameRecord {
    id: i32,
    rounds: Vec<GameRound>,
}

fn parse_game(input: Input) -> ParseResult<GameRecord> {
    enum _Term { EndScore, EndRound, EndRecord }
    use _Term::*;

    enum _Color { Red, Green, Blue }
    use _Color::*;

    // E.g.
    // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    let (input, _) = input.parse_token("Game")?;
    let (input, id) = input.parse_i32()?;
    let (input, _) = input.parse_token(":")?;

    // parse rounds
    let mut rounds = Vec::new();
    let mut input = input;
    loop {
        let mut round = GameRound::default();
        // Parse scores
        loop {
            let next = input;
            // {count} {color}
            let (next, score) = next.parse_i32()?;
            let (next, color) = next.parse_token("red").val(Red)
                .or_try(|| next.parse_token("green").val(Green))
                .or_try(|| next.parse_token("blue").val(Blue))?;
            match color {
                Red   => round.red += score,
                Green => round.green += score,
                Blue  => round.blue += score,
            }
            // scan for end of score, end of round or end of input
            let (next, term) = next.parse_token(",").val(EndScore)
                .or_try(|| next.parse_token(";").val(EndRound))
                .or_try(|| next.parse_end().val(EndRecord))?;
            input = next;
            
            match term {
                EndScore => {
                    continue;
                },
                EndRound => {
                    rounds.push(round);
                    break;
                },
                EndRecord => {
                    rounds.push(round);
                    return Ok((input, GameRecord { id, rounds }))
                },
            }
        }
    }
}

fn main() -> Result<(), AppErr> {
    /*
        --- Part One ---
        The Elf would first like to know which games would have been possible if the bag contained only 12 red cubes, 13 green cubes, and 14 blue cubes?
    */

    let input = read_input("input.txt")?
        .iter()
        .map(|s| parse_game(Input::new(s)).map(|r| r.1))
        .collect::<Result<Vec<_>, _>>()?;

    let result: i32 = input.iter()
        .filter(|g| g.rounds.iter().filter(|r| r.red > 12 || r.green > 13 || r.blue > 14).next().is_none())
        .map(|g| g.id)
        .sum();

    println!("Part 1: {:?}", result);

    /*
        --- Part Two ---
        For each game, find the minimum set of cubes that must have been present. What is the sum of the power of these sets?
    */

    let result: i32 = input.iter()
        // Find the minimum cubes required to play each game
        .map(|g| g.rounds.iter()
                .map(|r| (r.red, r.green, r.blue))
                .reduce(|(r1, g1, b1), (r2, g2, b2)| (r1.max(r2), g1.max(g2), b1.max(b2)))
                .unwrap_or_default())
        // Find the power for each minimum set
        .map(|(r, g, b)| r * g * b)
        .sum();

    println!("Part 2: {:?}", result);

    Ok(())
}
