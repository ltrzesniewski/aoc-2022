use crate::common::get_input_lines;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone)]
enum Direction {
    Left,
    Right,
}

#[derive(Clone)]
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

struct Puzzle {
    cave: Cave,
    pushes: Pushes,
    block_count: usize,
    push_count: usize,
}

#[allow(dead_code)]
pub fn run() {
    let pushes = Pushes::parse(get_input_lines().iter().next().unwrap());

    let result = part1(&pushes, 2022);
    println!("Result (part 1): {result}");

    let result = part2(&pushes, 1000000000000);
    println!("Result (part 2): {result}");
}

fn part1(pushes: &Pushes, iterations: usize) -> usize {
    let mut puzzle = Puzzle::new(pushes.clone());

    loop {
        puzzle.drop_block();

        if puzzle.block_count == iterations {
            break;
        }
    }

    return puzzle.cave.height();
}

fn part2(pushes: &Pushes, iterations: usize) -> usize {
    let mut puzzle = Puzzle::new(pushes.clone());

    #[derive(Copy, Clone, Default)]
    struct Status {
        count: usize,
        height: usize,
        blocks: usize,
    }

    let mut combinations = vec![Status::default(); puzzle.combination_count()];

    loop {
        let status = &mut combinations[puzzle.current_combination()];

        puzzle.drop_block();

        if puzzle.block_count == iterations {
            return puzzle.cave.height();
        }

        status.count += 1;

        let prev_height = status.height;
        status.height = puzzle.cave.height();

        let prev_blocks = status.blocks;
        status.blocks = puzzle.block_count;

        if status.count == 3 && status.height > prev_height {
            let diff_height = status.height - prev_height;
            let diff_blocks = status.blocks - prev_blocks;

            let needed_blocks = iterations - puzzle.block_count;
            let additional_iterations = needed_blocks / diff_blocks;

            let additional_blocks = additional_iterations * diff_blocks;
            let additional_height = additional_iterations * diff_height;

            loop {
                let blocks = puzzle.block_count + additional_blocks;

                if blocks == iterations {
                    return puzzle.cave.height() + additional_height;
                }

                puzzle.drop_block();
            }
        }
    }
}

impl Puzzle {
    fn new(pushes: Pushes) -> Puzzle {
        Puzzle {
            cave: Cave::new(),
            pushes,
            block_count: 0,
            push_count: 0,
        }
    }

    fn drop_block(&mut self) {
        let mut block = Block::new(self.block_count, self.cave.height() + 3);
        self.block_count += 1;

        loop {
            let next_push = self.pushes.get(self.push_count);
            self.push_count += 1;

            block = block.push(next_push, &self.cave);
            if let Ok(fallen) = block.fall(&self.cave) {
                block = fallen;
            } else {
                self.cave.merge_block(&block);
                break;
            }
        }
    }

    fn combination_count(&self) -> usize {
        Block::TYPE_COUNT * self.pushes.0.len()
    }

    fn current_combination(&self) -> usize {
        let block_index = self.block_count % Block::TYPE_COUNT;
        let push_index = self.push_count % self.pushes.0.len();
        block_index * self.pushes.0.len() + push_index
    }
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
    const TYPE_COUNT: usize = 5;

    fn new(index: usize, height: usize) -> Block {
        let lines = match index % Self::TYPE_COUNT {
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
