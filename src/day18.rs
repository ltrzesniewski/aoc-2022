use crate::common::get_input_lines;
use std::collections::HashMap;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Coord {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Default)]
struct Cube {
    top: bool,
    bottom: bool,
    left: bool,
    right: bool,
    front: bool,
    back: bool,
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

struct Puzzle {
    cubes: HashMap<Coord, Cube>,
}

#[allow(dead_code)]
pub fn run() {
    let puzzle = Puzzle::parse(get_input_lines());

    let result = part1(puzzle);
    println!("Result (part 1): {result}");
}

fn part1(mut puzzle: Puzzle) -> usize {
    for coord in puzzle.cubes.keys().copied().collect::<Vec<_>>() {
        for adjacent in coord.adjacent() {
            if let Some(cube) = puzzle.cubes.get_mut(&adjacent.1) {
                cube.set(&adjacent.0.opposite(), true)
            }
        }
    }

    puzzle.cubes.values().map(|i| i.surface_area()).sum()
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

impl Cube {
    fn set(&mut self, side: &Side, value: bool) {
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

    fn values(&self) -> [bool; 6] {
        [
            self.top,
            self.bottom,
            self.left,
            self.right,
            self.front,
            self.back,
        ]
    }

    fn surface_area(&self) -> usize {
        self.values().iter().filter(|&&i| !i).count()
    }
}
