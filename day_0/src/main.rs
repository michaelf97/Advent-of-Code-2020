mod problem {
    use std::fs::File;
    use std::io::{BufRead, BufReader, Error, Lines};
    use std::path::Path;

    fn problem_1() {

    }

    fn problem_2() {

    }

    fn read_input(filename: impl AsRef<Path>) -> Result<File, Error> {
        let file = File::open(filename)?;
        let buffer = BufReader::new(&file);

        parse_input(buffer.lines());
        Ok(file)
    }

    fn parse_input<T: BufRead>(lines: Lines<T>) {

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
