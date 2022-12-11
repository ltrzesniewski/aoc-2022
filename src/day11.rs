use crate::common::get_input_lines;
use regex::Regex;
use std::collections::VecDeque;
use std::str::FromStr;

#[derive(Copy, Clone)]
struct Item(i32);

enum Operand {
    Old,
    Number(i32),
}

enum Operator {
    Addition,
    Multiplication,
}

struct Operation(Operand, Operator, Operand);

struct Monkey {
    items: VecDeque<Item>,
    operation: Operation,
    test: i32,
    when_true: usize,
    when_false: usize,
    inspected: usize,
}

struct Puzzle {
    monkeys: Vec<Monkey>,
}

#[allow(dead_code)]
pub fn run() {
    let input = get_input_lines();

    let mut puzzle = Puzzle::parse(&input);
    let result = part1(&mut puzzle);
    println!("Result (part 1): {result}");
}

fn part1(puzzle: &mut Puzzle) -> usize {
    for _ in 0..20 {
        puzzle.play_round()
    }

    let mut scores = puzzle
        .monkeys
        .iter()
        .map(|i| i.inspected)
        .collect::<Vec<_>>();

    scores.sort_unstable_by(|a, b| b.cmp(a));
    scores.iter().take(2).fold(1, |s, i| s * i)
}

impl Puzzle {
    fn parse(input: &Vec<String>) -> Puzzle {
        let monkey_re = Regex::new(r"^Monkey (\d+):").unwrap();
        let items_re = Regex::new(r"^ {2}Starting items: ([\d, ]+)").unwrap();
        let operation_re =
            Regex::new(r"^ {2}Operation: new = (old|-?\d+) ([+*]) (old|-?\d+)").unwrap();
        let test_re = Regex::new(r"^ {2}Test: divisible by (\d+)").unwrap();
        let test_result_re = Regex::new(r"^ {4}If (true|false): throw to monkey (\d+)").unwrap();

        let mut monkeys = vec![];

        for line in input.iter() {
            if line.is_empty() {
                continue;
            }

            if let Some(cap) = monkey_re.captures(line) {
                let index = cap[1].parse::<usize>().unwrap();
                if index != monkeys.len() {
                    panic!()
                }

                monkeys.push(Monkey {
                    items: VecDeque::new(),
                    operation: Operation(
                        Operand::Number(0),
                        Operator::Addition,
                        Operand::Number(0),
                    ),
                    test: 0,
                    when_true: 0,
                    when_false: 0,
                    inspected: 0,
                });
            } else if let Some(cap) = items_re.captures(line) {
                let monkey = monkeys.last_mut().unwrap();
                let items = &cap[1];
                monkey
                    .items
                    .extend(items.split(", ").map(|i| Item(i.parse().unwrap())));
            } else if let Some(cap) = operation_re.captures(line) {
                let monkey = monkeys.last_mut().unwrap();
                monkey.operation = Operation(
                    cap[1].parse().unwrap(),
                    cap[2].parse().unwrap(),
                    cap[3].parse().unwrap(),
                );
            } else if let Some(cap) = test_re.captures(line) {
                let monkey = monkeys.last_mut().unwrap();
                monkey.test = cap[1].parse().unwrap();
            } else if let Some(cap) = test_result_re.captures(line) {
                let monkey = monkeys.last_mut().unwrap();
                if cap[1].parse().unwrap() {
                    monkey.when_true = cap[2].parse().unwrap();
                } else {
                    monkey.when_false = cap[2].parse().unwrap();
                }
            } else {
                panic!("Invalid input: {}", line);
            }
        }

        Puzzle { monkeys }
    }

    fn play_round(&mut self) {
        for i in 0..self.monkeys.len() {
            let result = self.monkeys[i].play();

            for (item, target) in result {
                self.monkeys[target].catch_item(item);
            }
        }
    }
}

impl Monkey {
    fn play(&mut self) -> Vec<(Item, usize)> {
        let mut result = vec![];

        loop {
            if let Some(item) = self.items.pop_front() {
                let new_item = self.operation.apply(item).relieve();
                let target_monkey = self.target_monkey(new_item);
                result.push((new_item, target_monkey));
                self.inspected += 1;
            } else {
                break;
            }
        }

        result
    }

    fn target_monkey(&self, item: Item) -> usize {
        if item.0 % self.test == 0 {
            self.when_true
        } else {
            self.when_false
        }
    }

    fn catch_item(&mut self, item: Item) {
        self.items.push_back(item);
    }
}

impl Item {
    fn relieve(&self) -> Item {
        Item(self.0 / 3)
    }
}

impl Operation {
    fn apply(&self, item: Item) -> Item {
        let a = self.0.value(item);
        let b = self.2.value(item);
        Item(self.1.apply(a, b))
    }
}

impl Operator {
    fn apply(&self, a: i32, b: i32) -> i32 {
        match self {
            Operator::Addition => a + b,
            Operator::Multiplication => a * b,
        }
    }
}

impl Operand {
    fn value(&self, item: Item) -> i32 {
        match self {
            Operand::Old => item.0,
            Operand::Number(n) => *n,
        }
    }
}

impl FromStr for Operand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "old" => Operand::Old,
            n => Operand::Number(n.parse().unwrap()),
        })
    }
}

impl FromStr for Operator {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "+" => Operator::Addition,
            "*" => Operator::Multiplication,
            _ => panic!(),
        })
    }
}
