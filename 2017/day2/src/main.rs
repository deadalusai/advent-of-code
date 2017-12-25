extern crate input;

fn main() {
    let input = input::read_input("input.txt").unwrap();

    let result = input.iter()
        .map(|line| {
            line.trim()
                .split("\t")
                .map(|field| field.parse::<u32>().expect("Unable to parse input field"))
                .collect::<Vec<_>>()
        })
        .map(|row| {
            let highest = row.iter().max().expect("Unable to find max");
            let lowest = row.iter().min().expect("Unable to find min");
            highest - lowest
        })
        .sum::<u32>();

    println!("Result: {}", result);
}
