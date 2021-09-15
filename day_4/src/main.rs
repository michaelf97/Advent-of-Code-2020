use std::collections::HashMap;

#[derive(Debug)]
struct Passport {
    byr: Option<String>,
    cid: Option<String>,
    ecl: Option<String>,
    eyr: Option<String>,
    hcl: Option<String>,
    hgt: Option<String>,
    iyr: Option<String>,
    pid: Option<String>,
}

impl Passport {
    fn new() -> Passport {
        Passport {
            byr: None,
            cid: None,
            ecl: None,
            eyr: None,
            hcl: None,
            hgt: None,
            iyr: None,
            pid: None,
        }
    }

    fn update(&mut self, data: HashMap<&str, &str>) {
        self.byr = match data.get("byr") {
            Some(value) => Some(value.to_string()),
            None => None,
        };
    }

    fn set(&mut self, key: &str, value: &str) {
        match key {
            "byr" => self.byr = Some(value.to_string()),
            "cid" => self.cid = Some(value.to_string()),
            "ecl" => self.ecl = Some(value.to_string()),
            "eyr" => self.eyr = Some(value.to_string()),
            "hcl" => self.hcl = Some(value.to_string()),
            "hgt" => self.hgt = Some(value.to_string()),
            "iyr" => self.iyr = Some(value.to_string()),
            "pid" => self.pid = Some(value.to_string()),
            _ => panic!["Field not found"],
        }
    }

    fn is_invalid(&self) -> bool {
        self.byr.is_none()
            || self.ecl.is_none()
            || self.eyr.is_none()
            || self.hcl.is_none()
            || self.hgt.is_none()
            || self.iyr.is_none()
            || self.pid.is_none()
    }
}

mod problem {
    use super::HashMap;
    use super::Passport;
    use std::fs;

    fn parse_input(filename: &str) {
        let input = fs::read_to_string(filename).expect("Error reading file");

        problem_1(input);
    }

    fn problem_1(data: String) {
        let mut passport = Passport::new();

        let mut counter = 0;

        for line in data.lines() {
            for l in line.split_whitespace() {
                passport.set(
                    l.split(":").collect::<Vec<&str>>()[0],
                    l.split(":").collect::<Vec<&str>>()[1],
                );
            }
            if line.is_empty() {
                if passport.is_invalid() == false {
                    counter += 1
                };
                passport = Passport::new();
            }
        }
        println!("Answer for part 1: {}", counter);
    }

    pub fn run() {
        parse_input("input.txt");
    }
}

fn main() {
    problem::run();
}
