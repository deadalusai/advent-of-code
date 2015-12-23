extern crate regex;

use std::env::args;
use std::fs::File;
use std::io::{ BufRead, BufReader };
use regex::Regex;

fn open_file() -> File {
    let filename = args().skip(1).next().expect("usage: day14 {input filename}");
    File::open(filename).expect("Error opening input")
}

#[derive(Debug)]
struct Reindeer {
    name: String,
    velocity_kms: i32,
    flight_time_secs: i32,
    rest_time_secs: i32
}

fn parse_reindeer(s: &str) -> Result<Reindeer, String> {

    let pattern = r"^([A-Za-z]+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.$";
    let re = Regex::new(pattern).unwrap();

    match re.captures(s) {
        None => Err("Invalid instruction".into()),
        Some(caps) => {

            let name   = caps.at(1).unwrap().into();
            let vel    = caps.at(2).unwrap().parse().unwrap();
            let f_time = caps.at(3).unwrap().parse().unwrap();
            let r_time = caps.at(4).unwrap().parse().unwrap();

            Ok(Reindeer {
                name: name,
                velocity_kms: vel,
                flight_time_secs: f_time,
                rest_time_secs: r_time
            })
        }
    }
}

fn read_reindeer(file: File) -> Vec<Reindeer> {
    BufReader::new(file)
        .lines()
        .map(|line| line.expect("Error reading line"))
        .map(|line| parse_reindeer(&line).expect("Error parsing instruction"))
        .collect()
}

fn main() {

    let instructions = read_reindeer(open_file());

    let mut states: Vec<_> =
        instructions.iter()
            .map(|r| State {
                reindeer: r,
                kms_travelled: 0,
                resting: false,
                secs: 0
            })
            .collect();

    for _ in 0..2503 {
        for s in states.iter_mut() {
            step(s);
        }
    }

    let best_dist = states.iter().map(|s| s.kms_travelled).max();

    println!("Best distance: {:?}", best_dist);
}

struct State<'a> {
    reindeer: &'a Reindeer,
    kms_travelled: i32,
    resting: bool,

    //Seconds since last state change
    secs: i32,
}

fn step(state: &mut State) {
    //Step by one second
    state.secs += 1;
    if state.resting {
        if state.secs >= state.reindeer.rest_time_secs {
            state.resting = false;
            state.secs = 0;
        }
    }
    else {
        state.kms_travelled += state.reindeer.velocity_kms;

        if state.secs >= state.reindeer.flight_time_secs {
            state.resting = true;
            state.secs = 0;
        }
    }
}
