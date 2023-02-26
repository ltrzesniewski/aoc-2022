use crate::common::get_input_lines;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::hash::Hash;
use std::iter;
use std::str::FromStr;

enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

struct Monkey {
    name: String,
    job: MonkeyJob,
    value: Option<isize>,
}

enum MonkeyJob {
    Value(isize),
    Formula(String, Op, String),
}

struct Puzzle {
    monkeys: HashMap<String, Monkey>,
}

struct DAG<'a, T>
where
    T: Node,
{
    roots: Vec<T::Key>,
    nodes: HashMap<T::Key, &'a T>,
}

trait Node {
    type Key: Hash + Eq + Clone;
    fn get_key(&self) -> &Self::Key;
    fn get_edges(&self) -> Vec<&Self::Key>;
}

#[allow(dead_code)]
pub fn run() {
    let puzzle = Puzzle::parse(get_input_lines());

    let result = part1(puzzle);
    println!("Result (part 1): {result}");
}

fn part1(mut puzzle: Puzzle) -> isize {
    let dag = DAG::new(puzzle.monkeys.values(), iter::once(String::from("root")));
    let sorted = dag
        .topo_sort()
        .iter()
        .rev()
        .map(|i| i.name.clone())
        .collect::<Vec<_>>();

    for name in sorted {
        puzzle.process(&name);
    }

    puzzle.get_node("root").value.unwrap()
}

impl Puzzle {
    fn parse(lines: Vec<String>) -> Puzzle {
        Self {
            monkeys: lines
                .iter()
                .map(|i| {
                    let monkey = i.parse::<Monkey>().unwrap();
                    (monkey.name.clone(), monkey)
                })
                .collect(),
        }
    }

    fn process(&mut self, name: &str) {
        let monkey = self.get_node(name);
        let value = Some(match &monkey.job {
            MonkeyJob::Value(n) => *n,
            MonkeyJob::Formula(a, op, b) => {
                let a = self.get_node(a).value.unwrap();
                let b = self.get_node(b).value.unwrap();

                match op {
                    Op::Add => a + b,
                    Op::Sub => a - b,
                    Op::Mul => a * b,
                    Op::Div => a / b,
                }
            }
        });

        self.get_node_mut(name).value = value;
    }

    fn get_node(&self, key: &str) -> &Monkey {
        self.monkeys.get(key).unwrap()
    }

    fn get_node_mut(&mut self, key: &str) -> &mut Monkey {
        self.monkeys.get_mut(key).unwrap()
    }
}

impl<'a, T> DAG<'a, T>
where
    T: Node,
{
    fn new(nodes: impl Iterator<Item = &'a T>, roots: impl Iterator<Item = T::Key>) -> Self {
        Self {
            nodes: HashMap::from_iter(nodes.map(|i| (i.get_key().clone(), i))),
            roots: roots.collect(),
        }
    }

    fn topo_sort(&self) -> Vec<&'a T> {
        let mut result = vec![];
        let mut stack = self.roots.iter().map(|i| (i, true)).collect::<Vec<_>>();

        enum Status {
            Visiting,
            Visited,
        }

        let mut statuses = HashMap::new();

        while let Some((key, enter)) = stack.pop() {
            let node = self.get_node(&key).unwrap();
            if enter {
                match statuses.get_mut(&key) {
                    Some(Status::Visited) => continue,
                    Some(Status::Visiting) => panic!("The graph is cyclic"),
                    None => {}
                };

                statuses.insert(key, Status::Visiting);
                stack.push((&key, false));

                for edge in node.get_edges() {
                    stack.push((&edge, true));
                }
            } else {
                statuses.insert(key, Status::Visited);
                result.push(node);
            }
        }

        result.reverse();
        result
    }

    fn get_node(&self, key: &T::Key) -> Option<&'a T> {
        self.nodes.get(key).copied()
    }
}

impl Node for Monkey {
    type Key = String;

    fn get_key(&self) -> &Self::Key {
        &self.name
    }

    fn get_edges(&self) -> Vec<&Self::Key> {
        match &self.job {
            MonkeyJob::Value(_) => vec![],
            MonkeyJob::Formula(a, _, b) => vec![&a, &b],
        }
    }
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"^\s*(?P<name>\w+)\s*:\s*(?:(?P<n>\d+)|(?P<a>\w+)\s*(?P<op>[-+*/])\s*(?P<b>\w+))\s*$"
            )
            .unwrap();
        }

        let cap = RE.captures(s).unwrap();

        Ok(Self {
            name: cap.name("name").unwrap().as_str().to_string(),
            value: None,
            job: if let Some(n) = cap.name("n") {
                MonkeyJob::Value(n.as_str().parse().unwrap())
            } else {
                MonkeyJob::Formula(
                    cap.name("a").unwrap().as_str().to_string(),
                    cap.name("op").unwrap().as_str().parse().unwrap(),
                    cap.name("b").unwrap().as_str().to_string(),
                )
            },
        })
    }
}

impl FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "+" => Op::Add,
            "-" => Op::Sub,
            "*" => Op::Mul,
            "/" => Op::Div,
            _ => panic!(),
        })
    }
}
