#![allow(unused)]

extern crate util;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum State {
    Live,
    Dead
}

#[derive(Debug)]
struct Rule {
    matches: [State; 5],
    result: State,
}

fn main() {

    fn parse_state(c: char) -> State {
        use State::*;
        match c {
            '.' => Dead,
            '#' => Live,
            _ => panic!("invalid state char {}", c),
        }
    }

    fn parse_initial_state(s: &str) -> Vec<State> {
        // initial state: #..#.#..##......###...###
        // ignore the preamble
        let s = &s["initial state: ".len()..];
        s.chars().map(parse_state).collect::<Vec<_>>()
    }

    fn parse_rule(s: &str) -> Rule {
        // ...## => #
        let mut parts = s.split(" => ");
        let mut matches = [State::Dead; 5];
        for (i, state) in parts.next().unwrap().chars().map(parse_state).enumerate().take(5) {
            matches[i] = state;
        }
        let result = parse_state(parts.next().unwrap().chars().next().unwrap());
        Rule {
            matches: matches,
            result: result
        }
    }

    /*
    --- Part One ---
    The pots are numbered, with 0 in front of you. To the left, the pots are numbered -1, -2, -3, and so on; to the
    right, 1, 2, 3.... Your puzzle input contains a list of pots from 0 to the right and whether they do (#) or do
    not (.) currently contain a plant, the initial state. (No other pots currently contain plants.) For example, an
    initial state of #..##.... indicates that pots 0, 3, and 4 currently contain plants.

    Your puzzle input also contains some notes you find on a nearby table: someone has been trying to figure out how
    these plants spread to nearby pots. Based on the notes, for each generation of plants, a given pot has or does
    not have a plant based on whether that pot (and the two pots on either side of it) had a plant in the last generation.
    These are written as LLCRR => N, where L are pots to the left, C is the current pot being considered, R are the pots
    to the right, and N is whether the current pot will have a plant in the next generation. For example:

    - A note like ..#.. => . means that a pot that contains a plant but with no plants within two pots of it will not have a plant in it during the next generation.
    - A note like ##.## => . means that an empty pot with two plants on each side of it will remain empty in the next generation.
    - A note like .##.# => # means that a pot has a plant in a given generation if, in the previous generation, there were plants in that pot, the one immediately to the left, and the one two pots to the right, but not in the ones immediately to the right and two to the left.
    
    It's not clear what these plants are for, but you're sure it's important, so you'd like to make sure the current
    configuration of plants is sustainable by determining what will happen after 20 generations.

    In this example, after 20 generations, the pots shown as # contain plants, the furthest left of which
    is pot -2, and the furthest right of which is pot 34. Adding up all the numbers of plant-containing pots after the 20th generation produces 325.
    */

    let mut input = util::read_input("input.txt").unwrap().into_iter();
    let initial_state = parse_initial_state(&input.next().unwrap());
    let rules = input.skip(1).map(|s| parse_rule(&s)).collect::<Vec<_>>();

    fn run_simulation(generations: usize, initial_state: &[State], rules: &[Rule]) -> isize {
        use State::*;
        let start = std::time::SystemTime::now();

        // Hax - build a state vector which is larger than the size of our initial state.
        // Position zero begins in the middle of the vector

        const WORLD_SIZE: usize = 180;
        const ZERO_START: usize = 60;

        let len = initial_state.len();
        let world_size = len * WORLD_SIZE;
        let zero_start = len * ZERO_START;
        let mut state = vec![Dead; world_size];
        let mut state_next = state.clone();
        
        // Initialise the world state
        for (s, is) in state.iter_mut().skip(zero_start).take(len).zip(initial_state) {
            *s = *is;
        }

        let dead = &[Dead; 5];
        let get_next_state = |window: &[State]| {
            if window == dead {
                return Dead;
            }
            match rules.iter().filter(|r| &r.matches == window).next() {
                Some(rule) => rule.result,
                None       => Dead,
            }
        };
    
        for gen in 1..=generations {
            for (i, window) in state.windows(5).enumerate() {
                // We update the pot in the middle of each window
                state_next[i + 2] = get_next_state(window);
            }
            std::mem::swap(&mut state, &mut state_next);
        }

        /*
        After N generations, what is the sum of the numbers of all pots which contain a plant?
        */
        let result = state.iter().enumerate()
            .map(|(i, s)| match s {
                Live => (i as isize - zero_start as isize),
                Dead => 0,
            })
            .sum::<isize>();

        result
    }

    let result = run_simulation(20, &initial_state, &rules);

    println!("Part 1 result: {}", result);

    /*
    --- Part Two ---
    You realize that 20 generations aren't enough. After all, these plants will
    need to last another 1500 years to even reach your timeline, not to mention your future.

    After fifty billion (50000000000) generations, what is the sum of the numbers of all pots which contain a plant?
    */

    // NOTE: This simulation is massive! However after the initial iterations the amount seems to
    // increase by a fixed amount every iteration...
    // 
    // Gather some samples, then do some arithmetic...
    let sample_a = run_simulation(2000, &initial_state, &rules);
    let sample_b = run_simulation(3000, &initial_state, &rules);
    let sample_c = run_simulation(4000, &initial_state, &rules);
    
    println!("Sample (   0-2000) delta of {}", sample_a);
    println!("Sample (2000-3000) delta of {}", sample_b - sample_a);
    println!("Sample (3000-4000) delta of {}", sample_c - sample_b);

    let simulation_size = 50_000_000_000;
    let total = ((simulation_size - 2000) / 1000) * (sample_b - sample_a) + sample_a;
    
    println!("Part 2 result: {}", total);
}