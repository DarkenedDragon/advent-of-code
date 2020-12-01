use std::fs;

fn main() {
    let filename = "input.txt";

    let contents = fs::read_to_string(filename).unwrap();
    // converts the input into a vector of 16 bit integers
    let values = contents.split_ascii_whitespace().map(|val| val.parse().unwrap()).collect::<Vec<i32>>();

    // brute force method
    println!("Find a pair that add to 2020");
    let mut res: (i32, i32) = (0,0);
    for val in values.iter() {
        for nxt_val in values.iter() {
            if val + nxt_val == 2020 {
                res = (val.clone(), nxt_val.clone());
                println!("Pair: {}, {}", val, nxt_val);
            }
        }
    }
    println!("Result: {} * {} = {}", res.0, res.1, res.0 * res.1);

    println!("Find a triple that add to 2020");
    // brute force method
    println!("Find a pair that add to 2020");
    let mut res: (i32, i32, i32) = (0,0,0);
    for val1 in values.iter() {
        for val2 in values.iter() {
            for val3 in values.iter() {
                if val1 + val2 + val3 == 2020 {
                    res = (val1.clone(), val2.clone(), val3.clone());
                    println!("Pair: {}, {}, {}", val1, val2, val3);
                }
            }
        }
    }
    println!("Result: {} * {} * {} = {}", res.0, res.1, res.2, res.0 * res.1 * res.2);

}
