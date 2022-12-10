use crate::common::*;
use regex::Regex;
use std::fmt::{Display, Formatter};

struct Input {
    stacks: Stacks,
    moves: Vec<Move>,
}

#[derive(Clone)]
struct Stacks(Vec<CrateStack>);

#[derive(Clone)]
struct CrateStack(Vec<Crate>);

#[derive(Clone, Copy)]
struct Crate(char);

struct Move {
    count: usize,
    from: usize,
    to: usize,
}

#[allow(dead_code)]
pub fn run() {
    let input = get_input();

    let mut stacks = input.stacks.clone();
    for m in input.moves {
        stacks.apply_move(&m);
    }

    let result = stacks.get_top_crates();
    println!("Result (part 1): {result}");
}

fn get_input() -> Input {
    let regex = Regex::new(
        r"(?x)
        \[(?P<crate>[A-Z])]
        | move \s (?P<move_count>[0-9]+) \s from \s (?P<from>[0-9]+) \s to \s (?P<to>[0-9]+)",
    )
    .unwrap();

    let mut stacks = vec![];
    let mut moves = vec![];

    for line in get_input_lines() {
        for capture in regex.captures_iter(&line) {
            if let Some(m) = capture.name("crate") {
                let name = Crate(m.as_str().chars().next().unwrap());
                let index = (m.start() - 1) / 4;

                let mut stack = stacks.get_mut(index);
                if stack.is_none() {
                    stacks.resize_with(index + 1, || CrateStack(vec![]));
                    stack = stacks.get_mut(index);
                }

                let stack = stack.unwrap();
                stack.push(name)
            } else if let Some(m) = capture.name("move_count") {
                let count = m.as_str().parse::<usize>().unwrap();
                let from = capture
                    .name("from")
                    .unwrap()
                    .as_str()
                    .parse::<usize>()
                    .unwrap();
                let to = capture
                    .name("to")
                    .unwrap()
                    .as_str()
                    .parse::<usize>()
                    .unwrap();
                moves.push(Move { count, from, to })
            }
        }
    }

    for stack in stacks.iter_mut() {
        stack.0.reverse()
    }

    Input {
        stacks: Stacks(stacks),
        moves,
    }
}

impl Stacks {
    pub fn apply_move(&mut self, mv: &Move) {
        for _ in 0..mv.count {
            let item = self.0.get_mut(mv.from - 1).unwrap().pop();
            self.0.get_mut(mv.to - 1).unwrap().push(item);
        }
    }

    pub fn get_top_crates(&self) -> String {
        self.0
            .iter()
            .filter_map(|i| i.top())
            .map(|i| i.name())
            .collect::<String>()
    }
}

impl Display for Stacks {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_top_crates())
    }
}

impl CrateStack {
    pub fn pop(&mut self) -> Crate {
        self.0.pop().unwrap()
    }
    pub fn push(&mut self, item: Crate) {
        self.0.push(item);
    }
    pub fn top(&self) -> Option<Crate> {
        self.0.last().copied()
    }
}

impl Crate {
    pub fn name(&self) -> char {
        self.0
    }
}
