use day_7::graph::Graph;
use std::fs;
use std::collections::{VecDeque, HashMap};
use std::collections::hash_map::Entry;
use std::borrow::Borrow;

/*
Create a graph were each node is a bag, and each edge represents being contained within another
directed graph: if A -> B, then A is contained within B
Counter-intuitive, but allows us to use BFS to find all the bags that we are contained by
 */

fn main() {
    let input = fs::read_to_string("input7.txt").expect("Failed to read input file");
    let mut graph = Graph::new();

    for line in input.lines() {
        let words = line.split_ascii_whitespace().collect::<Vec<&str>>();
        let name = words[0].to_owned() + " " + words[1];

        // check if this bag contains any other bags
        if words[4] == "no" {
            graph.insert_node(name);
        } else {
            let mut i = 4;
            while i < words.len() {
                // Add the contained bags to the graph
                let _weight = words[i];
                let contained_name = words[i+1].to_owned() + " " +  words[i+2];
                graph.insert_edge(contained_name, name.clone());

                i += 4;
            }
        }
    }

    // BFS
    let mut queue :VecDeque<&str>= VecDeque::new();
    let mut checked: HashMap<&str, bool> = HashMap::new();
    let start = "shiny gold";
    queue.push_front(start);
    checked.insert(start, false);

    let mut count = -1; // accounts for the first node
    while !queue.is_empty() {
        if let Some(current) = queue.pop_back() {
            // Check that we haven't already covered this node
            if let Entry::Occupied(mut ocp) = checked.entry(current) {
                if *ocp.get() == false {
                    *ocp.get_mut() = true;
                    // Add the others
                    if let Some(adj) = graph.get_adjacent(current) {
                        for node in adj {
                            queue.push_front(node);
                            // super hacky. Don't know why contains_key won't take a &str straight up
                            if !checked.contains_key(node.borrow() as &str) {
                                checked.insert(node, false);
                            }
                        }
                    }
                    count += 1;
                }
            }
        }
    }

    println!("There are {} colors contained in {}", count, start);

}
