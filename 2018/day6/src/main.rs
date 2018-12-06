extern crate util;

use util::{ read_input };

fn main() {

    type Id = usize;
    struct Input {
        id: Id,
        coords: (i32, i32),
    }

    let input =
        read_input("input.txt").unwrap()
            .iter().enumerate()
            .map(|(id, s)| {
                let mut s = s.split(", ").map(str::parse::<i32>);
                let x = s.next().unwrap().unwrap();
                let y = s.next().unwrap().unwrap();
                Input { id: id, coords: (x, y) }
            })
            .collect::<Vec<_>>();

    // Find bounds
    let min_x = input.iter().map(|input| input.coords.0).min().unwrap();
    let max_x = input.iter().map(|input| input.coords.0).max().unwrap();
    let min_y = input.iter().map(|input| input.coords.1).min().unwrap();
    let max_y = input.iter().map(|input| input.coords.1).max().unwrap();

    fn manhattan_distance((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> i32 {
        (x1 - x2).abs() + (y1 - y2).abs()
    }

    /*
    --- Part 1 ---
    The device then produces a list of coordinates (your puzzle input).
    Are they places it thinks are safe or dangerous? It recommends you check manual page 729.
    The Elves did not give you a manual.

    If they're dangerous, maybe you can minimize the danger by finding the coordinate that gives the largest distance from the other points.

    Using only the Manhattan distance, determine the area around each coordinate by counting the number
    of integer X,Y locations that are closest to that coordinate (and aren't tied in distance to any other coordinate).

    Your goal is to find the size of the largest area that isn't infinite. For example, consider the following list of coordinates:

    1, 1
    1, 6
    8, 3
    3, 4
    5, 5
    8, 9
    
    If we name these coordinates A through F, we can draw them on a grid, putting 0,0 at the top left:

    ..........
    .A........
    ..........
    ........C.
    ...D......
    .....E....
    .B........
    ..........
    ..........
    ........F.
    
    This view is partial - the actual grid extends infinitely in all directions. Using the Manhattan distance, each location's closest coordinate can be determined, shown here in lowercase:

    aaaaa.cccc
    aAaaa.cccc
    aaaddecccc
    aadddeccCc
    ..dDdeeccc
    bb.deEeecc
    bBb.eeee..
    bbb.eeefff
    bbb.eeffff
    bbb.ffffFf
    
    Locations shown as . are equally far from two or more coordinates, and so they don't count as being closest to any.

    What is the size of the largest area that isn't infinite?
    */

    let mut input_area_sizes = vec![0; input.len()];
    
    // For each element in the bounds, find the nearest input by smallest manhattan distance
    for x in min_x..max_x {
        for y in min_y..max_y {
            // For each input, find the manhattan distance to this point
            let distances = input.iter().map(|input| (input.id, manhattan_distance((x, y), input.coords)));
            // Scan each distance to find a *single* shortest one
            enum Ids { Single(Id), Multiple }
            let mut state = (Ids::Multiple, i32::max_value());
            for (id, dist) in distances {
                state = match state {
                    (_, _dist) if dist <  _dist => (Ids::Single(id), dist),
                    (_, _dist) if dist == _dist => (Ids::Multiple, dist),
                    otherwise => otherwise,
                };
            }
            // Did we find a single closest input? Bump the area size for that input:
            if let Ids::Single(id) = state.0 {
                input_area_sizes[id] += 1;
            }
        }
    }

    println!("Part 1 result: {}", input_area_sizes.iter().max().unwrap());

    /*
    --- Part 2 ---
    On the other hand, if the coordinates are safe, maybe the best you can
    do is try to find a region near as many coordinates as possible.

    For example, suppose you want the sum of the Manhattan distance to all of the
    coordinates to be less than 32. For each location, add up the distances to all of
    the given coordinates; if the total of those distances is less than 32, that location
    is within the desired region. Using the same coordinates as above, the resulting region looks like this:

    ..........
    .A........
    ..........
    ...###..C.
    ..#D###...
    ..###E#...
    .B.###....
    ..........
    ..........
    ........F.
    In particular, consider the highlighted location 4,3 located at the top middle of the region.
    Its calculation is as follows, where abs() is the absolute value function:

    Distance to coordinate A: abs(4-1) + abs(3-1) =  5
    Distance to coordinate B: abs(4-1) + abs(3-6) =  6
    Distance to coordinate C: abs(4-8) + abs(3-3) =  4
    Distance to coordinate D: abs(4-3) + abs(3-4) =  2
    Distance to coordinate E: abs(4-5) + abs(3-5) =  3
    Distance to coordinate F: abs(4-8) + abs(3-9) = 10
    Total distance: 5 + 6 + 4 + 2 + 3 + 10 = 30
    Because the total distance to all coordinates (30) is less than 32, the location is within the region.

    This region, which also includes coordinates D and E, has a total size of 16.

    Your actual region will need to be much larger than this
    example, though, instead including all locations with a total distance of less than 10000.

    What is the size of the region containing all locations
    which have a total distance to all given coordinates of less than 10000?
    */

    // For each element in the bounds, find the sum of the distances to each input
    // Count each sum under 10,000
    let mut count_in_region = 0;
    for x in min_x..max_x {
        for y in min_y..max_y {
            let distances = input.iter().map(|input| manhattan_distance((x, y), input.coords));
            if distances.sum::<i32>() < 10_000 {
                count_in_region += 1;
            }
        }
    }

    println!("Part 2 result: {}", count_in_region);
}
