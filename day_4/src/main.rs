use hex;

#[derive(Debug, Default)]
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
        vec![
            self.byr.is_some(),
            self.ecl.is_some(),
            self.eyr.is_some(),
            self.hcl.is_some(),
            self.hgt.is_some(),
            self.iyr.is_some(),
            self.pid.is_some(),
        ].iter().all(|x| x == &true)
    }

    fn valid_problem_2(&self) -> bool {
        vec![
            self.byr_valid(),
            self.ecl_valid(),
            self.eyr_valid(),
            self.hcl_valid(),
            self.hgt_valid(),
            self.iyr_valid(),
            self.pid_valid(),
        ].iter().all(|x| x == &true)
    }

    fn byr_valid(&self) -> bool {
        self.byr
            .as_ref()
            .and_then(|v| v.parse::<usize>().ok())
            .map_or(false, |v| (1920..2003).contains(&v))
    }

    fn ecl_valid(&self) -> bool {
        self.ecl.as_ref().map_or(false, |v| {
            ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&v.as_str())
        })
    }

    fn eyr_valid(&self) -> bool {
        self.eyr
            .as_ref()
            .and_then(|v| v.parse::<usize>().ok())
            .map_or(false, |v| (2020..2031).contains(&v))
    }

    fn hcl_valid(&self) -> bool {
        self.hcl.as_ref().map_or(false, |v| {
            let mut v = v.clone();
            v.remove(0) == '#' && hex::decode(v).is_ok()
        })
    }

    fn hgt_valid(&self) -> bool {
        self.hgt.as_ref().map_or(false, |v| {
            let (prefix, suffix) = v.split_at(v.len() - 2);
            match suffix {
                "in" => prefix
                    .parse::<usize>()
                    .map_or(false, |p| (59..77).contains(&p)),
                "cm" => prefix
                    .parse::<usize>()
                    .map_or(false, |p| (150..194).contains(&p)),
                _ => false,
            }
        })
    }

    fn pid_valid(&self) -> bool {
        self.pid
            .as_ref()
            .map_or(false, |v| v.parse::<usize>().is_ok() && v.len() == 9)
    }

    fn iyr_valid(&self) -> bool {
        self.iyr
            .as_ref()
            .and_then(|v| v.parse::<usize>().ok())
            .map_or(false, |v| (2010..2021).contains(&v))
    }
}

mod problem {
    use super::Passport;
    use std::fs::File;
    use std::io::{BufRead, BufReader, Error, Lines};
    use std::path::Path;

    fn problem_1(passports: &Vec<Passport>) {
        let mut counter: usize = passports.len() + 1;
        for passport in passports {
            if !passport.valid() {
                counter -= 1
            };
        }

        println!("Answer to problem 1: {}", counter);
    }

    fn problem_2(passports: &Vec<Passport>) {
        let mut counter: usize = 0;
        for passport in passports {
            if passport.valid_problem_2() {
                counter += 1
            }
        }

        println!("Answer to problem 2: {}", counter);
    }

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
        problem_1(&passports);
        problem_2(&passports);
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
