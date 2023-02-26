use crate::common::get_input_lines;

#[derive(Copy, Clone, PartialEq)]
enum Tile {
    None,
    Open,
    Wall,
}

#[derive(Copy, Clone)]
enum Step {
    Advance(usize),
    Rotate(Rotation),
}

#[derive(Copy, Clone)]
enum Rotation {
    Left,
    Right,
}

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone)]
struct Coord(isize, isize);

#[derive(Clone)]
struct Puzzle {
    board: Vec<Vec<Tile>>,
    steps: Vec<Step>,
    coord: Coord,
    direction: Direction,
    width: usize,
}

#[allow(dead_code)]
pub fn run() {
    let puzzle = Puzzle::parse(get_input_lines());

    let result = part1(puzzle.clone());
    println!("Result (part 1): {result}");
}

fn part1(mut puzzle: Puzzle) -> usize {
    puzzle.process();
    puzzle.get_password()
}

impl Puzzle {
    fn parse(lines: Vec<String>) -> Puzzle {
        let mut board = vec![];

        let mut lines_iter = lines.iter();
        while let Some(str) = lines_iter.next() {
            if str.is_empty() {
                break;
            }

            let line = str
                .chars()
                .map(|c| match c {
                    ' ' => Tile::None,
                    '.' => Tile::Open,
                    '#' => Tile::Wall,
                    _ => panic!(),
                })
                .collect::<Vec<_>>();

            board.push(line);
        }

        let steps_str = lines_iter.next().unwrap();
        let mut steps = vec![];
        for value in steps_str.split_inclusive(&['R', 'L']) {
            let last_char = value.chars().last().unwrap();
            if last_char.is_numeric() {
                steps.push(Step::Advance(value.parse().unwrap()));
            } else {
                steps.push(Step::Advance(value[0..value.len() - 1].parse().unwrap()));
                steps.push(Step::Rotate(match last_char {
                    'R' => Rotation::Right,
                    'L' => Rotation::Left,
                    _ => panic!(),
                }));
            }
        }

        let x = board[0].iter().position(|&i| i == Tile::Open).unwrap();
        let width = board.iter().map(|i| i.len()).max().unwrap();

        Puzzle {
            board,
            steps,
            coord: Coord(x as isize, 0),
            direction: Direction::Right,
            width,
        }
    }

    fn process(&mut self) {
        for i in 0..self.steps.len() {
            self.process_step(self.steps[i]);
        }
    }

    fn process_step(&mut self, step: Step) {
        match step {
            Step::Advance(n) => self.advance(n),
            Step::Rotate(d) => self.rotate(d),
        }
    }

    fn rotate(&mut self, rotation: Rotation) {
        self.direction = match (self.direction, rotation) {
            (Direction::Up, Rotation::Right) => Direction::Right,
            (Direction::Right, Rotation::Right) => Direction::Down,
            (Direction::Down, Rotation::Right) => Direction::Left,
            (Direction::Left, Rotation::Right) => Direction::Up,
            (Direction::Up, Rotation::Left) => Direction::Left,
            (Direction::Left, Rotation::Left) => Direction::Down,
            (Direction::Down, Rotation::Left) => Direction::Right,
            (Direction::Right, Rotation::Left) => Direction::Up,
        }
    }

    fn advance(&mut self, steps: usize) {
        for _ in 0..steps {
            let next_coord = self.get_coord_ahead();
            let tile_ahead = self.get_tile(next_coord);
            match tile_ahead {
                Tile::None => panic!(),
                Tile::Open => {
                    self.coord = next_coord;
                }
                Tile::Wall => break,
            }
        }
    }

    fn get_tile(&self, coord: Coord) -> Tile {
        if coord.0 < 0 || coord.1 < 0 || coord.1 as usize >= self.board.len() {
            return Tile::None;
        }

        let line = &self.board[coord.1 as usize];
        line.get(coord.0 as usize).copied().unwrap_or(Tile::None)
    }

    fn get_coord_ahead(&self) -> Coord {
        let next = self.coord.next(self.direction);

        match self.get_tile(next) {
            Tile::Open => next,
            Tile::Wall => self.coord,
            Tile::None => match self.direction {
                Direction::Up => (0..self.board_height())
                    .map(|y| Coord(self.coord.0, y as isize))
                    .rfind(|&i| self.get_tile(i) != Tile::None)
                    .unwrap(),
                Direction::Down => (0..self.board_height())
                    .map(|y| Coord(self.coord.0, y as isize))
                    .find(|&i| self.get_tile(i) != Tile::None)
                    .unwrap(),
                Direction::Left => (0..self.board_width())
                    .map(|x| Coord(x as isize, self.coord.1))
                    .rfind(|&i| self.get_tile(i) != Tile::None)
                    .unwrap(),
                Direction::Right => (0..self.board_width())
                    .map(|x| Coord(x as isize, self.coord.1))
                    .find(|&i| self.get_tile(i) != Tile::None)
                    .unwrap(),
            },
        }
    }

    fn get_password(&self) -> usize {
        ((self.coord.1 as usize + 1) * 1000)
            + ((self.coord.0 as usize + 1) * 4)
            + match self.direction {
                Direction::Up => 3,
                Direction::Down => 1,
                Direction::Left => 2,
                Direction::Right => 0,
            }
    }

    fn board_width(&self) -> usize {
        self.width
    }

    fn board_height(&self) -> usize {
        self.board.len()
    }
}

impl Coord {
    fn next(&self, dir: Direction) -> Coord {
        match dir {
            Direction::Up => Coord(self.0, self.1 - 1),
            Direction::Down => Coord(self.0, self.1 + 1),
            Direction::Left => Coord(self.0 - 1, self.1),
            Direction::Right => Coord(self.0 + 1, self.1),
        }
    }
}
