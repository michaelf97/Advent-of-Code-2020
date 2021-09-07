use std::env;
use std::fs;

fn main() {
    let data = read_input("input.txt");

    let answer = find_answer(data);

    println!("The answer is {:?}", answer);
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

fn find_answer(vector: Vec<u32>) -> (u32,u32) {
    for i in &vector {
        for j in &vector {
            if i + j == 2020 {
                return (*i, *j);
            }
        }
    }
    panic!("No matching numbers found")
}
