use std::fs;
use std::collections::HashMap;
use regex::Regex;

fn main() {
    let input = fs::read_to_string("input4.txt").expect("Failed to read input file");

    // Can't seem to get this to split the paragraphs
    let passports = input
        .split("\n\n")
        //.split("\r\n\r\n")
        .map(|s| Passport::new(s))
        .collect::<Vec<Passport>>()
        ;

    println!("There are {} passports", passports.len());

    let required_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let mut num_valid = 0;
    for passport in passports {
        // Check that it has all the valid fields
        let mut valid = true;

        // byr validation
        let mut byr = true;
        match passport.map.get("byr") {
            Some(val) => {
                let num = val.parse::<i32>().unwrap();
                if !(val.len() == 4 && num >= 1920 && num <= 2002) {
                    byr = false;
                }
            }
            None => {
                byr = false;
            }
        }

        // iry validation
        let mut iyr = true;
        match passport.map.get("iyr") {
            Some(val) => {
                let num = val.parse::<i32>().unwrap();
                if !(val.len() == 4 && num >= 2010 && num <= 2020) {
                    iyr = false;
                }
            }
            None => {
                iyr = false;
            }
        }

        // eyr validation
        let mut eyr = true;
        match passport.map.get("eyr") {
            Some(val) => {
                let num = val.parse::<i32>().unwrap();
                if !(val.len() == 4 && num >= 2020 && num <= 2030) {
                    eyr = false;
                }
            }
            None => {
                eyr = false;
            }
        }

        // hgt validation
        let mut hgt = true;
        match passport.map.get("hgt") {
            Some(val) => {

                let unit = &val[(val.len()-2)..];
                // make sure that if there isn't a unit we don't mess up
                let num = match val[0..(val.len()-2)].parse::<i32>() {
                    Ok(n) => n,
                    Err(_) => -1
                };

                match unit {
                    "in" => {
                        if !(num >= 59 && num <= 76) {
                            hgt = false;
                        }
                    }
                    "cm" => {
                        if !(num >= 150 && num <= 193) {
                            hgt = false;
                        }
                    }
                    _ => {
                        hgt = false;
                    }
                }

            }
            None => {
                hgt = false;
            }
        }

        // hcl validation
        let mut hcl = true;
        match passport.map.get("hcl") {
            Some(val) => {
                let re = Regex::new(r"^#[a-z0-9]{6}$").expect("Failed to generate regex");
                if !re.is_match(val) {
                    hcl = false;
                }
            }
            None => {
                hcl = false;
            }
        }

        // ecl validation
        let mut ecl = true;
        match passport.map.get("ecl") {
            Some(val) => {
                if !(*val == "amb" || *val == "blu" || *val == "brn" || *val == "gry" ||
                    *val == "grn" || *val == "hzl" || *val == "oth") {
                    ecl = false;
                }
            }
            None => {
                ecl = false;
            }
        }

        // pid validation
        let mut pid = true;
        match passport.map.get("pid") {
            Some(val) => {
                let re = Regex::new(r"^\d{9}$").expect("Failed to generate regex");
                if !re.is_match(val) {
                    pid = false;
                }
            }
            None => {
                pid = false;
            }
        }

        /*
        // Debug
        println!("Validations");
        println!("-----------");
        println!("byr : {}", byr);
        println!("iyr : {}", iyr);
        println!("eyr : {}", eyr);
        println!("hgt : {}", hgt);
        println!("hcl : {}", hcl);
        println!("ecl : {}", ecl);
        println!("pid : {}", pid);
        */

        // If everything passed, then this passport was valid
        if byr && iyr && eyr && hgt && hcl && ecl && pid {
            num_valid += 1;
        }
    }

    println!("There are {} valid passports", num_valid);

}

struct Passport<'a> {
    _entry: &'a str,
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

        return Passport {_entry: input, map};
    }
}
