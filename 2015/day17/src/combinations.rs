
pub fn combinations<T>(pool: Vec<T>, r: usize) -> Combinations<T>
    where T: Copy
{
    Combinations {
        pool: pool,
        indices: (0..r).collect(),
        r: r,
        state: 0
    }
}

pub struct Combinations<T: Copy> {
    pool: Vec<T>,
    indices: Vec<usize>,
    r: usize,
    state: usize
}

fn make_result<T: Copy>(c: &Combinations<T>) -> Vec<T> {
    let mut v = Vec::with_capacity(c.r);
    for &i in &c.indices {
        v.push(c.pool[i]);
    }
    v
}

impl <T: Copy> Iterator for Combinations<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        // Implementation ported from https://docs.python.org/2/library/itertools.html#itertools.combinations

        // Track the last `i` value from the loop in state 2
        let mut last_index = 0;

        loop {
            match self.state {
                0 => {
                    if self.r > self.pool.len() {
                        return None;
                    }
                    self.state = 1;
                    return Some(make_result(self));
                },
                1 => {
                    // If the following loop completes without changing
                    // `state` then the iterator terminates
                    self.state = 3;
                    let n = self.pool.len();
                    for i in (0..self.r).rev() {
                        last_index = i;
                        if self.indices[i] != i + n - self.r {
                            self.state = 2;
                            break;
                        }
                    }
                },
                2 => {

                    self.state = 1;
                    self.indices[last_index] += 1;
                    for j in last_index + 1 .. self.r {
                        self.indices[j] = self.indices[j - 1] + 1
                    }
                    return Some(make_result(self));

                },
                3 => {
                    return None;
                },
                _ => unreachable!()
            }
        }
    }
}