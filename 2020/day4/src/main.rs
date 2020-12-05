extern crate util;

use std::collections::{ HashMap };

use util::{ read_input, ConsumeIterator };
use util::error::{ AppErr };

fn main() -> Result<(), AppErr> {
    /*
    --- Part One ---
    
    The automatic passport scanners are slow because they're having trouble
    detecting which passports have all required fields.
    The expected fields are as follows:

        byr (Birth Year)
        iyr (Issue Year)
        eyr (Expiration Year)
        hgt (Height)
        hcl (Hair Color)
        ecl (Eye Color)
        pid (Passport ID)
        cid (Country ID)

    Passport data is validated in batch files (your puzzle input).
    Each passport is represented as a sequence of key:value pairs separated
    by spaces or newlines. Passports are separated by blank lines.

    E.g. one passport:
        ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
        byr:1937 iyr:2017 cid:147 hgt:183cm

    Count the number of valid passports - those that have all required fields.
    Treat cid as optional. In your batch file, how many passports are valid?
    */

    struct Passport<'a> {
        byr: Option<&'a str>,
        iyr: Option<&'a str>,
        eyr: Option<&'a str>,
        hgt: Option<&'a str>,
        hcl: Option<&'a str>,
        ecl: Option<&'a str>,
        pid: Option<&'a str>,
        cid: Option<&'a str>,
    }

    fn try_read_passport<'a>(input: &mut dyn Iterator<Item=&'a String>) -> Result<Option<Passport<'a>>, AppErr> {
        let hash =  input
            .take_while(|line| line.trim().len() > 0)
            .flat_map(|line| line.split(" "))
            .map(|segment| {
                let mut pair = segment.split(":");
                let key = pair.take_next()?.trim();
                let value = pair.take_last()?.trim();
                Ok((key, value))
            })
            .collect::<Result<HashMap<_, _>, AppErr>>()?;
        
        if hash.len() == 0 {
            return Ok(None);
        }

        let passport = Passport {
            byr: hash.get("byr").copied(),
            iyr: hash.get("iyr").copied(),
            eyr: hash.get("eyr").copied(),
            hgt: hash.get("hgt").copied(),
            hcl: hash.get("hcl").copied(),
            ecl: hash.get("ecl").copied(),
            pid: hash.get("pid").copied(),
            cid: hash.get("cid").copied(),
        };

        Ok(Some(passport))
    }

    fn is_passport_valid(passport: &Passport) -> bool {
        // NOTE: cid is optional
        passport.byr.is_some() &&
            passport.iyr.is_some() &&
            passport.eyr.is_some() &&
            passport.hgt.is_some() &&
            passport.hcl.is_some() &&
            passport.ecl.is_some() &&
            passport.pid.is_some()
    }

    let input = read_input("input.txt")?;
    let mut input_reader = input.iter();
    
    let mut valid_count = 0;
    while let Some(passport) = try_read_passport(&mut input_reader)? {
        if is_passport_valid(&passport) {
            valid_count += 1;
        }
    }

    println!("Part 1: {} valid passports", valid_count);

    Ok(())
}
