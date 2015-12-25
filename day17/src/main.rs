#![feature(iter_arith)]

mod combinations;

use combinations::combinations;

fn main() {

    let target = 150;
    let input = vec![
        33, 14, 18, 20, 45, 35, 16, 35, 1, 13, 18, 13, 50, 44, 48, 6, 24, 41, 30, 42,
        //20, 15, 10, 5, 5
    ];

    let mut all_sets = Vec::new();

    for size in 2 .. input.len() {
        for set in combinations(input.clone(), size) {

            let sum: u32 = set.iter().sum();
            if sum == target {
                all_sets.push(set);
            }
        }
    }

    println!("Count: {}", all_sets.len());

    let shortest_len = all_sets.iter().map(|s| s.len()).min().unwrap();

    println!("Shortest length: {}", shortest_len);

    let short_set_count = all_sets.iter().filter(|s| s.len() == shortest_len).count();

    println!("Short set count: {}", short_set_count);
}
