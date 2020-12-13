
// stores an adjaceny list of nodes
// This is a directed graph
pub mod graph {
    use std::collections::{HashMap};
    use std::collections::hash_map::Entry;

    #[derive(Debug)]
    pub struct Graph {
        adj_list: HashMap<String, Vec<Edge>>
    }

    pub struct Edge {
        node: String,
        weight: i32
    }

    impl PartialEq for Edge {
        fn eq(&self, other: &Self) -> bool {
            return self.node == other.node;
        }
    }
    impl Eq for Edge {}

    impl Graph {
        pub fn new() -> Graph {
            let adj_list: HashMap<String, Vec<Edge>> = HashMap::new();
            return Graph {adj_list};
        }

        // Inserts a node if not currently present. Does nothing with edges
        pub fn insert_node(&mut self, node: String) -> bool{
            let mut out = false;
            if !self.adj_list.contains_key(&node) {
                out = match self.adj_list.insert(node.to_owned(), Vec::new()) {
                    Some(_t) => false,
                    None => true
                };
            }

            return out;
        }

        // Places an edge between two nodes. If one or both nodes are not present in the graph,
        // they will be added
        // This is a directed graph, so the node is placed node1 -> node2
        // Returns false if the edge was already present or could not be added
        pub fn insert_edge(&mut self, node1: String, node2: String, weight: i32) -> bool {
            let mut out = false;
            // Add node2 to the graph if not already present
            if !self.adj_list.contains_key(&node2) {
                out = match self.adj_list.insert(node2, Vec::new()) {
                    Some(_t) => false,
                    None => true
                };
            }
            // Add node2 to node1's list
            // Will add node1 if not present
            let node = self.adj_list.entry(node1).or_insert(Vec::new());
            let edge = Edge {node: node2.to_owned(), weight};
            if !node.contains(&edge) {
                node.push(edge);
                out = true;
            }

            return out;
        }

        // Removes a node from the graph
        // Returns true if it was successfully removed
        pub fn remove_node(&mut self, node: &str) -> bool {
            let mut out = false;
            // Remove the entry in the table
            if let Entry::Occupied(val) = self.adj_list.entry(node.to_owned()) {
                val.remove_entry();
                out = true;
            }
            // Get rid of the node in every other nodes list
            for key in self.adj_list.iter_mut() {
                // Remove the value in the list
                if let Some(pos) = key.1.iter().position(|val| *val.node == node) {
                    key.1.remove(pos) ;
                    out = true;
                }
            }

            return out;
        }

        // Removes an edge from the graph
        pub fn remove_edge(&mut self, node1: &str, node2: &str) -> bool {
            let mut out = false;

            if let Some(val) = self.adj_list.get_mut(node1) {
                if let Some(pos) = val.iter().position(|val| *val.node == node2) {
                    val.remove(pos);
                    out = true;
                }
            }

            return out;
        }

        // Get the adjacent nodes
        pub fn get_adjacent(&self, node: &str) -> Option<&Vec<Edge>> {
            return self.adj_list.get(node);
        }
    }
}
