use crate::common::get_input_lines;
use std::collections::HashSet;

struct Field(Vec<Vec<i32>>);

#[allow(dead_code)]
pub fn run() {
    let input = parse(&get_input_lines());

    let result = input.get_visible_trees();
    println!("Result (part 1): {result}");
}

fn parse(input: &Vec<String>) -> Field {
    let mut lines = vec![];

    for line in input {
        lines.push(
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect(),
        );
    }

    Field(lines)
}

impl Field {
    fn get_visible_trees(&self) -> usize {
        let mut visible = HashSet::new();
        let mut current;

        let mut trace = |x, y, current: &mut i32| {
            let value = self.value(x, y).unwrap();
            if value > *current {
                visible.insert((x, y));
                *current = value;
            }
        };

        for x in 0..self.width() {
            current = -1;
            for y in 0..self.height() {
                trace(x, y, &mut current);
            }

            current = -1;
            for y in (0..self.height()).rev() {
                trace(x, y, &mut current);
            }
        }

        for y in 0..self.height() {
            current = -1;
            for x in 0..self.width() {
                trace(x, y, &mut current);
            }

            current = -1;
            for x in (0..self.width()).rev() {
                trace(x, y, &mut current);
            }
        }

        visible.len()
    }

    fn width(&self) -> usize {
        self.0.get(0).unwrap().len()
    }

    fn height(&self) -> usize {
        self.0.len()
    }

    fn value(&self, x: usize, y: usize) -> Option<i32> {
        self.0.get(y).map(|line| line.get(x).copied()).flatten()
    }
}
