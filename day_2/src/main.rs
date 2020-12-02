use std::fs;

fn main() {
    let input = fs::read_to_string("day_2/input2.txt").expect("Failed to read input file");

    // Split into each line
    let contents = input.split("\n").collect::<Vec<&str>>();


}

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
