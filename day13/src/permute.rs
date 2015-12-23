
//! Experimental iterative permutator
//! (Takes ownership and requires copy)

pub struct Permute<T> {
    source: Vec<T>,
    index: usize,
    sub: Option<Box<Permute<T>>>,
}

impl <T: Clone> Iterator for Permute<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {

        // Simple case - source of one elements
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
                1 => Some(vec![self.source[1].clone(), self.source[0].clone()]),
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

            if self.sub.is_none() {
                let source = self.source.iter()
                                 .enumerate()
                                 .filter(|&(i, _)| i != self.index)
                                 .map(|(_, v)| v.clone())
                                 .collect();

                self.sub = Some(Box::new(permute(source)));
            }

            let next = match self.sub.as_mut().unwrap().next() {
                None => None,
                Some(mut v) => {
                    v.insert(0, self.source[self.index].clone());
                    Some(v)
                }
            };

            if next.is_none() {
                //Reached the end of the sub iterator!
                self.sub = None;
                self.index += 1;
            }
            else {
                return next;
            }
        }
    }
}

pub fn permute<T>(arr: Vec<T>) -> Permute<T> {
    Permute { source: arr, index: 0, sub: None }
}