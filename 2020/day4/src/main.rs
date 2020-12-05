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

    type PassportData<'a> = HashMap<&'a str, &'a str>;

    fn try_read_passport<'a>(input: &mut dyn Iterator<Item=&'a str>) -> Result<Option<PassportData<'a>>, AppErr> {
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

        Ok(Some(hash))
    }

    let input = read_input("input.txt")?;
    let mut input_reader = input.iter().map(|s| s.as_str());
    let mut passports = Vec::new();
    while let Some(passport) = try_read_passport(&mut input_reader)? {
        passports.push(passport);
    }

    fn is_passport_valid_part_1(passport: &PassportData) -> bool {
        // NOTE: cid is optional
        passport.get("byr").is_some() &&
        passport.get("iyr").is_some() &&
        passport.get("eyr").is_some() &&
        passport.get("hgt").is_some() &&
        passport.get("hcl").is_some() &&
        passport.get("ecl").is_some() &&
        passport.get("pid").is_some()
    }

    let valid_count_part_1 = passports.iter().filter(|p| is_passport_valid_part_1(p)).count();

    println!("Part 1: {} valid passports", valid_count_part_1);

    /*
    --- Part Two ---
    You can continue to ignore the cid field, but each other field has strict
    rules about what values are valid for automatic validation:

    byr (Birth Year) - four digits; at least 1920 and at most 2002.
    iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    hgt (Height) - a number followed by either cm or in:
        If cm, the number must be at least 150 and at most 193.
        If in, the number must be at least 59 and at most 76.
    hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    pid (Passport ID) - a nine-digit number, including leading zeroes.
    cid (Country ID) - ignored, missing or not.

    */

    fn is_passport_valid_part_2(passport: &PassportData) -> bool {
        fn validate_year(year: &str, min: i32, max: i32) -> bool {
            year.len() == 4 &&
            year.parse::<i32>()
                .map(|year| year >= min && year <= max)
                .unwrap_or(false)
        }
        fn validate_height(height: &str) -> bool {
            enum Measure { Cm, In };
            struct Height(u32, Measure);
            fn try_parse_height(s: &str) -> Result<Height, AppErr> {
                let unit = match s {
                    u if u.ends_with("cm") => Ok(Measure::Cm),
                    u if u.ends_with("in") => Ok(Measure::In),
                    u => Err(AppErr::from_debug("invalid measure", &u)),
                }?;
                let value = s[..s.len() - 2].parse::<u32>()?;
                Ok(Height(value, unit))
            }
            try_parse_height(height)
                .map(|h| match h {
                    Height(v, Measure::Cm) => v >= 150 && v <= 193,
                    Height(v, Measure::In) => v >= 59  && v <= 76,
                })
                .unwrap_or(false)
        }
        fn validate_hair_color(color: &str) -> bool {
            color.starts_with("#") &&
            match &color[1..] {
                hex =>
                    hex.len() == 6 &&
                    hex.chars().all(|c| c.is_ascii_hexdigit())
            }
        }
        fn validate_eye_color(color: &str) -> bool {
            match color {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                _ => false,
            }
        }
        fn validate_passport_id(pid: &str) -> bool {
            pid.len() == 9 &&
            pid.parse::<i32>().is_ok()
        }

        let byr = passport.get("byr").filter(|s| validate_year(s, 1920, 2002));
        let iyr = passport.get("iyr").filter(|s| validate_year(s, 2010, 2020));
        let eyr = passport.get("eyr").filter(|s| validate_year(s, 2020, 2030));
        let hgt = passport.get("hgt").filter(|s| validate_height(s));
        let hcl = passport.get("hcl").filter(|s| validate_hair_color(s));
        let ecl = passport.get("ecl").filter(|s| validate_eye_color(s));
        let pid = passport.get("pid").filter(|s| validate_passport_id(s));
        // NOTE: cid is optional
        
        byr.and(iyr).and(eyr).and(hgt).and(hcl).and(ecl).and(pid).is_some()
    }

    let valid_count_part_2 = passports.iter().filter(|p| is_passport_valid_part_2(p)).count();

    println!("Part 2: {} valid passports", valid_count_part_2);

    Ok(())
}
