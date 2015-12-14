use std::env::args;
use std::fs::File;
use std::io::{ BufRead, BufReader };

fn open_file() -> File {
    let filename = args().skip(1).next().expect("usage: day8 {input filename}");
    File::open(filename).expect("Error opening input")
}

fn read_quoted_strings(file: File) -> Vec<StringInfo> {
    
    BufReader::new(file)
        .lines()
        .map(|line| line.expect("Error reading file"))
        .map(|line| parse_quoted_string(line).expect("Error parsing line"))
        .collect()
    
}

struct StringInfo {
    source: String,
    parsed: String
}

fn hex_octet(a: char, b: char) -> Option<u8> {
    
    fn hex_char(c: char) -> Option<u8> {
        match c {
            '0'...'9' => Some(c as u8 - '0' as u8),
            'a'...'f' => Some(c as u8 - 'a' as u8 + 10),
            'A'...'F' => Some(c as u8 - 'A' as u8 + 10),
            _         => None
        }
    }
    
    hex_char(a).and_then(|a| hex_char(b).map(|b| (16 * a) + b))
}

fn parse_quoted_string(source: String) -> Result<StringInfo, String> {
    
    let parsed = {
        let mut chars = source.chars();
        
        //Consume the first quote character up front
        match chars.next() {
            Some('"') => {
                //OK!
            },
            Some(c) => return Err(format!("Expected '\"', found '{}'", c)),
            None    => return Err(format!("Expected character, found nothing"))
        }
        
        let mut parsed = String::new();
        let mut found_closing_quote = false;
        
        //Parse the source string one char at a time
        while let Some(c) = chars.next() {
        
            match c {
                '"' => {
                    found_closing_quote = true;
                    break;
                },
                '\\' => {
                    //Read escape sequence
                    match chars.next() {
                        Some('"')  => { parsed.push('"'); },
                        Some('\\') => { parsed.push('\\'); },
                        Some('x')  => {
                            //Read the next two characters from the stream
                            let next_two = chars.next().and_then(|a| chars.next().map(|b| (a, b)));
                            //Parse them as a Hex octet
                            let octet = match next_two {
                                Some((a, b)) => hex_octet(a, b),
                                None         => return Err("Expected ASCII escape sequence, found nothing".into())
                            };
                            match octet {
                                Some(c) => { parsed.push(c as char); },
                                None    => return Err("Invalid ASCII escape sequence".into())
                            };
                        },
                        Some(c) => return Err(format!("Invalid escape sequence: \\{}", c)),
                        None    => return Err(format!("Expected character, found end-of-input"))
                    }
                },
                c => parsed.push(c)
            }
        }
        
        match chars.next() {
            Some(c)                      => return Err(format!("Expected end-of-input, found '{}'", c)),
            None if !found_closing_quote => return Err(format!("Expected '\"', found end-of-input")),
            None => {
                //Ok!
            }
        }
        
        parsed
    };
    
    Ok(StringInfo { source: source, parsed: parsed })
}

fn main() {
    // Disregarding the whitespace in the file, what is the number of characters of code 
    // for string literals minus the number of characters in memory for the values of the
    // strings in total for the entire file?
    
    let lines = read_quoted_strings(open_file());
    let mut weird_sum = 0;
    
    for line in lines {
        let source_chars = line.source.chars().count();
        let parsed_chars = line.parsed.chars().count();
        
        println!("{} ({}) -> {} ({})", line.source, source_chars, line.parsed, parsed_chars);
        
        weird_sum += source_chars - parsed_chars;
    }
    
    println!("Weird sum: {}", weird_sum);
}