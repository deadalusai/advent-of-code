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

type Map = HashMap<String, City>; 

#[derive(Debug)]
struct City {
    distances: HashMap<String, KMs>
}

fn build_map(distances: Vec<Distance>) -> Map {
    
    let mut map = HashMap::new();
    
    fn update_map(map: &mut Map, from: &str, to: &str, dist: KMs) {
        let city = map.entry(from.into()).or_insert_with(|| City { distances: HashMap::new() });
        city.distances.insert(to.into(), dist);
    }
    
    for dist in distances {
        //Locate an existing cities in the map and update them
        update_map(&mut map, &dist.from, &dist.to, dist.distance);
        update_map(&mut map, &dist.to, &dist.from, dist.distance);
    }
    
    map
}

fn nearest_neighbour<'a>(map: &'a Map, city_name: &'a str, visited: &Vec<&'a str>) -> Option<(&'a str, KMs)> {
    
    let mut nearest: Option<(&'a str, KMs)> = None;
    
    for (name, city) in map.iter() {
        if name == city_name || visited.contains(&&name[..]) {
            continue;
        }
        
        if let Some(&dist) = city.distances.get(city_name) {
            
            match nearest {
                Some((_, nearest_dist)) if dist < nearest_dist => {
                    nearest = Some((&name[..], dist));
                },
                None => {
                    nearest = Some((&name[..], dist));
                },
                _ => {
                    
                }
            }
        }
    }
    
    nearest
}

fn find_shortest_route_starting_from<'a>(map: &'a Map, start: &'a str) -> (Vec<&'a str>, KMs) {
    
    let mut route = vec![start];
    let mut current = start;
    let mut length = 0;
    
    while let Some((next, dist)) = nearest_neighbour(map, current, &route) {
        
        current = next;
        length += dist;
        route.push(current);
    }
    
    (route, length)
}

fn main() {
    
    let distances = read_input_file(open_file());
    
    let map = build_map(distances);
    
    let mut routes: Vec<(Vec<&str>, KMs)> =
        map.keys()
           .map(|name| find_shortest_route_starting_from(&map, &name))
           .collect();
           
    routes.sort_by(|&(_, ref a), &(_, ref b)| a.cmp(b));
    
    for (route, length) in routes {
         
        println!("{} kms: {}", length, route.join(" -> "));
    }
}