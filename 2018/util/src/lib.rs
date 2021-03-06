pub mod error;

use std::path::{ Path };
use std::io::{ BufRead, BufReader, Error as IoError };
use std::fs::{ File };

pub fn read_input <P> (input_path: P) -> Result<Vec<String>, IoError>
    where P: AsRef<Path>
{
    let mut result = Vec::new();
    
    let file = File::open(input_path.as_ref())?;
    let mut input = BufReader::new(file);

    let mut line = String::new();
    loop {
        if input.read_line(&mut line)? == 0 {
            break;
        }
        result.push(line.trim_end().to_string());
        line.clear();
    }
    Ok(result)
}
