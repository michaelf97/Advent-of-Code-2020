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

impl Default for Passport {
    fn default() -> Passport {
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
}

impl Passport {
    fn new() -> Passport {
        Passport {
            ..Default::default()
        }
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
            _ => panic!("Field not found"),
        }
    }

    fn valid(&self) -> bool {
        self.byr.is_some()
            && self.ecl.is_some()
            && self.eyr.is_some()
            && self.hcl.is_some()
            && self.hgt.is_some()
            && self.iyr.is_some()
            && self.pid.is_some()
    }
}

mod problem {
    use super::Passport;
    use std::fs::File;
    use std::io::{BufRead, BufReader, Error, Lines};
    use std::path::Path;

    fn problem_1(passports: Vec<Passport>) {
        let mut counter: usize = passports.len() + 1;
        for passport in passports {
            if !passport.valid() {
                counter -= 1
            };
        }

        println!("{}", counter);
    }

    fn problem_2() {}

    fn read_input(filename: impl AsRef<Path>) -> Result<File, Error> {
        let file = File::open(filename)?;
        let buffer = BufReader::new(&file);

        parse_input(buffer.lines());
        Ok(file)
    }

    fn parse_input<T: BufRead>(lines: Lines<T>) {
        let mut passport = Passport::new();
        let mut passports: Vec<Passport> = Vec::new();

        for line in lines {
            if line.as_ref().unwrap().is_empty() || passport.valid() {
                passports.push(passport);
                passport = Passport::new();
            }
            let key_values: Vec<&str> = line.as_ref().unwrap().split_whitespace().collect();

            for (key, value) in key_values.into_iter().map(|kv| {
                (
                    kv.split(":").collect::<Vec<&str>>()[0],
                    kv.split(":").collect::<Vec<&str>>()[1],
                )
            }) {
                passport.set(key, value);
            }
        }
        problem_1(passports);
    }

    pub fn run() {
        read_input("input.txt").unwrap_or_else(|error| {
            panic!("{:?}", error);
        });
    }
}

fn main() {
    problem::run();
}
