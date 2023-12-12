pub mod error;
pub mod parse;

// Utility for reading input files

use std::path::Path;
use std::io::{ BufRead, BufReader, Error as IoError, Read };
use std::fs::File;

pub fn read_input<P>(input_path: P) -> Result<Vec<String>, IoError>
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

pub fn read_input_to_string<P>(input_path: P) -> Result<String, IoError>
    where P: AsRef<Path>
{
    let mut result = String::new();
    let mut file = File::open(input_path.as_ref())?;
    file.read_to_string(&mut result)?;
    Ok(result)
}

// Utility for strictly consuming iterables

#[derive(Debug)]
pub enum ConsumeIteratorError {
    // Asked empty iterator for a single element.
    IteratorEmpty,
    // Asked for last element on iterator with more than one
    // element left.
    IteratorNotEmpty,
}

pub trait ConsumeIterator: Iterator {
    /// Takes the next item in the iterator, returning an
    /// error if no items remain.
    fn take_next(&mut self) -> Result<Self::Item, ConsumeIteratorError>;
    
    /// Takes the next item in the iterator, returning an
    /// error if no items remain before the operation or if
    /// further items remain after the operation.
    fn take_last(&mut self) -> Result<Self::Item, ConsumeIteratorError>;
}

impl<I> ConsumeIterator for I where I: Iterator {

    fn take_next(&mut self) -> Result<Self::Item, ConsumeIteratorError> {
        match self.next() {
            Some(v) => Ok(v),
            None    => Err(ConsumeIteratorError::IteratorEmpty),
        }
    }
    
    fn take_last(&mut self) -> Result<Self::Item, ConsumeIteratorError> {
        let v = self.take_next()?;
        match self.next() {
            Some(_) => Err(ConsumeIteratorError::IteratorNotEmpty),
            None    => Ok(v)
        }
    }
}