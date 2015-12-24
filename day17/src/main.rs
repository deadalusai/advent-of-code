extern crate itertools;

mod combinations;

use itertools::Itertools;
use combinations::combinations_from_iter;

const INPUT: [u32; 20] = [
    33, 14, 18, 20, 45, 35, 16, 35, 1, 13, 18, 13, 50, 44, 48, 6, 24, 41, 30, 42,
];

fn main() {


    let a = [1, 2, 3, 4, 5];

    for (a, b) in a.iter().combinations() {
        println!("{}{}", a, b);
    }

    for set in combinations_from_iter(a.into_iter(), 2) {
        println!("{:?}", set);
    }
}
