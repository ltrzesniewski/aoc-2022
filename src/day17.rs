use crate::common::get_input_lines;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone)]
enum Direction {
    Left,
    Right,
}

struct Pushes(Vec<Direction>);

#[derive(Copy, Clone, Eq, PartialEq)]
struct Line(u8);

#[derive(Clone)]
struct Block {
    lines: [Line; 4],
    height: usize,
}

struct Cave {
    lines: Vec<Line>,
}

#[allow(dead_code)]
pub fn run() {
    let pushes = Pushes::parse(get_input_lines().iter().next().unwrap());

    let result = part1(&pushes);
    println!("Result (part 1): {result}");
}

fn part1(pushes: &Pushes) -> usize {
    let mut cave = Cave::new();
    let mut block_count = 0;
    let mut push_count = 0;

    loop {
        let mut block = Block::new(block_count, cave.height() + 3);
        block_count += 1;

        loop {
            let next_push = pushes.get(push_count);
            push_count += 1;

            block = block.push(next_push, &cave);
            if let Ok(fallen) = block.fall(&cave) {
                block = fallen;
            } else {
                cave.merge_block(&block);
                break;
            }
        }

        if block_count == 2022 {
            break;
        }
    }

    return cave.height();
}

impl Pushes {
    fn parse(value: &str) -> Pushes {
        let mut result = Vec::with_capacity(value.len());

        for b in value.bytes() {
            result.push(match b {
                b'<' => Direction::Left,
                b'>' => Direction::Right,
                _ => panic!(),
            })
        }

        Pushes(result)
    }

    fn get(&self, index: usize) -> Direction {
        self.0[index % self.0.len()]
    }
}

impl Cave {
    fn new() -> Cave {
        Cave { lines: vec![] }
    }

    fn height(&self) -> usize {
        self.lines
            .iter()
            .rposition(|i| !i.is_empty())
            .map_or(0, |i| i + 1)
    }

    fn get(&self, i: usize) -> Line {
        self.lines.get(i).copied().unwrap_or_default()
    }

    fn merge_block(&mut self, block: &Block) {
        if self.lines.len() < block.height + block.lines.len() {
            self.lines
                .resize(block.height + block.lines.len(), Line::default());
        }

        for (i, &block_line) in block.lines.iter().enumerate() {
            let cave_line = self.lines.get_mut(block.height + i).unwrap();
            *cave_line = cave_line.merge(block_line);
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        println!();
        for line in self.lines.iter().rev() {
            println!("|{}|", line);
        }
        println!("+-------+");
    }
}

impl Line {
    fn push(&self, dir: Direction) -> Line {
        Line(match dir {
            Direction::Left => {
                if self.0 & 0b01000000 == 0 {
                    self.0 << 1
                } else {
                    self.0
                }
            }
            Direction::Right => {
                if self.0 & 0b1 == 0 {
                    self.0 >> 1
                } else {
                    self.0
                }
            }
        })
    }

    fn overlaps(&self, other: Line) -> bool {
        self.0 & other.0 != 0
    }

    fn is_empty(&self) -> bool {
        self.0 == 0
    }

    fn merge(&self, other: Line) -> Line {
        Line(self.0 | other.0)
    }
}

impl Default for Line {
    fn default() -> Self {
        Line(0)
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 0..7 {
            write!(f, "{}", if self.0 & (1 << 6 - i) != 0 { '#' } else { '.' })?
        }
        Ok(())
    }
}

impl Block {
    fn new(index: usize, height: usize) -> Block {
        let lines = match index % 5 {
            0 => [
                Line(0b0011110),
                Line(0b0000000),
                Line(0b0000000),
                Line(0b0000000),
            ],
            1 => [
                Line(0b0001000),
                Line(0b0011100),
                Line(0b0001000),
                Line(0b0000000),
            ],
            2 => [
                Line(0b0011100),
                Line(0b0000100),
                Line(0b0000100),
                Line(0b0000000),
            ],
            3 => [
                Line(0b0010000),
                Line(0b0010000),
                Line(0b0010000),
                Line(0b0010000),
            ],
            4 => [
                Line(0b0011000),
                Line(0b0011000),
                Line(0b0000000),
                Line(0b0000000),
            ],
            _ => unreachable!(),
        };

        Block { lines, height }
    }

    fn push(&self, dir: Direction, cave: &Cave) -> Block {
        let mut result = self.clone();

        for (idx, line) in result.lines.iter_mut().enumerate() {
            if line.is_empty() {
                break;
            }

            let pushed = line.push(dir);

            if pushed == *line {
                return self.clone();
            }

            if cave.get(self.height + idx).overlaps(pushed) {
                return self.clone();
            }

            *line = pushed
        }

        result
    }

    fn fall(&self, cave: &Cave) -> Result<Block, ()> {
        if self.height == 0 {
            return Err(());
        }

        for (idx, &line) in self.lines.iter().enumerate() {
            if cave.get(self.height + idx - 1).overlaps(line) {
                return Err(());
            }
        }

        return Ok(Block {
            lines: self.lines,
            height: self.height - 1,
        });
    }
}
