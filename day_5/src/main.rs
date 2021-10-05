use std::convert::TryFrom;

#[derive(Debug)]
struct TwoDSpace {
    x_dimension: Vec<usize>,
    y_dimension: Vec<usize>,
}

impl TwoDSpace {
    fn new() -> TwoDSpace {
        TwoDSpace {
            x_dimension: (0..128).collect(),
            y_dimension: (0..8).collect(),
        }
    }

    fn id(&self) -> usize {
        if self.x_dimension.len() == 1 && self.y_dimension.len() == 1 {
            (self.x_dimension.get(0).unwrap() * 8) + self.y_dimension.get(0).unwrap()
        } else {
            panic!("ID not calculable")
        }
    }

    fn operation(&mut self, command: Command) {
        match command {
            Command::Forward | Command::Back => self.x_operation(&command),
            Command::Left | Command::Right => self.y_operation(&command),
        }
    }

    fn y_operation(&mut self, command: &Command) {
        let chunks: Vec<Vec<usize>> = self
            .y_dimension
            .chunks(self.y_dimension.len() / 2)
            .map(|u| u.to_owned())
            .collect();

        if chunks.len() > 2 {
            panic!("Unexpected dimension size {:?}", chunks)
        }

        self.y_dimension = match command {
            Command::Left => chunks.get(0).unwrap().to_owned(),
            Command::Right => chunks.get(1).unwrap().to_owned(),
            _ => panic!("Unxpected Command enumeration, {:?}", command),
        }
    }

    fn x_operation(&mut self, command: &Command) {
        let chunks: Vec<Vec<usize>> = self
            .x_dimension
            .chunks(self.x_dimension.len() / 2)
            .map(|u| u.to_owned())
            .collect();

        if chunks.len() > 2 {
            panic!("Unexpected dimension size {:?}", chunks)
        }

        self.x_dimension = match command {
            Command::Forward => chunks.get(0).unwrap().to_owned(),
            Command::Back => chunks.get(1).unwrap().to_owned(),
            _ => panic!("Unxpected Command enumeration, {:?}", command),
        }
    }
}

#[derive(Debug)]
enum Command {
    Forward,
    Back,
    Left,
    Right,
}

impl TryFrom<char> for Command {
    type Error = &'static str;

    fn try_from(character: char) -> Result<Self, Self::Error> {
        match character {
            'F' => Ok(Self::Forward),
            'B' => Ok(Self::Back),
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => Err("Invalid command"),
        }
    }
}

mod problem {
    use super::Command;
    use super::TwoDSpace;
    use std::convert::TryFrom;
    use std::fs::File;
    use std::io::{BufRead, BufReader, Error, Lines};
    use std::path::Path;

    fn problem_1(data: &Vec<String>) {
        let mut answer: usize = 0;

        for tree in data {
            let mut space = TwoDSpace::new();

            tree.chars().for_each(|t| {
                space.operation(Command::try_from(t).unwrap());
            });

            if space.id() > answer {
                answer = space.id();
            }
        }

        println!("Answer for problem 1: {}", answer);
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

    let test = String::from("Hello");
}
