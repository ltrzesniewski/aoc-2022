use crate::common::get_input_lines;
use regex::Regex;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
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

#[derive(Eq, Clone)]
struct Status2 {
    position_a: usize,
    position_b: usize,
    score: usize,
    valves: Rc<Vec<bool>>,
}

#[allow(dead_code)]
pub fn run() {
    let layout = Layout::parse(get_input_lines());

    let result = part1(&layout);
    println!("Result (part 1): {result}");

    let result = part2(&layout);
    println!("Result (part 2): {result}");
}

fn part1(layout: &Layout) -> usize {
    const TOTAL_MINUTES: usize = 30;

    let mut statuses = HashSet::new();
    statuses.insert(layout.initial_status());

    for minute in 0..TOTAL_MINUTES {
        let minutes_left = TOTAL_MINUTES - minute;

        let prev_statuses = statuses.iter().cloned().collect::<Vec<_>>();
        statuses.clear();

        for current_status in prev_statuses {
            statuses.extend(current_status.possible_moves(layout, minutes_left));
        }
    }

    statuses.iter().map(|i| i.score).max().unwrap()
}

fn part2(layout: &Layout) -> usize {
    const TOTAL_MINUTES: usize = 26;

    let mut statuses = HashSet::new();
    statuses.insert(layout.initial_status2());

    let mut max_score = 0;

    for minute in 0..TOTAL_MINUTES {
        let minutes_left = TOTAL_MINUTES - minute;

        let prev_statuses = statuses.iter().cloned().collect::<Vec<_>>();
        statuses.clear();

        let mut max_potential = max_score;

        for current_status in prev_statuses {
            let potential_score = current_status.potential_score(layout, minutes_left);
            max_potential = max_potential.max(potential_score);

            if potential_score <= max_score {
                continue;
            }

            for next_status in current_status.possible_moves(layout, minutes_left) {
                max_score = max_score.max(next_status.score);
                statuses.insert(next_status);
            }
        }

        println!(
            "Minute {}, score {}, count {}, potential {}",
            minute + 1,
            max_score,
            statuses.len(),
            max_potential
        );
    }

    max_score
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

    fn initial_status2(&self) -> Status2 {
        let status = self.initial_status();

        Status2 {
            position_a: status.position,
            position_b: status.position,
            score: 0,
            valves: status.valves,
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

impl Status2 {
    fn possible_moves(&self, layout: &Layout, minutes_left: usize) -> Vec<Status2> {
        let mut moves = vec![];

        let moves_a = self
            .to_single_status(true)
            .possible_moves(layout, minutes_left);

        let mut moves_b = self
            .to_single_status(false)
            .possible_moves(layout, minutes_left);

        // Can't open the same valve twice
        if self.position_a == self.position_b && !self.valves[self.position_a] {
            moves_b.remove(0);
        }

        for a in moves_a.iter() {
            for b in moves_b.iter() {
                moves.push(Status2 {
                    position_a: a.position,
                    position_b: b.position,
                    score: a.score + b.score - self.score,
                    valves: if a.score == self.score {
                        Rc::clone(&b.valves)
                    } else if b.score == self.score {
                        Rc::clone(&a.valves)
                    } else {
                        Rc::new(
                            a.valves
                                .iter()
                                .zip(b.valves.iter())
                                .map(|(&a, &b)| a || b)
                                .collect(),
                        )
                    },
                })
            }
        }

        moves
    }

    fn to_single_status(&self, first: bool) -> Status {
        Status {
            position: if first {
                self.position_a
            } else {
                self.position_b
            },
            score: self.score,
            valves: Rc::clone(&self.valves),
        }
    }

    fn potential_score(&self, layout: &Layout, minutes_left: usize) -> usize {
        let mut score = self.score;

        for i in 0..self.valves.len() {
            if !self.valves[i] {
                score += minutes_left * layout.valves[i].flow_rate;
            }
        }

        score
    }
}

impl PartialEq for Status2 {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
            && (self.position_a == other.position_a && self.position_b == other.position_b
                || self.position_a == other.position_b && self.position_b == other.position_a)
            && self.valves == other.valves
    }
}

impl Hash for Status2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.position_a + self.position_b).hash(state);
        self.score.hash(state);
        self.valves.hash(state);
    }
}

impl Display for Valve {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
