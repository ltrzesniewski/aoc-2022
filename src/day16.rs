use crate::common::get_input_lines;
use regex::Regex;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::ops::Deref;
use std::rc::Rc;

struct Valve {
    name: String,
    flow_rate: usize,
    tunnels: Vec<usize>,
}

struct Layout {
    valves: Vec<Valve>,
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct Status {
    position: usize,
    score: usize,
    valves: Rc<Vec<bool>>,
}

#[allow(dead_code)]
pub fn run() {
    let layout = Layout::parse(get_input_lines());

    let result = part1(&layout);
    println!("Result (part 1): {result}");
}

fn part1(layout: &Layout) -> usize {
    const TOTAL_MINUTES: usize = 30;

    let mut statuses = HashSet::new();
    statuses.insert(layout.initial_status());

    for minute in 0..TOTAL_MINUTES {
        let minutes_left = TOTAL_MINUTES - minute;

        for current_status in statuses.iter().cloned().collect::<Vec<_>>() {
            for next_status in current_status.possible_moves(layout, minutes_left) {
                statuses.insert(next_status);
            }
        }
    }

    statuses.iter().map(|i| i.score).max().unwrap()
}

impl Layout {
    fn parse(input: Vec<String>) -> Layout {
        let re =
            Regex::new(r"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? ([\w, ]+)")
                .unwrap();

        let mut layout = Layout { valves: vec![] };

        for line in input {
            if line.is_empty() {
                continue;
            }

            let captures = re.captures(&line).unwrap();
            let valve = layout.get_or_create_valve(&captures[1]);
            layout.valves[valve].flow_rate = captures[2].parse().unwrap();

            for path in captures[3].split(", ") {
                let path = layout.get_or_create_valve(path);
                layout.valves[valve].tunnels.push(path);
            }
        }

        layout
    }

    fn get_or_create_valve(&mut self, name: &str) -> usize {
        if let Some(idx) = self.valves.iter().position(|i| i.name == name) {
            return idx;
        }

        self.valves.push(Valve {
            name: name.to_string(),
            flow_rate: 0,
            tunnels: vec![],
        });

        self.valves.len() - 1
    }

    fn initial_status(&self) -> Status {
        let mut valves = vec![false; self.valves.len()];

        for i in 0..self.valves.len() {
            if self.valves[i].flow_rate == 0 {
                valves[i] = true;
            }
        }

        Status {
            position: self.valves.iter().position(|i| i.name == "AA").unwrap(),
            score: 0,
            valves: Rc::new(valves),
        }
    }
}

impl Status {
    fn possible_moves(&self, layout: &Layout, minutes_left: usize) -> Vec<Status> {
        let mut moves = vec![];

        if !self.valves[self.position] {
            let mut valves = self.valves.deref().clone();
            valves[self.position] = true;

            moves.push(Status {
                position: self.position,
                score: self.score + (minutes_left - 1) * layout.valves[self.position].flow_rate,
                valves: Rc::new(valves),
            })
        }

        for other in layout.valves[self.position].tunnels.iter() {
            moves.push(Status {
                position: *other,
                score: self.score,
                valves: Rc::clone(&self.valves),
            })
        }

        moves
    }
}

impl Display for Valve {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
