use crate::common::get_input_lines;
use std::str::FromStr;

#[derive(Copy, Clone)]
enum Instruction {
    Noop,
    AddX(i32),
}

#[allow(dead_code)]
pub fn run() {
    let input = get_input_lines()
        .into_iter()
        .map(|i| i.parse().unwrap())
        .collect::<Vec<Instruction>>();

    let result = part1(&input);
    println!("Result (part 1): {result}");
}

fn part1(input: &[Instruction]) -> i32 {
    let mut x = 1;
    let mut cycle_counter = -1;
    let mut input_iter = input.iter();

    let mut current_instruction = Instruction::Noop;
    let mut current_cycles_left = 0;

    let mut result = 0;

    loop {
        cycle_counter += 1;

        if matches!(cycle_counter, 20 | 60 | 100 | 140 | 180 | 220) {
            result += cycle_counter * x;
        }

        if current_cycles_left > 0 {
            current_cycles_left -= 1;
            continue;
        }

        match current_instruction {
            Instruction::Noop => {}
            Instruction::AddX(value) => x += value,
        }

        match input_iter.next() {
            None => break,
            Some(instruction) => current_instruction = *instruction,
        }

        current_cycles_left = current_instruction.cycle_count() - 1
    }

    result
}

impl Instruction {
    fn cycle_count(&self) -> usize {
        match self {
            Instruction::Noop => 1,
            Instruction::AddX(_) => 2,
        }
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s.split_whitespace().collect::<Vec<_>>();
        Ok(match tokens[..] {
            ["noop"] => Instruction::Noop,
            ["addx", count] => Instruction::AddX(count.parse().unwrap()),
            _ => panic!("Invalid instruction"),
        })
    }
}
