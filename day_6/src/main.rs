use std::collections::HashMap;
use std::iter::FromIterator;

#[derive(Debug)]
struct Form {
    questions: HashMap<char, bool>,
}

impl Form {
    fn new() -> Form {
        let questions = HashMap::from_iter(('a'..='z').into_iter().map(|c| (c, false)));
        Form {
            questions: questions,
        }
    }

    fn count(&self) -> usize {
        self.questions.values().filter(|q| *q == &true).count()
    }

    fn check(&mut self, input: &String) {
        for i in input.chars() {
            self.questions.insert(i, true);
        }
    }
}

mod problem {
    use super::Form;
    use std::fs::File;
    use std::io::{BufRead, BufReader, Error, Lines};
    use std::path::Path;

    fn problem_1(input: &Vec<String>) {
        let mut form = Form::new();
        let mut counter = 0;
        for i in input {
            if i.is_empty() {
                counter += form.count();
                form = Form::new();
            }
            else {
                form.check(i);
            }
        }

        println!("Answer for part 1: {}", counter);
    }

    fn problem_2(input: &Vec<String>) {}

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
        problem_2(&data);
    }

    pub fn run() {
        read_input("input.txt").unwrap_or_else(|error| {
            panic!("{:?}", error);
        });
    }
}

fn main() {
    //println!("{:?}", ('a'..='z').collect::<Vec<char>>());
    println!("{:?}", Form::new());
    problem::run();
}
