use std::fs;
use itertools::Itertools;

struct Problem {}

impl Problem {
    fn run(filename: &str) {
        let data = Problem::read_input(filename);

        Problem::part_1(&data);

        Problem::part_2(&data)
    }

    fn read_input(filename: &str) -> Vec<u32> {
        let contents = fs::read_to_string(filename)
        .expect("Error reading file");

        let contents: Vec<u32> = contents
            .split_whitespace()
            .map(|s| s.to_string())
            .map(|s| s.parse().expect("Please input a number"))
            .collect();

        contents
    }

    fn part_1(vector: &Vec<u32>) {
        for (a, b) in vector.iter().tuple_combinations() {
            if a + b == 2020 { println!("Part 1 Answer: {}", *a * *b) }
        }
    }

    fn part_2(vector: &Vec<u32>) {
        for (a, b, c) in vector.iter().tuple_combinations() {
            if a + b + c == 2020 { println!("Part 1 Answer: {}", *a * *b * *c) }
        }
    }
}

fn main() {
    Problem::run("input.txt")
}