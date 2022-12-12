use crate::common::get_input_lines;
use std::collections::HashSet;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Pos(usize);

#[derive(Copy, Clone)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Elevation(u8);

struct Field {
    width: usize,
    height: usize,
    squares: Vec<Elevation>,
    start: Pos,
    end: Pos,
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[allow(dead_code)]
pub fn run() {
    let field = Field::parse(&get_input_lines());

    let result = field.find_shortest_path_length();
    println!("Result (part 1): {result}");
}

impl Field {
    fn parse(input: &Vec<String>) -> Field {
        let width = input[0].len();
        let mut squares = Vec::with_capacity(width * input.len());
        let mut start = Pos::default();
        let mut end = Pos::default();

        for line in input {
            if line.is_empty() {
                continue;
            }

            if line.len() != width {
                panic!("Invalid line width");
            }

            for c in line.chars() {
                match c {
                    'S' => start = Pos::from_index(squares.len()),
                    'E' => end = Pos::from_index(squares.len()),
                    _ => {}
                }

                squares.push(c.into());
            }
        }

        Field {
            width,
            height: squares.len() / width,
            squares,
            start,
            end,
        }
    }

    fn find_shortest_path_length(&self) -> usize {
        // Dijkstra's algorithm, as in https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm

        let mut dist = (0..self.squares.len())
            .map(|_| usize::MAX)
            .collect::<Vec<_>>();

        let mut queue = (0..self.squares.len())
            .map(|i| Pos::from_index(i))
            .collect::<HashSet<_>>();

        dist[self.start.index()] = 0;

        while !queue.is_empty() {
            let u = queue
                .iter()
                .min_by_key(|pos| dist[pos.index()])
                .copied()
                .unwrap();

            queue.remove(&u);

            if u == self.end {
                return dist[u.index()];
            }

            for v in self.reachable_neighbors(u).filter(|v| queue.contains(v)) {
                let alt = dist[u.index()] + 1;
                if alt < dist[v.index()] {
                    dist[v.index()] = alt;
                }
            }
        }

        panic!("No path found")
    }

    fn coord(&self, pos: Pos) -> Coord {
        Coord {
            x: pos.index() % self.width,
            y: pos.index() / self.width,
        }
    }

    fn pos(&self, coord: Coord) -> Pos {
        Pos(coord.x + coord.y * self.width)
    }

    fn elevation(&self, pos: Pos) -> Elevation {
        self.squares[pos.index()]
    }

    fn neighbor(&self, pos: Pos, dir: Direction) -> Option<Pos> {
        let coord = self.coord(pos);

        let ok = match dir {
            Direction::Up if coord.y > 0 => true,
            Direction::Down if coord.y < self.height - 1 => true,
            Direction::Left if coord.x > 0 => true,
            Direction::Right if coord.x < self.width - 1 => true,
            _ => false,
        };

        if ok {
            Some(self.pos(coord.translate(dir)))
        } else {
            None
        }
    }

    fn reachable_neighbor(&self, pos: Pos, dir: Direction) -> Option<Pos> {
        self.neighbor(pos, dir)
            .filter(|n| self.elevation(pos).can_move_to(self.elevation(*n)))
    }

    fn reachable_neighbors(&self, square: Pos) -> ReachableNeighborsIter {
        ReachableNeighborsIter::new(self, square)
    }
}

struct ReachableNeighborsIter<'a> {
    field: &'a Field,
    pos: Pos,
    dir: Option<Direction>,
}

impl<'a> ReachableNeighborsIter<'a> {
    fn new(field: &'a Field, pos: Pos) -> ReachableNeighborsIter<'a> {
        ReachableNeighborsIter {
            field,
            pos,
            dir: Some(Direction::Up),
        }
    }
}

impl<'a> Iterator for ReachableNeighborsIter<'a> {
    type Item = Pos;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(dir) = self.dir {
                let result = self.field.reachable_neighbor(self.pos, dir);

                self.dir = match dir {
                    Direction::Up => Some(Direction::Down),
                    Direction::Down => Some(Direction::Left),
                    Direction::Left => Some(Direction::Right),
                    Direction::Right => None,
                };

                if result.is_some() {
                    return result;
                }
            } else {
                return None;
            }
        }
    }
}

impl Elevation {
    fn can_move_to(&self, other: Elevation) -> bool {
        other.0 <= self.0 + 1
    }
}

impl Default for Pos {
    fn default() -> Self {
        Pos(0)
    }
}

impl Pos {
    fn from_index(index: usize) -> Pos {
        Pos(index)
    }

    fn index(&self) -> usize {
        self.0
    }
}

impl Coord {
    fn new(x: usize, y: usize) -> Coord {
        Coord { x, y }
    }

    fn translate(&self, dir: Direction) -> Coord {
        match dir {
            Direction::Up => Coord::new(self.x, self.y - 1),
            Direction::Down => Coord::new(self.x, self.y + 1),
            Direction::Left => Coord::new(self.x - 1, self.y),
            Direction::Right => Coord::new(self.x + 1, self.y),
        }
    }
}

impl From<char> for Elevation {
    fn from(c: char) -> Self {
        match c {
            'a'..='z' => Elevation((c as u32 - 'a' as u32) as u8),
            'S' => Elevation(0),
            'E' => Elevation('z' as u8 - 'a' as u8),
            _ => panic!("Invalid elevation: {c}"),
        }
    }
}
