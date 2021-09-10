use std::fs;

struct Problem {}

impl Problem {
    fn run(filename: &str) {
        let data = Problem::read_input(filename);

        println!("Answer for part 1: {}", Problem::part_1(&data, 3, 1));
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

    fn part_1(vector: &Vec<String>, x_gradient: usize, y_gradient: usize) -> usize {
        let mut counter = 0;

        for (y_coordinate, x_axis) in vector.iter().enumerate().step_by(y_gradient) {
            let mut x_coordinate = y_coordinate * x_gradient;
            if x_coordinate != 0 { x_coordinate = x_coordinate % x_axis.len() }
            if x_axis.chars().collect::<Vec<char>>().get(x_coordinate) == Some(&'#') { 
                counter += 1;
            }
        }
        counter
    }
}

fn main() {
    Problem::run("input.txt")
}