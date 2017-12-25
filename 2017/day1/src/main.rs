extern crate input;

fn char_to_number (c: char) -> u32 {
    match c {
        '0'...'9' => (c as u32) - ('0' as u32),
        _________ => panic!("Unexpeted input! '{}'", c)
    }
}

fn main() {
    let input = input::read_input("./input.txt").unwrap();

    let numbers: Vec<_> = input.iter()
        .flat_map(|line| line.trim().chars())
        .map(char_to_number)
        .collect();
    
    let mut sum = 0;
    let len = numbers.len();
    // Day 1 Part 1
    // let step = 1;
    // Day 1 Part 2
    let step = len / 2;

    for i in 0..len {
        
        let a = numbers[i];
        let b = numbers[(i + step) % len];

        if a == b {
            sum += a;
        }
    }

    println!("Sum: {}", sum);
}
