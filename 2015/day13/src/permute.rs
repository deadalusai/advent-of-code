
//! Experimental iterative permutator

pub fn permute<'a, T: 'a, I: Iterator<Item=&'a T>>(iter: I) -> Permute<'a, T> {
    permute_raw(iter.collect())
}

fn permute_raw<'a, T: 'a>(arr: Vec<&'a T>) -> Permute<'a, T> {
    Permute { source: arr, index: 0, sub: None }
}

pub struct Permute<'a, T: 'a> {
    source: Vec<&'a T>,
    index: usize,
    sub: Option<Box<Permute<'a, T>>>,
}

impl <'a, T> Iterator for Permute<'a, T> {
    type Item = Vec<&'a T>;

    fn next(&mut self) -> Option<Self::Item> {

        // Simple case - source of one or less elements
        if self.source.len() <= 1 {
            let result = match self.index {
                0 => Some(self.source.clone()),
                _ => None
            };
            self.index += 1;
            return result;
        }

        // Simple case - source of two elements
        if self.source.len() == 2 {
            let result = match self.index {
                0 => Some(self.source.clone()),
                1 => Some(vec![self.source[1], self.source[0]]),
                _ => None
            };
            self.index += 1;
            return result;
        }

        loop {
            // Done iterating?
            if self.index >= self.source.len() {
                return None;
            }

            // Start the sub-iterator?
            if self.sub.is_none() {
                // Permute `source` excepting the current element
                let source =
                    self.source.iter().enumerate()
                        .filter_map(|(i, v)| if i != self.index { Some(*v) } else { None })
                        .collect();

                let inner = Box::new(permute_raw(source));
                self.sub = Some(inner);
            }

            // Get the next result from the sub-iterator?
            if let Some(mut v) = self.sub.as_mut().unwrap().next() {
                // Got a result? Push the current element onto the end and return
                v.push(self.source[self.index]);
                return Some(v);
            }

            // Sub-iterator finished? Reset it and step.
            self.sub = None;
            self.index += 1;
        }
    }
}