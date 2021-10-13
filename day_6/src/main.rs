use std::collections::HashMap;
use std::iter::FromIterator;

#[derive(Debug)]
struct Form {
    questions: HashMap<char, usize>,
    members: usize,
}

impl Form {
    fn new() -> Form {
        Form {
            questions: HashMap::from_iter(('a'..='z').into_iter().map(|c| (c, 0))),
            members: 0,
        }
    }

    fn count(&self) -> usize {
        self.questions.values().filter(|q| *q > &0).count()
    }

    fn check(&mut self, input: &String) {
        self.members += 1;
        ('a'..='z').for_each(|c| {
            if input.contains(c) {
                self.questions.insert(c, 1 + self.questions[&c]);
            }
        });
    }

    fn all_true(&mut self) -> usize {
        self.questions
            .values()
            .filter(|v| *v == &self.members)
            .count()
    }
}

mod problem {
    use super::Form;
    use std::fs::File;
    use std::io::{BufRead, BufReader, Error, Lines};
    use std::path::Path;

    fn problem_1(input: &Vec<String>) {
        let mut form = Form::new();
        let mut answer = 0;
        input.iter().for_each(|i| {
            form.check(i);
            if i.is_empty() {
                answer += form.count();
                form = Form::new();
            }
        });
        answer += form.count();

        println!("Answer for part 1: {}", answer);
    }

    fn problem_2(input: &Vec<String>) {
        let mut form = Form::new();
        let mut answer = 0;
        input.iter().for_each(|i| {
            if i.is_empty() {
                answer += form.all_true();
                form = Form::new();
            } else {
                form.check(i);
            }
        });
        answer += form.all_true();

        println!("Answer for part 2: {}", answer);
    }

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
    problem::run();
}
