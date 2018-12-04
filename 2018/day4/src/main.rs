extern crate util;
extern crate regex;

use util::{ read_input };
use util::error::{ AppErr };
use regex::{ Regex };

use std::collections::hash_map::{ HashMap };

fn main() -> Result<(), AppErr> {
    type Id = u32;
    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
    enum Kind {
        ShiftStart(Id),
        Sleep,
        Wake,
    }
    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
    struct Event {
        pub year: u32,
        pub month: u32,
        pub day: u32,
        pub hour: u32,
        pub minute: u32,
        pub kind: Kind,
    }
    
    let parse_event = {
        // [1518-04-05 00:00] Guard #131 begins shift
        // [1518-08-28 00:12] falls asleep
        // [1518-06-06 00:25] wakes up
        let event_matcher = Regex::new(r"^\[(\d+)-(\d+)-(\d+) (\d+):(\d+)\] (.*)$").unwrap();
        let id_matcher = Regex::new(r"#(\d+)").unwrap();
        // Result<Event, AppErr>
        move |line: &str| {
            let parts = event_matcher.captures(line).unwrap();
            Event {
                year: parts.get(1).unwrap().as_str().parse().unwrap(),
                month: parts.get(2).unwrap().as_str().parse().unwrap(),
                day: parts.get(3).unwrap().as_str().parse().unwrap(),
                hour: parts.get(4).unwrap().as_str().parse().unwrap(),
                minute: parts.get(5).unwrap().as_str().parse().unwrap(),
                kind: match parts.get(6).unwrap().as_str() {
                    "falls asleep" => Kind::Sleep,
                    "wakes up" => Kind::Wake,
                    s => {
                        let id = id_matcher.captures(s).unwrap().get(1).unwrap().as_str().parse().unwrap();
                        Kind::ShiftStart(id)
                    }
                }
            }
        }
    };

    let mut events =
        read_input("input.txt")?
            .into_iter()
            .map(|line| parse_event(&line))
            .collect::<Vec<_>>();

    events.sort();

    /*
    --- Part One ---
    Strategy 1: Find the guard that has the most minutes asleep. What minute does that guard spend asleep the most?

    In the example above, Guard #10 spent the most minutes asleep, a total of 50 minutes (20+25+5), while
    Guard #99 only slept for a total of 30 minutes (10+10+10). Guard #10 was asleep most during minute
    24 (on two days, whereas any other minute the guard was asleep was only seen on one day).

    While this example listed the entries in chronological order, your entries are in the order you found them.
    You'll need to organize them before they can be analyzed.

    What is the ID of the guard you chose multiplied by the minute you chose?
    (In the above example, the answer would be 10 * 24 = 240.)
    */
    let mut guard_map = HashMap::new();
    let mut guard_id: Id = 0;
    let mut minute_sleep_started: Option<u32> = None;
    for event in events {
        let mut guard_woke_up = false;
        let mut next_guard_id = guard_id;
        match event.kind {
            Kind::ShiftStart(id) => {
                next_guard_id = id;
                guard_woke_up = minute_sleep_started.is_some();
            },
            Kind::Wake => {
                guard_woke_up = minute_sleep_started.is_some();
            }
            Kind::Sleep => {
                minute_sleep_started = Some(event.minute);
            },
        }
        if guard_woke_up && guard_id != 0 {
            let sleep_minutes = guard_map.entry(guard_id).or_insert_with(|| [0_u32; 60]);
            for minute in minute_sleep_started.unwrap()..event.minute {
                sleep_minutes[minute as usize] += 1;
            }
            minute_sleep_started = None;
        }
        guard_id = next_guard_id;
    }

    // Locate the sleepiest guard...
    let (&id, sleep_minutes) =
        guard_map.iter().max_by_key(|(_, sleep_minutes)| sleep_minutes.iter().sum::<u32>()).unwrap();

    // And find the sleepiest minute
    let (minute, _) =
        sleep_minutes.iter().enumerate().max_by_key(|(_, &count)| count).unwrap();

    println!("Part 1 result: {}", id * minute as u32);

    /*
    --- Part Two ---
    Strategy 2: Of all guards, which guard is most frequently asleep on the same minute?

    In the example above, Guard #99 spent minute 45 asleep more than any other guard or minute - three times in total.
    (In all other cases, any guard spent any minute asleep at most twice.)

    What is the ID of the guard you chose multiplied by the minute you chose?
    (In the above example, the answer would be 99 * 45 = 4455.)
    */

    // Locate the guard sleepiest on a particular minute...
    let (id, minute, _) =
        guard_map.iter().fold((0, 0, 0), |(last_id, last_minute, last_count), (&id, sleep_minutes)| {
            let (most_slept_minute, &count) = sleep_minutes.iter().enumerate().max_by_key(|(_, &count)| count).unwrap();
            if count > last_count {
                (id, most_slept_minute, count)
            } else {
                (last_id, last_minute, last_count) 
            }
        });

    println!("Part 2 result: {}", id * minute as u32);

    Ok(())
}
