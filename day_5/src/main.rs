use std::fs;
use std::collections::HashMap;

fn main() {
    let input = fs::read_to_string("input5.txt").expect("Failed to read input file");

    let mut high_id = 0;
    let mut ids: HashMap<i32, i32> = HashMap::new();
    for entry in input.lines() {
        // perform 2 binary searches
        // 1 on the first 7 characters to find the row
        // 1 on the last 3 to find the column
        let row_data = &entry[..7];
        let mut range = (0, 127);
        for place in row_data.chars() {
            let split = (range.0 + range.1)/2;
            match place {
                'F' => range.1 = split, // take the lower half.
                'B' => range.0 = split+1, // take the upper half. The +1 accounts for 0-index
                _ => ()
            }
        }
        let row = range.0; // both values in range should be the same

        let col_data = &entry[7..];
        let mut range = (0, 7);
        for place in col_data.chars() {
            let split = (range.0 + range.1)/2;
            match place {
                'L' => range.1 = split, // take the lower half.
                'R' => range.0 = split+1, // take the upper half. The +1 accounts for 0-index
                _ => ()
            }
        }

        let col = range.0;
        let seat_id = row * 8 + col;
        ids.insert(seat_id, seat_id);
        println!("{}: row {}, column {}, seat ID {}", entry, row, col, seat_id);

        if seat_id > high_id {
            high_id = seat_id;
        }
    }

    println!("The highest seat id is {}", high_id);

    // part 2 : find your seat!
    // find a value that isn't in the list, but its neighbors are
    for i in 0..high_id {
        if !ids.contains_key(&i) && ids.contains_key(&(i+1)) && ids.contains_key(&(i-1)) {
            println!("Seat found: {}", i);
        }
    }
}
