use crate::common::get_input_lines;
use std::collections::HashSet;

struct Field(Vec<Vec<i32>>);

#[allow(dead_code)]
pub fn run() {
    let input = parse(&get_input_lines());

    let result = input.get_visible_trees();
    println!("Result (part 1): {result}");

    let result = input.get_best_view_score();
    println!("Result (part 2): {result}");
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

    fn get_best_view_score(&self) -> i32 {
        let mut best = 0;

        for x in 1..(self.width() - 1) {
            for y in 1..(self.height() - 1) {
                let mut score = 1;
                let max = self.value(x, y).unwrap();

                for delta in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let (mut dx, mut dy) = (x, y);
                    let mut distance = 0;

                    loop {
                        dx = (dx as isize + delta.0) as usize;
                        dy = (dy as isize + delta.1) as usize;

                        match self.value(dx, dy) {
                            None => break,
                            Some(value) => {
                                distance += 1;
                                if value >= max {
                                    break;
                                }
                            }
                        }
                    }

                    score *= distance;
                }

                best = best.max(score);
            }
        }

        best
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
