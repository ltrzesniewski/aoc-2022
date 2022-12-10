use crate::common::get_input_lines;
use std::collections::{HashSet, VecDeque};

#[allow(dead_code)]
pub fn run() {
    let input = get_input_lines().into_iter().next().unwrap();

    let result = part1(&input);
    println!("Result (part 1): {result}");
}

fn part1(input: &str) -> usize {
    const LENGTH: usize = 4;
    let mut buffer = VecDeque::with_capacity(LENGTH + 1);
    let mut set = HashSet::with_capacity(LENGTH);

    for (index, c) in input.chars().enumerate() {
        if buffer.len() < LENGTH {
            buffer.push_back(c);
            continue;
        }

        buffer.pop_front();
        buffer.push_back(c);

        set.clear();
        set.extend(buffer.iter().copied());

        if set.len() == LENGTH {
            return index + 1;
        }
    }

    panic!("Not found")
}
