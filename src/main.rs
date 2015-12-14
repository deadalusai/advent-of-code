use std::env::args;
use std::fs::File;
use std::io::{ BufRead, BufReader };

use std::collections::HashMap;

fn open_file() -> File {
    let filename = args().skip(1).next().expect("usage: day9 {input filename}");
    File::open(filename).expect("Error opening input")
}

type KMs = u32;

#[derive(Debug)]
struct Distance {
    from: String,
    to: String,
    distance: KMs
}

fn read_input_file(file: File) -> Vec<Distance> {
    
    BufReader::new(file)
        .lines()
        .map(|line| line.expect("Error reading file"))
        .map(|line| parse_distance(&line).expect("Error parsing line"))
        .collect()
}

fn parse_distance(s: &str) -> Result<Distance, String> {
    let mut parts = s.split(" ");
    let from = match parts.next() { Some(s) => s.into(),  o => return Err(format!("Expected FROM, got {:?}", o)) };
               match parts.next() { Some("to") => { },    o => return Err(format!("Expected `to`, got {:?}", o)) };
    let to =   match parts.next() { Some(s) => s.into(),  o => return Err(format!("Expected TO, got {:?}", o))   };
               match parts.next() { Some("=") =>  { },    o => return Err(format!("Expected `=`, got {:?}", o))  };
    let dist = match parts.next() { Some(s) => s.parse(), o => return Err(format!("Expected DIST, got {:?}", o)) };
    
    let dist = match dist { Ok(d) => d, Err(e) => return Err(format!("Error parsing distance: {}", e)) };
    
    Ok(Distance { from: from, to: to, distance: dist })
}

#[derive(Debug)]
struct City {
    distances: HashMap<String, KMs>
}

fn update_map(map: &mut HashMap<String, City>, from: &str, to: &str, dist: KMs) {
    
    let city = map.entry(from.into()).or_insert_with(|| City { distances: HashMap::new() });
            
    city.distances.insert(to.into(), dist);
}

fn main() {
    
    let distances = read_input_file(open_file());
    
    let mut map = HashMap::new();
    
    for dist in distances {
        //Locate an existing cities in the map and update them
        update_map(&mut map, &dist.from, &dist.to, dist.distance);
        update_map(&mut map, &dist.to, &dist.from, dist.distance);
    }
    
    for (key, value) in map.iter() { 
        println!("{} = {:?}", key, value);
    }
}