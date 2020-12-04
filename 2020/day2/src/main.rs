extern crate util;
extern crate itertools;

use util::{ read_input, ConsumeIterator };
use util::error::{ AppErr };

fn main() -> Result<(), AppErr> {
    /*
    --- Part One ---
    Their password database seems to be a little corrupted: some of the passwords wouldn't
    have been allowed by the Official Toboggan Corporate Policy that was in effect when they
    were chosen.

    To try to debug the problem, they have created a list (your puzzle input) of passwords
    (according to the corrupted database) and the corporate policy when that password was set.

    For example, suppose you have the following list:

        1-3 a: abcde
        1-3 b: cdefg
        2-9 c: ccccccccc

    Each line gives the password policy and then the password. The password policy indicates
    the lowest and highest number of times a given letter must appear for the password to be
    valid. For example, 1-3 a means that the password must contain a at least 1 time and at
    most 3 times.

    In the above example, 2 passwords are valid. The middle password, cdefg, is not; it
    contains no instances of b, but needs at least 1. The first and third passwords are
    valid: they contain one a or nine c, both within the limits of their respective policies.

    How many passwords are valid according to their policies?
    */

    #[derive(Debug)]
    struct PasswordPolicy {
        min: usize,
        max: usize,
        character: char,
    }
    
    fn parse_item(s: &str) -> Result<(PasswordPolicy, String), AppErr> {
        let mut parts = s.split(":");
        // PasswordPolicy
        let mut policy_parts = parts.take_next()?.split(|c| c == '-' || c == ' ');
        let policy = PasswordPolicy {
            min: policy_parts.take_next()?.parse::<usize>()?,
            max: policy_parts.take_next()?.parse::<usize>()?,
            character: policy_parts.take_last()?.chars().take_last()?
        };
        // Password
        let password = parts.take_last()?.trim().to_string();
        Ok((policy, password))
    }

    let input = 
        read_input("input.txt")?
            .iter()
            .map(|s| parse_item(s))
            .collect::<Result<Vec<_>, AppErr>>()?;

    fn is_password_valid(policy: &PasswordPolicy, password: &str) -> bool {
        let count = password.chars()
            .filter(|c| *c == policy.character)
            .count();
        count <= policy.max && count >= policy.min
    }

    let valid_count = input.iter()
        .filter(|(policy, pass)| is_password_valid(policy, pass))
        .count();

    println!("Part 1: Valid passwords: {}", valid_count);

    /*
    --- Part Two ---
    Each policy actually describes two positions in the password, where 1 means the first
    character, 2 means the second character, and so on. (Be careful; Toboggan Corporate
    Policies have no concept of "index zero"!) Exactly one of these positions must
    contain the given letter. Other occurrences of the letter are irrelevant for the
    purposes of policy enforcement.

    Given the same example list from above:

        1-3 a: abcde is valid: position 1 contains a and position 3 does not.
        1-3 b: cdefg is invalid: neither position 1 nor position 3 contains b.
        2-9 c: ccccccccc is invalid: both position 2 and position 9 contain c.

    How many passwords are valid according to the new interpretation of the policies?
    */

    fn is_password_valid_v2(policy: &PasswordPolicy, password: &str) -> bool {
        password.char_indices()
            .filter(|(i, _)| *i == (policy.min - 1) || *i == (policy.max - 1))
            .filter(|(_, c)| *c == policy.character)
            .count() == 1
    }

    let valid_count = input.iter()
        .filter(|(policy, pass)| is_password_valid_v2(policy, pass))
        .count();

    println!("Part 2: Valid passwords: {}", valid_count);

    Ok(())
}
