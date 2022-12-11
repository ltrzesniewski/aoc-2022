use crate::common::get_input_lines;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
struct Position(i32, i32);

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Move(Direction, u32);

enum Delta {
    Same,
    Less,
    More,
}

struct RelativePosition(Delta, Delta);

struct Simulation {
    head: Position,
    tail: Position,
    visited: HashSet<Position>,
}

#[allow(dead_code)]
pub fn run() {
    let input = get_input_lines()
        .into_iter()
        .map(|i| i.parse().unwrap())
        .collect::<Vec<Move>>();

    let result = part1(&input);
    println!("Result (part 1): {result}");
}

fn part1(input: &[Move]) -> usize {
    let mut sim = Simulation::new();

    for m in input {
        for _ in 0..(m.1) {
            sim.move_head(m.0)
        }
    }

    sim.visited_pos_count()
}

impl Simulation {
    fn new() -> Simulation {
        let mut sim = Simulation {
            head: Position::default(),
            tail: Position::default(),
            visited: HashSet::new(),
        };

        sim.visited.insert(sim.tail);
        sim
    }

    fn move_head(&mut self, dir: Direction) {
        self.head = dir.move_by_one(self.head);
        self.move_tail();
    }

    fn move_tail(&mut self) {
        if self.tail.touches(self.head) {
            return;
        }

        self.tail = self.tail.move_towards(self.head);
        self.visited.insert(self.tail);
    }

    fn visited_pos_count(&self) -> usize {
        self.visited.len()
    }
}

impl Position {
    fn touches(&self, other: Position) -> bool {
        (self.0 - other.0).abs() <= 1 && (self.1 - other.1).abs() <= 1
    }

    fn get_relative_pos(&self, other: Position) -> RelativePosition {
        fn delta(d: i32) -> Delta {
            match d {
                0 => Delta::Same,
                1.. => Delta::More,
                _ => Delta::Less,
            }
        }

        RelativePosition(delta(other.0 - self.0), delta(other.1 - self.1))
    }

    fn move_towards(&self, other: Position) -> Position {
        self.move_by_one(self.get_relative_pos(other))
    }

    fn move_by_one(&self, direction: RelativePosition) -> Position {
        match direction {
            RelativePosition(Delta::Same, Delta::Same) => *self,
            RelativePosition(Delta::Less, Delta::Less) => Position(self.0 - 1, self.1 - 1),
            RelativePosition(Delta::More, Delta::Less) => Position(self.0 + 1, self.1 - 1),
            RelativePosition(Delta::Less, Delta::More) => Position(self.0 - 1, self.1 + 1),
            RelativePosition(Delta::More, Delta::More) => Position(self.0 + 1, self.1 + 1),
            RelativePosition(Delta::Same, Delta::Less) => Position(self.0, self.1 - 1),
            RelativePosition(Delta::Same, Delta::More) => Position(self.0, self.1 + 1),
            RelativePosition(Delta::Less, Delta::Same) => Position(self.0 - 1, self.1),
            RelativePosition(Delta::More, Delta::Same) => Position(self.0 + 1, self.1),
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Self(0, 0)
    }
}

impl Direction {
    fn move_by_one(&self, p: Position) -> Position {
        match self {
            Direction::Up => Position(p.0, p.1 - 1),
            Direction::Down => Position(p.0, p.1 + 1),
            Direction::Left => Position(p.0 - 1, p.1),
            Direction::Right => Position(p.0 + 1, p.1),
        }
    }
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s.split_whitespace().collect::<Vec<_>>();
        Ok(match tokens[..] {
            ["U", count] => Move(Direction::Up, count.parse().unwrap()),
            ["D", count] => Move(Direction::Down, count.parse().unwrap()),
            ["L", count] => Move(Direction::Left, count.parse().unwrap()),
            ["R", count] => Move(Direction::Right, count.parse().unwrap()),
            _ => panic!("Invalid move"),
        })
    }
}
