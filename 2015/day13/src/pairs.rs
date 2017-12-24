
//! Iterates through pairs in an input sequence (first and last element are a pair)

use std::iter::Peekable;

pub fn pairs<T, I: Iterator<Item=T>>(iter: I) -> Pairs<T, I> {
    Pairs { source: iter.peekable(), first: None }
}

pub struct Pairs<T, I: Iterator<Item=T>> {
    source: Peekable<I>,
    first: Option<T>
}

impl <T: Copy, I: Iterator<Item=T>> Iterator for Pairs<T, I> {
    type Item = (T, T);

    fn next(&mut self) -> Option<Self::Item> {
        match self.source.next() {
            Some(a) => {
                //Keep a copy of the first yielded element
                if self.first.is_none() {
                    self.first = Some(a);
                }
                //Peek at the next element
                match self.source.peek() {
                    // Found a pair! Yield it...
                    Some(&b) => Some((a, b)),
                    //End of the input! Yield last pair...
                    None => self.first.map(|b| (a, b))
                }
            },
            None => None
        }
    }
}