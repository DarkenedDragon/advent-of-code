use std::fs;
use std::collections::HashMap;

fn main() {
    let input = fs::read_to_string("input6.txt").expect("Failed to read input file");
    let groups = input
        .split("\n\n")
        //.split("\r\n\r\n") // use for test file
        .map(|val| Group::new(val))
        .collect::<Vec<Group>>();

    let mut total_q_ans = 0;
    let mut total_ev_ans = 0;
    for group in groups {
        total_q_ans += group.answers.keys().len();
        for ans in group.answers {
            if ans.1 == group.size {
                total_ev_ans +=1;
            }
        }
    }

    println!("{} questions were answered yes", total_q_ans);
    println!("{} questions were answered yes by everyone in a group", total_ev_ans);

}

// Struct to keep relevant data together
struct Group {
    answers: HashMap<char, i32>,
    size: i32
}

impl Group {
    fn new(input: &str) -> Group {
        let mut size = 1;
        let mut map: HashMap<char, i32> = HashMap::new();

        for letter in input.chars() {
            match letter {
                '\n' => size += 1,
                'a'..='z' => {
                    // Increments the number of times that letter has appeared
                    // or adds it if it hadn't before
                    let num = map.entry(letter).or_insert(0);
                    *num += 1;
                }
                _ => ()
            }
        }

        return Group {answers: map, size};
    }
}
