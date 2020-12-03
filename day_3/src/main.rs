use std::fs;

fn main() {
    let input = fs::read_to_string("input3.txt").expect("Failed to read from file");

    // (Right, Down)
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut runs : Vec<u32> = Vec::new();

    for slope in slopes {
        let mut trees = 0;
        let mut pos = 0;
        // every slope is repeating, so we can use modulo to save space
        for line in input.lines().step_by(slope.1) {
            let len = line.chars().count();
            let thing = match line.chars().nth(pos as usize) {
                Some(t) => t,
                None => '\0'
            };

            // if we've hit a tree
            // the very first position should never be a tree
            if thing == '#' {
                trees += 1;
            }

            pos = (pos + slope.0) % len;
        }
        println!("There are {} trees on slope Right: {}, Down: {}", trees, slope.0, slope.1);
        runs.push(trees);
    }

    let mut product: u32 = 1;
    for run in runs {
        product *= run;
    }
    println!("Their product is {}", product);
}
