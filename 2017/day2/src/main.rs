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
        // Part 1
        // .map(|row| {
        //     let highest = row.iter().max().expect("Unable to find max");
        //     let lowest = row.iter().min().expect("Unable to find min");
        //     highest - lowest
        // })
        // Part 2
        .map(|row| {
            // Find the two numbers in this row which divide evenly, largest over smallest
            for (i, a) in row.iter().enumerate() {
                for (j, b) in row.iter().enumerate() {
                    if i == j {
                        continue;
                    }
                    if a % b == 0 {
                        return a / b;
                    }
                }
            }
            panic!("Unable to find two evenly divisible numbers in row! {:?}", row);
        })
        .sum::<u32>();

    println!("Result: {}", result);
}
