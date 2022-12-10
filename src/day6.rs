use crate::common::get_input_lines;
use std::collections::{HashSet, VecDeque};

#[allow(dead_code)]
pub fn run() {
    let input = get_input_lines().into_iter().next().unwrap();

    let result = find_start(&input, 4);
    println!("Result (part 1): {result}");

    let result = find_start(&input, 14);
    println!("Result (part 2): {result}");
}

fn find_start(input: &str, length: usize) -> usize {
    let mut buffer = VecDeque::with_capacity(length + 1);
    let mut set = HashSet::with_capacity(length);

    for (index, c) in input.chars().enumerate() {
        if buffer.len() < length {
            buffer.push_back(c);
            continue;
        }

        buffer.pop_front();
        buffer.push_back(c);

        set.clear();
        set.extend(buffer.iter().copied());

        if set.len() == length {
            return index + 1;
        }
    }

    panic!("Not found")
}
