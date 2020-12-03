use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let right = 3;
    let down = 1;

    let input = File::open("input3.txt").expect("Failed to read from file");
    let reader = BufReader::new(input);

    let mut trees = 0;
    let mut pos = 0;

    for line in reader.lines().step_by(down) {
        match line {
            Ok(t) => {
                let len = t.chars().count();
                let thing = match t.chars().nth(pos as usize) {
                    Some(t) => t,
                    None => '\0'
                };

                // if we've hit a tree
                // the very first position should never be a tree
                if thing == '#' {
                    trees += 1;
                }

                pos = (pos + right) % len;
            }
            Err(_) => (),
        };
    }

    println!("There are {} trees on this slope", trees);
}
