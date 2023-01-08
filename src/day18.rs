use crate::common::get_input_lines;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Coord {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Default, Clone)]
struct Cube<T> {
    top: T,
    bottom: T,
    left: T,
    right: T,
    front: T,
    back: T,
}

#[derive(Copy, Clone)]
enum Side {
    Top,
    Bottom,
    Left,
    Right,
    Front,
    Back,
}

#[derive(Clone)]
struct Puzzle {
    cubes: HashMap<Coord, Cube<bool>>,
}

#[allow(dead_code)]
pub fn run() {
    let puzzle = Puzzle::parse(get_input_lines());

    let result = part1(puzzle.clone());
    println!("Result (part 1): {result}");

    let result = part2(puzzle);
    println!("Result (part 2): {result}");
}

fn part1(mut puzzle: Puzzle) -> usize {
    for coord in puzzle.cubes.keys().copied().collect::<Vec<_>>() {
        for adjacent in coord.adjacent() {
            if let Some(cube) = puzzle.cubes.get_mut(&adjacent.1) {
                cube.set(&adjacent.0.opposite(), true)
            }
        }
    }

    puzzle.cubes.values().map(|i| i.count(|&j| !j)).sum()
}

fn part2(mut puzzle: Puzzle) -> usize {
    let min = Coord {
        x: puzzle.cubes.keys().map(|i| i.x - 1).min().unwrap(),
        y: puzzle.cubes.keys().map(|i| i.y - 1).min().unwrap(),
        z: puzzle.cubes.keys().map(|i| i.z - 1).min().unwrap(),
    };

    let max = Coord {
        x: puzzle.cubes.keys().map(|i| i.x + 1).max().unwrap(),
        y: puzzle.cubes.keys().map(|i| i.y + 1).max().unwrap(),
        z: puzzle.cubes.keys().map(|i| i.z + 1).max().unwrap(),
    };

    let mut visited = HashSet::new();
    let mut to_visit = VecDeque::from([min]);

    while let Some(coord) = to_visit.pop_front() {
        if !visited.insert(coord) {
            continue;
        }

        for adjacent in coord.adjacent() {
            let coord = adjacent.1;

            if coord.x < min.x
                || coord.y < min.y
                || coord.z < min.z
                || coord.x > max.x
                || coord.y > max.y
                || coord.z > max.z
            {
                continue;
            }

            if let Some(cube) = puzzle.cubes.get_mut(&coord) {
                cube.set(&adjacent.0.opposite(), true)
            } else {
                to_visit.push_back(coord)
            }
        }
    }

    puzzle.cubes.values().map(|i| i.count(|&j| j)).sum()
}

impl Puzzle {
    fn parse(input: Vec<String>) -> Puzzle {
        Puzzle {
            cubes: input
                .iter()
                .map(|i| (Coord::parse(i), Cube::default()))
                .collect(),
        }
    }
}

impl Coord {
    fn parse(value: &str) -> Coord {
        let mut coords = value.trim().split(",").map(|i| i.parse().unwrap());
        Coord {
            x: coords.next().unwrap(),
            y: coords.next().unwrap(),
            z: coords.next().unwrap(),
        }
    }

    fn adjacent(&self) -> [(Side, Coord); 6] {
        [
            (
                Side::Top,
                Coord {
                    x: self.x,
                    y: self.y - 1,
                    z: self.z,
                },
            ),
            (
                Side::Bottom,
                Coord {
                    x: self.x,
                    y: self.y + 1,
                    z: self.z,
                },
            ),
            (
                Side::Left,
                Coord {
                    x: self.x - 1,
                    y: self.y,
                    z: self.z,
                },
            ),
            (
                Side::Right,
                Coord {
                    x: self.x + 1,
                    y: self.y,
                    z: self.z,
                },
            ),
            (
                Side::Front,
                Coord {
                    x: self.x,
                    y: self.y,
                    z: self.z - 1,
                },
            ),
            (
                Side::Back,
                Coord {
                    x: self.x,
                    y: self.y,
                    z: self.z + 1,
                },
            ),
        ]
    }
}

impl Side {
    fn opposite(&self) -> Side {
        match self {
            Side::Top => Side::Bottom,
            Side::Bottom => Side::Top,
            Side::Left => Side::Right,
            Side::Right => Side::Left,
            Side::Front => Side::Back,
            Side::Back => Side::Front,
        }
    }
}

impl<T> Cube<T> {
    fn set(&mut self, side: &Side, value: T) {
        match side {
            Side::Top => {
                self.top = value;
            }
            Side::Bottom => {
                self.bottom = value;
            }
            Side::Left => {
                self.left = value;
            }
            Side::Right => {
                self.right = value;
            }
            Side::Front => {
                self.front = value;
            }
            Side::Back => {
                self.back = value;
            }
        }
    }

    fn values(&self) -> [&T; 6] {
        [
            &self.top,
            &self.bottom,
            &self.left,
            &self.right,
            &self.front,
            &self.back,
        ]
    }

    fn count<P>(&self, predicate: P) -> usize
    where
        P: Fn(&T) -> bool,
    {
        self.values().iter().filter(|i| predicate(i)).count()
    }
}
