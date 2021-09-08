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
    fn passed(&self) -> bool {
        let password = self.password.chars().collect::<Vec<char>>();
        let char_list: [Option<&char>; 2] = [
            password.get(self.min - 1),
            password.get(self.max - 1),
        ];
        return char_list[0] != char_list[1] && (char_list[0] == Some(&self.constraint) || char_list[1] == Some(&self.constraint))
    }
}

fn main() {
    let data = read_input("input.txt");

    let answer = find_answer(data);

    println!("{}", answer);
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

fn find_answer(vector: Vec<String>) -> usize {

    let mut counter: usize = 0;

    for (a, b, c) in izip!(&vector, &vector[1..], &vector[2..]).step_by(3) {
        let profile = PasswordProfile {
            min: a.split("-").collect::<Vec<&str>>()[0].parse().expect("Minimum constraint is not a number"),
            max: a.split("-").collect::<Vec<&str>>()[1].parse().expect("Maximum constraint is not a number"),
            constraint: b.chars().next().unwrap(),
            password: &c,
            };
        if profile.passed() == true { counter += 1 }
        };
    
    counter
}