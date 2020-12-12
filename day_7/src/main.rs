use day_7::graph::Graph;

fn main() {
    println!("Hello, world!");
    let mut graph = Graph::new();
    graph.insert_edge("hi", "there");
    graph.insert_edge("hi", "general kenobi");

    println!("{:?}", graph);
    graph.remove_node("general kenobi");
    graph.remove_edge("hi", "there");
    println!("{:?}", graph);
}
