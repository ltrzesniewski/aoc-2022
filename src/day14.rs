use crate::common::get_input_lines;
use std::fmt::{Display, Formatter};
use std::ops::RangeInclusive;
use std::str::FromStr;

const SAND_ORIGIN: Point = Point::new(500, 0);

#[derive(Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}

struct Line(Vec<Point>);

#[derive(Copy, Clone)]
struct Segment(Point, Point);

#[derive(Copy, Clone)]
struct Bounds(Point, Point);

#[derive(Copy, Clone)]
enum Tile {
    Void,
    Empty,
    Rock,
    Sand,
}

struct Cave {
    tiles: Vec<Tile>,
    bounds: Bounds,
}

#[allow(dead_code)]
pub fn run() {
    let lines = get_input_lines()
        .iter()
        .filter(|i| !i.is_empty())
        .map(|i| i.parse().unwrap())
        .collect::<Vec<Line>>();

    let mut cave = Cave::new(&lines);

    let result = part1(&mut cave);
    println!("{cave}");
    println!("Result (part 1): {result}");
}

fn part1(cave: &mut Cave) -> usize {
    let mut count = 0;

    loop {
        if cave.add_unit_of_sand() {
            count += 1;
        } else {
            break;
        }
    }

    count
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points = s.split(" -> ").map(|i| i.parse().unwrap()).collect();
        Ok(Line(points))
    }
}

impl Line {
    fn segments(&self) -> impl Iterator<Item = Segment> + '_ {
        (1..self.0.len()).map(|i| Segment::new(self.0[i - 1], self.0[i]))
    }
}

impl Segment {
    fn new(from: Point, to: Point) -> Segment {
        Segment(from, to)
    }

    fn points(&self) -> impl Iterator<Item = Point> + '_ {
        let (min, len, horiz);

        if self.0.x == self.1.x {
            min = self.0.y.min(self.1.y);
            len = self.0.y.abs_diff(self.1.y) + 1;
            horiz = false;
        } else if self.0.y == self.1.y {
            min = self.0.x.min(self.1.x);
            len = self.0.x.abs_diff(self.1.x) + 1;
            horiz = true;
        } else {
            panic!()
        }

        return (0..len).map(move |i| {
            if horiz {
                Point::new(min + i, self.0.y)
            } else {
                Point::new(self.0.x, min + i)
            }
        });
    }
}

impl Bounds {
    fn new(min: Point, max: Point) -> Bounds {
        Bounds(min, max)
    }

    fn from_lines(lines: &[Line]) -> Bounds {
        let mut bounds = Bounds::default();
        for line in lines {
            for &point in line.0.iter() {
                bounds = bounds.extend(point);
            }
        }
        bounds
    }

    fn extend(&self, p: Point) -> Bounds {
        Bounds::new(
            Point::new(self.0.x.min(p.x), self.0.y.min(p.y)),
            Point::new(self.1.x.max(p.x), self.1.y.max(p.y)),
        )
    }

    fn width(&self) -> usize {
        self.1.x - self.0.x + 1
    }

    fn height(&self) -> usize {
        self.1.y - self.0.y + 1
    }

    fn range_x(&self) -> RangeInclusive<usize> {
        self.0.x..=self.1.x
    }

    fn range_y(&self) -> RangeInclusive<usize> {
        self.0.y..=self.1.y
    }

    fn contains(&self, p: Point) -> bool {
        self.range_x().contains(&p.x) && self.range_y().contains(&p.y)
    }
}

impl Default for Bounds {
    fn default() -> Self {
        Bounds::new(
            Point::new(usize::MAX, usize::MAX),
            Point::new(usize::MIN, usize::MIN),
        )
    }
}

impl Cave {
    fn new(lines: &[Line]) -> Cave {
        let bounds = Bounds::from_lines(lines).extend(SAND_ORIGIN);
        let tiles = vec![Tile::Empty; bounds.width() * bounds.height()];
        let mut cave = Cave { tiles, bounds };

        for line in lines {
            cave.set_rock_line(line);
        }

        cave
    }

    fn set_rock_line(&mut self, line: &Line) {
        for segment in line.segments() {
            for point in segment.points() {
                self.set(point, Tile::Rock);
            }
        }
    }

    fn to_index(&self, p: Point) -> Option<usize> {
        if self.bounds.contains(p) {
            Some((p.y - self.bounds.0.y) * self.bounds.width() + p.x - self.bounds.0.x)
        } else {
            None
        }
    }

    fn get(&self, p: Point) -> Tile {
        if let Some(index) = self.to_index(p) {
            self.tiles[index]
        } else {
            Tile::Void
        }
    }

    fn set(&mut self, p: Point, t: Tile) {
        let index = self.to_index(p);
        self.tiles[index.unwrap()] = t;
    }

    fn add_unit_of_sand(&mut self) -> bool {
        let mut pt = SAND_ORIGIN;

        'down: loop {
            let next = [pt.delta(0, 1), pt.delta(-1, 1), pt.delta(1, 1)];

            for pos in next {
                match self.get(pos) {
                    Tile::Void => {
                        return false;
                    }
                    Tile::Empty => {
                        pt = pos;
                        continue 'down;
                    }
                    Tile::Rock | Tile::Sand => {
                        continue;
                    }
                }
            }

            self.set(pt, Tile::Sand);
            return true;
        }
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in self.bounds.range_y() {
            for x in self.bounds.range_x() {
                write!(f, "{}", self.get(Point::new(x, y)))?;
            }
            writeln!(f, "")?;
        }

        Ok(())
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Void => write!(f, "~"),
            Tile::Empty => write!(f, "."),
            Tile::Rock => write!(f, "#"),
            Tile::Sand => write!(f, "o"),
        }
    }
}

impl Point {
    const fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }

    fn delta(&self, x: isize, y: isize) -> Point {
        Self::new(
            (self.x as isize + x) as usize,
            (self.y as isize + y) as usize,
        )
    }
}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(',');
        let x = iter.next().unwrap().parse().unwrap();
        let y = iter.next().unwrap().parse().unwrap();
        if iter.next().is_some() {
            panic!();
        }
        Ok(Point { x, y })
    }
}
