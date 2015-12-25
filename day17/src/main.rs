#![feature(iter_arith)]

mod combinations;

use combinations::combinations;

fn main() {

    let target = 150;
    let input = vec![
        33, 14, 18, 20, 45, 35, 16, 35, 1, 13, 18, 13, 50, 44, 48, 6, 24, 41, 30, 42,
        //20, 15, 10, 5, 5
    ];

    let mut count = 0;

    for size in 2 .. input.len() {
        for set in combinations(input.clone(), size) {

            let sum: u32 = set.iter().sum();
            if sum == target {
                //println!("{:?}", &set);
                count += 1;
            }
        }
    }

    println!("Count: {}", count);
}
