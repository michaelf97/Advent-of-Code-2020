use std::fs;
use itertools::izip;

#[derive(Debug)]
struct PasswordProfile<'a> {
    min: usize,
    max: usize,
    constraint: char,
    password: &'a str,
}

impl PasswordProfile<'_> {
    fn passed_old(&self) -> bool {
        let count = self.password.chars().filter(|c| *c == self.constraint).count();
        count >= self.min && count <= self.max
    }
    fn passed(&self) -> bool {
        let password = self.password.chars().collect::<Vec<char>>();
        let char_list: [Option<&char>; 2] = [
            password.get(self.min - 1),
            password.get(self.max - 1),
        ];
        return char_list[0] != char_list[1] && (char_list[0] == Some(&self.constraint) || char_list[1] == Some(&self.constraint))
    }
}

struct Problem {}

impl Problem {
    fn run(filename: &str) {
        let data = Problem::read_input(filename);

        Problem::part_1(&data);

        Problem::part_2(&data);
    }

    fn read_input(filename: &str) -> Vec<String> {

        let contents = fs::read_to_string(filename)
            .expect("Error reading file");
    
        let contents: Vec<String> = contents
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
    
        contents
    }

    fn part_1(vector: &Vec<String>) {

        let mut counter: usize = 0;
    
        for (a, b, c) in izip!(vector, &vector[1..], &vector[2..]).step_by(3) {
            let profile = PasswordProfile {
                min: a.split("-").collect::<Vec<&str>>()[0].parse().expect("Minimum constraint is not a number"),
                max: a.split("-").collect::<Vec<&str>>()[1].parse().expect("Maximum constraint is not a number"),
                constraint: b.chars().next().unwrap(),
                password: &c,
                };
            if profile.passed_old() == false { counter += 1 }
        };
        println!("Part 1 Answer: {}", counter)
    }

    fn part_2(vector: &Vec<String>) {

        let mut counter: usize = 0;

        for (a, b, c) in izip!(vector, &vector[1..], &vector[2..]).step_by(3) {
            let profile = PasswordProfile {
                min: a.split("-").collect::<Vec<&str>>()[0].parse().expect("Minimum constraint is not a number"),
                max: a.split("-").collect::<Vec<&str>>()[1].parse().expect("Maximum constraint is not a number"),
                constraint: b.chars().next().unwrap(),
                password: &c,
                };
            if profile.passed() == true { counter += 1 }
            };
        
        println!("Part 2 Answer: {}", counter);
    }
}

fn main() {
    Problem::run("input.txt")
}