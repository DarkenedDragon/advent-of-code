use day_7::graph::{Graph, breadth_first_search};
use std::fs;

/*
Create a graph were each node is a bag, and each edge represents being contained within another
directed graph: if A -> B, then A is contained within B
Counter-intuitive, but allows us to use BFS to find all the bags that we are contained by
 */

fn main() {
    let input = fs::read_to_string("input7.txt").expect("Failed to read input file");
    let mut graph_prt_1 = Graph::new();
    let mut graph_prt_2 = Graph::new();

    for line in input.lines() {
        let words = line.split_ascii_whitespace().collect::<Vec<&str>>();
        let name = words[0].to_owned() + " " + words[1];

        // check if this bag contains any other bags
        if words[4] == "no" {
            graph_prt_1.insert_node(name);
        } else {
            let mut i = 4;
            while i < words.len() {
                // Add the contained bags to the graph
                let weight = match words[i].parse::<i32>() {
                    Ok(val) => val,
                    Err(_e) => 0
                };
                let contained_name = words[i+1].to_owned() + " " +  words[i+2];
                graph_prt_1.insert_edge(contained_name.clone(), name.clone(), weight);
                graph_prt_2.insert_edge(name.clone(), contained_name.clone(), weight);

                i += 4;
            }
        }
    }


    let mut count = -1;
    let mut total = 0;
    let start = "shiny gold";
    // Experiments with closures
    breadth_first_search(graph_prt_1, start, |current| count+=1);
    breadth_first_search(graph_prt_2, start, |current| )

    println!("There are {} colors contained in {}", count, start);

}
