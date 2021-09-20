use hex;

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

    fn is_valid(&self) -> bool {
        self.byr_valid() && self.ecl_valid()
    }

    fn byr_valid(&self) -> bool {
        match &self.byr {
            Some(value) => match value.parse::<u32>() {
                Ok(number) => 2002 >= number && number >= 1920,
                Err(_) => false,
            },
            None => false,
        }
    }

    fn ecl_valid(&self) -> bool {
        match &self.ecl {
            Some(value) => {
                ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value.as_str())
            }
            None => false,
        }
    }

    fn eyr_valid(&self) -> bool {
        match &self.eyr {
            Some(value) => match value.parse::<u32>() {
                Ok(number) => 2020 >= number && number >= 2010,
                Err(_) => false,
            },
            None => false,
        }
    }

    fn hcl_valid(&self) -> bool {
        match &self.hcl {
            Some(value) => {
                value.get(..1) == Some("#")
                    && value.get(1..).unwrap().len() == 6
                    && match hex::decode(value.get(1..).unwrap()).is_ok() {
                        Ok(_) => true,
                        Err(_) => false,
                    }
            }
            None => false,
        }
    }

    fn hgt_valid(&self) -> bool {
        match &self.hgt {
            Some(value) => {
                ["in", "cm"].contains(&value.get().unwrap())
            }
            None => false,
        }
    }
}

mod problem {
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
                println!("{:?} is {}", passport.hgt, passport.hgt_valid());
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
