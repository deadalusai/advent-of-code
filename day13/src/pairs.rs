
//! Iterates through pairs in an array (first and last element are a pair)

pub fn pairs<'a, T>(source: &'a [T]) -> Pairs<'a, T> {
    Pairs { source: source, i: 0 }
}

pub struct Pairs<'a, T: 'a> {
    source: &'a [T],
    i: usize
}

impl <'a, T> Iterator for Pairs<'a, T> {
    type Item = (&'a T, &'a T);

    fn next(&mut self) -> Option<Self::Item> {

        let len = self.source.len();
        let next = match self.i {
            i if i == len - 1 => Some((&self.source[i], &self.source[0])),
            i if i >= len     => None,
            i                 => Some((&self.source[i], &self.source[i + 1]))
        };

        self.i += 1;
        next
    }
}