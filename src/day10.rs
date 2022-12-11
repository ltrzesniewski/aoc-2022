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

    println!("Result (part 2):");
    println!("{}", part2(&input));
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

fn part2(input: &[Instruction]) -> String {
    let mut x = 1;
    let mut cycle_counter = 0;
    let mut input_iter = input.iter();

    let mut current_instruction = *input_iter.next().unwrap();
    let mut current_cycles_left = current_instruction.cycle_count();

    let mut output = String::new();

    loop {
        cycle_counter += 1;
        current_cycles_left -= 1;

        let col = cycle_counter % 40;
        let sprite = x..=(x + 2);

        output += if sprite.contains(&col) { "#" } else { "." };

        if col == 0 {
            output += "\n";
        }

        if current_cycles_left > 0 {
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

        current_cycles_left = current_instruction.cycle_count()
    }

    output
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
