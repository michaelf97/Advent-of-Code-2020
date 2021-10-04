struct TwoDSpace {
    x_dimension: Vec<usize>,
    y_dimension: Vec<usize>,
}

enum Command {
    Forward,
    Back,
    Left,
    Right,
}

mod problem {
    use super::TwoDSpace;
    use super::Command;
    use std::fs::File;
    use std::io::{BufRead, BufReader, Error, Lines};
    use std::path::Path;

    fn problem_1(data: &Vec<String>) {
        for tree in data {
            let mut space = TwoDSpace{
                x_dimension: (0..128).collect(),
                y_dimension: (0..8).collect(),
            };

            let mut command: Command;

            tree.chars().for_each(|t| {
                match t {
                    'F' => command = Command::Forward,
                    'B' => command = Command::Back,
                    'L' => command = Command::Left,
                    'R' => command = Command::Right,
                    _ => println!("Unknown command"),
                }
            })
        }
    }

    fn problem_2() {}

    fn read_input(filename: impl AsRef<Path>) -> Result<File, Error> {
        let file = File::open(filename)?;
        let buffer = BufReader::new(&file);

        parse_input(buffer.lines());
        Ok(file)
    }

    fn parse_input<T: BufRead>(lines: Lines<T>) {
        let mut data = vec![];
        for line in lines {
            data.push(line.unwrap());
        }

        problem_1(&data);
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
