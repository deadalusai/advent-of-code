extern crate util;

use util::{ read_input };
use util::error::{ AppErr };

use std::collections::hash_set::{ HashSet };

#[derive(Clone, Copy, Eq, PartialEq)]
enum Polarity { Pos, Neg }

#[derive(Clone, Copy)]
struct Component { 
    name: char,
    polarity: Polarity,
}

fn main() -> Result<(), AppErr> {
    use Polarity::*;

    fn parse_component(c: char) -> Component {
        Component {
            name: c.to_uppercase().next().unwrap(),
            polarity: if c.is_uppercase() { Pos } else { Neg },
        }
    }

    let input =
        read_input("input.txt")?
            .into_iter().next().unwrap()
            .chars()
            .map(|c| parse_component(c))
            .collect::<Vec<_>>();
    
    /*
    --- Part One ---
    The polymer is formed by smaller units which, when triggered, react with each other such that two adjacent units of the same type and opposite polarity are destroyed. Units' types are represented by letters; units' polarity is represented by capitalization. For instance, r and R are units with the same type but opposite polarity, whereas r and s are entirely different types and do not react.

    For example:

    In aA, a and A react, leaving nothing behind.
    In abBA, bB destroys itself, leaving aA. As above, this then destroys itself, leaving nothing.
    In abAB, no two adjacent units are of the same type, and so nothing happens.
    In aabAAB, even though aa and AA are of the same type, their polarities match, and so nothing happens.
    Now, consider a larger example, dabAcCaCBAcCcaDA:

    dabAcCaCBAcCcaDA  The first 'cC' is removed.
    dabAaCBAcCcaDA    This creates 'Aa', which is removed.
    dabCBAcCcaDA      Either 'cC' or 'Cc' are removed (the result is the same).
    dabCBAcaDA        No further actions can be taken.
    After all possible reactions, the resulting polymer contains 10 units.

    How many units remain after fully reacting the polymer you scanned?
    */

    fn reduce(input: Vec<Component>) -> Vec<Component> {
        // Iteratively reduce the list until no
        // further reactive pairs are found
        let mut list = input;
        let mut temp = Vec::with_capacity(list.len());
        loop {
            // Scan the input seqence for adjacent, reactive pairs
            {
                let mut iter = list.iter().peekable();
                loop {
                    let c1 = match iter.next() {
                        Some(&c1) => c1,
                        None => break
                    };
                    let c2 = match iter.peek() { 
                        Some(&c2) => c2,
                        None => {
                            // End of input
                            temp.push(c1);
                            break;
                        }
                    };
                    // Check for a reaction...
                    if c1.name == c2.name && c1.polarity != c2.polarity {
                        // Consume c1 AND c2
                        iter.next().unwrap();
                        continue;
                    }
                    temp.push(c1);
                }
            }
            if list.len() == temp.len() {
                // No reactions found? We're done.
                break;
            }
            std::mem::swap(&mut list, &mut temp);
            temp.clear();
        }
        list.shrink_to_fit();
        list
    }

    let result = reduce(input.clone());
    
    println!("Part 1 result: {}", result.len());

    /*
    --- Part Two ---
    Time to improve the polymer.

    One of the unit types is causing problems; it's preventing the polymer from collapsing as much as it should.
    Your goal is to figure out which unit type is causing the most problems, remove all instances of it (regardless of polarity),
    fully react the remaining polymer, and measure its length.

    For example, again using the polymer dabAcCaCBAcCcaDA from above:

    Removing all A/a units produces dbcCCBcCcD. Fully reacting this polymer produces dbCBcD, which has length 6.
    Removing all B/b units produces daAcCaCAcCcaDA. Fully reacting this polymer produces daCAcaDA, which has length 8.
    Removing all C/c units produces dabAaBAaDA. Fully reacting this polymer produces daDA, which has length 4.
    Removing all D/d units produces abAcCaCBAcCcaA. Fully reacting this polymer produces abCBAc, which has length 6.
    In this example, removing all C/c units was best, producing the answer 4.

    What is the length of the shortest polymer you can produce by removing all units of exactly one type and fully reacting the result?
    */

    // Scan for unit types
    let mut unit_types = HashSet::new();
    for c in &input {
        unit_types.insert(c.name);
    }

    let (name, result) =
        unit_types.iter()
            .map(|&name| {
                let mut input = input.clone();
                input.retain(|c| c.name != name);
                (name, reduce(input).len())
            })
            .min_by_key(|&(_, len)| len)
            .unwrap();

    println!("Part 2 result: {} ({})", result, name);

    Ok(())
}
