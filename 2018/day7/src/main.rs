extern crate util;
extern crate itertools;
extern crate regex;

use util::{ read_input };
use itertools::Itertools;
use std::collections::hash_map::HashMap;
use std::cell::RefCell;

fn main() {

    type Id = char;

    #[derive(Debug)]
    struct Step {
        pub id: Id,
        pub depends_on: Id,
    }

    let pattern = regex::Regex::new("^Step ([A-Z]) must be finished before step ([A-Z]) can begin.$").unwrap();

    let input =
        read_input("input.txt").unwrap().iter()
            .map(|s| {
                let caps = pattern.captures(&s).unwrap();
                Step {
                    id: caps[2].chars().next().unwrap(),
                    depends_on: caps[1].chars().next().unwrap(),
                }
            })
            .collect::<Vec<_>>();

    /*
    --- Part One ---
    The instructions specify a series of steps and requirements about which steps must be finished before others can begin (your puzzle input).
    Each step is designated by a single letter. For example, suppose you have the following instructions:

    Step C must be finished before step A can begin.
    Step C must be finished before step F can begin.
    Step A must be finished before step B can begin.
    Step A must be finished before step D can begin.
    Step B must be finished before step E can begin.
    Step D must be finished before step E can begin.
    Step F must be finished before step E can begin.
    Visually, these requirements look like this:

     -->A--->B--
    /    \       \
    C     -->D----->E
    \            /
     ---->F-----

    Your first goal is to determine the order in which the steps should be completed. If more than one step is ready,
    choose the step which is first alphabetically. In this example, the steps would be completed as follows:

    Only C is available, and so it is done first.
    Next, both A and F are available. A is first alphabetically, so it is done next.
    Then, even though F was available earlier, steps B and D are now also available, and B is the first alphabetically of the three.
    After that, only D and F are available. E is not available because only some of its prerequisites are complete. Therefore, D is completed next.
    F is the only choice, so it is done next.
    Finally, E is completed.
    So, in this example, the correct order is CABDFE.

    In what order should the steps in your instructions be completed?
    */

    // Build a lookup of step dependencies
    let mut step_dependencies = HashMap::new();
    for step in input.iter() {
        let init = || Vec::new();
        step_dependencies.entry(step.depends_on).or_insert_with(init);
        step_dependencies.entry(step.id).or_insert_with(init).push(step.depends_on);
    }

    // Get a sorted list of all steps
    let available_steps = {
        let mut temp = step_dependencies.keys().cloned().collect::<Vec<_>>();
        temp.sort();
        temp
    };

    // Repeatedly scan the list for the next step which
    // a) is not yet completed
    // b) has all its dependencies completed
    let mut completed_steps: Vec<char> = Vec::new();
    loop {
        let step = available_steps.iter()
            .filter(|step| {
                let is_completed = || completed_steps.contains(step);
                let are_dependencies_met = || step_dependencies.get(step).unwrap().iter().all(|dep| completed_steps.contains(dep));
                !is_completed() && are_dependencies_met()
            })
            .next();

        match step {
            Some(&step) => completed_steps.push(step),
            None => break
        }
    }

    println!("Part 1 result: {}", completed_steps.iter().join(""));

    /*
    --- Part Two ---
    As you're about to begin construction, four of the Elves offer to help. "The sun will set soon; it'll go faster if we work together."
    Now, you need to account for multiple people working on steps simultaneously. If multiple steps are available, workers should still begin them in alphabetical order.

    Each step takes 60 seconds plus an amount corresponding to its letter: A=1, B=2, C=3, and so on.
    So, step A takes 60+1=61 seconds, while step Z takes 60+26=86 seconds. No time is required between steps.

    To simplify things for the example, however, suppose you only have help from one Elf (a total of two workers) and
    that each step takes 60 fewer seconds (so that step A takes 1 second and step Z takes 26 seconds). Then, using the
    same instructions as above, this is how each second would be spent:

    Second   Worker 1   Worker 2   Done
    0         C          .        
    1         C          .        
    2         C          .        
    3         A          F         C
    4         B          F         CA
    5         B          F         CA
    6         D          F         CAB
    7         D          F         CAB
    8         D          F         CAB
    9         D          .         CABF
    10        E          .         CABFD
    11        E          .         CABFD
    12        E          .         CABFD
    13        E          .         CABFD
    14        E          .         CABFD
    15        .          .         CABFDE
   
    Each row represents one second of time. The Second column identifies how many seconds have passed as
    of the beginning of that second. Each worker column shows the step that worker is currently
    doing (or . if they are idle). The Done column shows completed steps.

    Note that the order of the steps has changed; this is because steps now take time to finish and multiple
    workers can begin multiple steps simultaneously.

    In this example, it would take 15 seconds for two workers to complete these steps.

    With 5 workers and the 60+ second step durations described above, how long will it take to complete all of the steps?
    */
    #[derive(Copy, Clone)]
    struct Work {
        step: Id,
        time_remaining: u32,
    }

    // NOTE: This puzzle requires us to interrogate a collection (the worker state) while we modify it
    // RefCell to the rescue

    let assigned_steps = vec![RefCell::new(None as Option<Work>); 5];
    let mut completed_steps = Vec::new();
    let mut time = 0;
    loop {
        for assigned in assigned_steps.iter() {
            let mut work_ref = assigned.borrow_mut();
            if let Some(work) = *work_ref {
                // Work on the assigned step
                if work.time_remaining > 1 {
                    *work_ref = Some(Work { time_remaining: work.time_remaining - 1, ..work });
                }
                else {
                    completed_steps.push(work.step);
                    *work_ref = None;
                }
            };
            // All done? Get new work
            if work_ref.is_none() {
                *work_ref = available_steps.iter()
                    .filter(|step| {
                        let is_completed = || completed_steps.contains(step);
                        let is_in_progress = || assigned_steps.iter().any(|a| a.try_borrow().ok().and_then(|b| b.map(|w| w.step == **step)).unwrap_or(false));
                        let are_dependencies_met = || step_dependencies.get(step).unwrap().iter().all(|dep| completed_steps.contains(dep));
                        !is_completed() && !is_in_progress() && are_dependencies_met()
                    })
                    .map(|&step| Work {
                        step: step,
                        time_remaining: (step as u32 - 'A' as u32) + 61,
                    })
                    .next();
            }
        }
        if assigned_steps.iter().all(|s| s.borrow().is_none()) {
            break;
        }
        time += 1;
    }

    println!("Part 2 result: {} in {}s", completed_steps.iter().join(""), time);
}
