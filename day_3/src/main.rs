struct Line {
    gradient: f32,
    y_intercept: f32,
    step_by: usize,
}

impl Line {
    fn x_coordinate(&self, y_coordinate: f32) -> f32 {
        (y_coordinate - self.y_intercept) / self.gradient
    }

    fn y_coordinate(&self, x_coordinate: f32) -> f32 {
        (self.gradient * x_coordinate) + self.y_intercept
    }

    fn calculate_gradient(x_coordinate: f32, y_coordinate: f32) -> f32 {
        y_coordinate / x_coordinate
    }
}

mod problem {
    use super::Line;
    use std::fs;

    fn read_input(filename: &str) -> Vec<Vec<char>> {
        let contents = fs::read_to_string(filename).expect("Error reading file");

        let contents: Vec<Vec<char>> = contents
            .split_whitespace()
            .map(|s| s.chars().collect())
            .collect();

        contents
    }

    fn problem_1(plot: &[Vec<char>]) {
        let line = Line {
            gradient: Line::calculate_gradient(3.0, 1.0),
            y_intercept: 0.0,
            step_by: 1,
        };
        count(&plot, '#', &line);
    }

    fn problem_2(plot: &[Vec<char>]) {
        let mut answer = 1;

        let lines = vec![
            Line {
                gradient: Line::calculate_gradient(1.0, 1.0),
                y_intercept: 0.0,
                step_by: 1,
            },
            Line {
                gradient: Line::calculate_gradient(3.0, 1.0),
                y_intercept: 0.0,
                step_by: 1,
            },
            Line {
                gradient: Line::calculate_gradient(5.0, 1.0),
                y_intercept: 0.0,
                step_by: 1,
            },
            Line {
                gradient: Line::calculate_gradient(7.0, 1.0),
                y_intercept: 0.0,
                step_by: 1,
            },
            Line {
                gradient: Line::calculate_gradient(1.0, 2.0),
                y_intercept: 0.0,
                step_by: 2,
            },
        ];

        for line in lines {
            answer *= count(&plot, '#', &line)
        }
        println!("Answer for problem 2: {}", answer);
    }

    fn count(plot: &[Vec<char>], character: char, line: &Line) -> usize {
        let mut counter: usize = 0;

        for y_coordinate in (0..plot.len()).step_by(line.step_by) {
            let x_coordinate =
                line.x_coordinate(y_coordinate as f32).ceil() as usize % plot[0].len();
            if plot[y_coordinate][x_coordinate] == character {
                counter += 1
            }
        }
        counter
    }

    pub fn run() {
        let data = read_input("input.txt");
        problem_1(&data);
        problem_2(&data);
    }
}

fn main() {
    problem::run();
}
