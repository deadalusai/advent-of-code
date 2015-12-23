
//! Experimental iterative permutator
//! (Takes ownership and requires copy)

pub fn permute<T>(arr: &[T]) -> Permute<T> {
    permute_raw(arr.iter().collect())
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
                let source: Vec<_> =
                    self.source.iter()
                        .enumerate()
                        .filter(|&(i, _)| i != self.index)
                        .map(|(_, v)| v.clone())
                        .collect();

                let inner = Box::new(permute_raw(source));
                self.sub = Some(inner);
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