use std::fs;
use std::collections::HashMap;
use regex::Regex;

fn main() {
    let input = fs::read_to_string("test.txt").expect("Failed to read input file");
    // Can't seem to get this to split the paragraphs
    let reg = Regex::new(r"\w\n").unwrap();
    let passports = reg
        .split(&input)
        .map(|s| Passport::new(s))
        .collect::<Vec<Passport>>()
        ;

    let test = input.split("\n\n").collect::<Vec<&str>>();
    println!("Test len: {}", test.len());

    println!("There are {} passports", passports.len());

    let required_fields = vec!["byr", "iry", "eyr", "hgt", "hcl", "ecl", "pid"];
    let mut num_valid = 0;
    for passport in passports {
        println!("{}", passport.entry);
        println!("Key : Pair");
        for key in passport.map.keys() {
            println!("{} : {}", key, passport.map.get(key).unwrap());
        }

        // Check that it has all the valid fields
        let mut valid = true;
        for field in &required_fields {
            if !passport.map.contains_key(field) {
                valid = false;
            }
        }

        if valid {
            num_valid += 1;
        }
    }

    println!("There are {} valid passports", num_valid);

}

struct Passport<'a> {
    entry: &'a str,
    map: HashMap<&'a str, &'a str>
}

impl<'a> Passport<'a> {
    fn new(input: &str) -> Passport {
        let mut map = HashMap::new();
        let fields = input.split_ascii_whitespace();
        for field in fields {
            let temp = field.split(":").collect::<Vec<&str>>();
            let key_pair = match *temp {
                [first, second] => (first, second),
                _ => ("\0","\0")
            };
            map.insert(key_pair.0, key_pair.1);
        }

        return Passport {entry: input, map};
    }
}
