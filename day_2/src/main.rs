use std::fs;

fn main() {
    let input = fs::read_to_string("day_2/input2.txt").expect("Failed to read input file");

    // Split into each line
    let contents = input.split("\n").collect::<Vec<&str>>();

    let mut valid_1 = 0;
    let mut valid_2 = 0;
    // Check each line
    for line in contents {
        if line.len() > 0 {
            // part one
            // setup our variables
            let password_line = PasswordLine::new(line);
            let password = password_line.password;
            let range = password_line.range;
            // Count how many of that letter are in the password
            let mut num_of_letter = 0;
            for letter in password.chars() {
                if letter == password_line.character {
                    num_of_letter += 1;
                }
            }
            // if the number of letters in the password is within the range, add one
            if num_of_letter >= password_line.range.0 && num_of_letter <= password_line.range.1 {
                valid_1 += 1;
            }

            // check part 2
            // ranges are 1 indexed
            let first_letter = match password.chars().nth((range.0-1) as usize) {
                Some(a) => a,
                None => '\0'
            };
            let second_letter = match password.chars().nth((range.1-1) as usize) {
                Some(a) => a,
                None => '\0'
            };
            // XOR: if either the first letter or second letter are in the right place
            // but not both
            if (first_letter == password_line.character)
                ^ (second_letter == password_line.character)  {
                valid_2 += 1;
            }
        }

    }
    println!("Part one has {} valid passwords", valid_1);
    println!("Part two has {} valid passwords", valid_2);
}
// helper struct
// keeps things more orgainized
struct PasswordLine<'a> {
    range: (u32, u32),
    character: char,
    password: &'a str
}

impl<'a> PasswordLine<'a> {
    fn new(line: &str) -> PasswordLine {
        // should have three parts, the range, the letter, and the password
        let parts = line.split_ascii_whitespace().collect::<Vec<&str>>();
        let mut range = parts[0].split("-");
        let low = range.next().unwrap().parse::<u32>().expect("Couldn't parse lower bound");
        let high = range.next().unwrap().parse::<u32>().expect("Couldn't parse upper bound");

        let letter = parts[1].chars().next().expect("Couldn't parse letter"); // get the first char
        let password = parts[2];

        return PasswordLine {range: (low, high), character: letter, password};
    }
}
