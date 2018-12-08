extern crate util;
extern crate itertools;
extern crate regex;

use util::{ read_input };

fn main() {

    /*
    --- Part One ---
    The navigation system's license file consists of a list of numbers (your puzzle input).
    The numbers define a data structure which, when processed, produces some kind of tree
    that can be used to calculate the license number.

    The tree is made up of nodes; a single, outermost node forms the tree's root, and it
    contains all other nodes in the tree (or contains nodes that contain nodes, and so on).

    Specifically, a node consists of:

    A header, which is always exactly two numbers:
    The quantity of child nodes.
    The quantity of metadata entries.
    Zero or more child nodes (as specified in the header).
    One or more metadata entries (as specified in the header).
    Each child node is itself a node that has its own header, child nodes, and metadata. For example:

    2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2
    A----------------------------------
        B----------- C-----------
                        D-----
    
    In this example, each node of the tree is also marked with an underline starting with a letter for easier identification. In it, there are four nodes:

    A, which has 2 child nodes (B, C) and 3 metadata entries (1, 1, 2).
    B, which has 0 child nodes and 3 metadata entries (10, 11, 12).
    C, which has 1 child node (D) and 1 metadata entry (2).
    D, which has 0 child nodes and 1 metadata entry (99).
    The first check done on the license file is to simply add up all of the metadata entries. In this example, that sum is 1+1+2+10+11+12+2+99=138.

    What is the sum of all metadata entries?
    */

    let input =
        read_input("input.txt").unwrap()[0]
            .split(" ")
            .map(str::parse::<u32>)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

    struct Record {
        children: Vec<Record>,
        metadata: Vec<u32>,
    }

    fn parse_record<I>(iter: &mut I) -> Record
        where I: Iterator<Item=u32>
    {
        let children_count = iter.next().unwrap();
        let metadata_count = iter.next().unwrap();
        let mut children = Vec::new();
        for _ in 0..children_count as usize {
            children.push(parse_record(iter));
        }
        let mut metadata = Vec::new();
        for i in iter.take(metadata_count as usize) {
            metadata.push(i);
        }
        Record { children: children, metadata: metadata }
    }

    let root_record = parse_record(&mut input.into_iter());

    fn sum_metadata(record: &Record) -> u32 {
        record.children.iter().map(sum_metadata).sum::<u32>() + record.metadata.iter().sum::<u32>()
    }

    let result = sum_metadata(&root_record);

    println!("Part 1 result: {}", result);

    /*
    --- Part Two ---
    The second check is slightly more complicated: you need to find the value of the root node (A in the example above).

    The value of a node depends on whether it has child nodes.

    If a node has no child nodes, its value is the sum of its metadata entries.
    So, the value of node B is 10+11+12=33, and the value of node D is 99.

    However, if a node does have child nodes, the metadata entries become indexes which refer to those child nodes.
    A metadata entry of 1 refers to the first child node, 2 to the second, 3 to the third, and so on. The value of
    this node is the sum of the values of the child nodes referenced by the metadata entries. If a referenced child
    node does not exist, that reference is skipped. A child node can be referenced multiple time and counts each
    time it is referenced. A metadata entry of 0 does not refer to any child node.

    For example, again using the above nodes:

    Node C has one metadata entry, 2. Because node C has only one child node, 2 references a child node which does
    not exist, and so the value of node C is 0. Node A has three metadata entries: 1, 1, and 2. The 1 references
    node A's first child node, B, and the 2 references node A's second child node, C. Because node B has a value
    of 33 and node C has a value of 0, the value of node A is 33+33+0=66.
    So, in this example, the value of the root node is 66.

    What is the value of the root node?
    */

    fn get_record_value(record: &Record) -> u32 {
        if record.children.len() == 0 {
            return record.metadata.iter().sum::<u32>();
        }
        record.metadata.iter()
            .map(|&m| record.children.get(m as usize - 1).map(get_record_value).unwrap_or(0))
            .sum::<u32>()
    }

    let result = get_record_value(&root_record);

    println!("Part 2 result: {}", result);
}
