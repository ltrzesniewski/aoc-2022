use crate::common::get_input_lines;
use lazy_static::lazy_static;
use regex::Regex;
use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Copy, Clone)]
struct Bounds(Point, Point);

struct Report {
    items: Vec<ReportItem>,
}

struct ReportItem {
    sensor: Point,
    beacon: Point,
}

#[allow(dead_code)]
pub fn run() {
    let items = get_input_lines()
        .iter()
        .filter(|i| !i.is_empty())
        .map(|i| i.parse().unwrap())
        .collect::<Vec<ReportItem>>();

    let report = Report { items };

    let result = part1(&report, 2000000);
    println!("Result (part 1): {result}");
}

fn part1(report: &Report, line_index: i64) -> usize {
    let (min, max) = report.coverage_x_bounds().into_inner();

    let mut coverage = vec![false; (max - min + 1) as usize];

    for item in report.items.iter() {
        if let Some(range) = item.coverage_x_bounds_at_y(line_index) {
            for x in range {
                coverage[(x - min) as usize] = true;
            }
        }
    }

    for item in report.items.iter() {
        if item.beacon.y == line_index {
            coverage[(item.beacon.x - min) as usize] = false;
        }
    }

    coverage.into_iter().filter(|&i| i).count()
}

impl Report {
    fn coverage_x_bounds(&self) -> RangeInclusive<i64> {
        let mut result = self.items[0].coverage_x_bounds();
        for item in self.items.iter().skip(1) {
            let range = item.coverage_x_bounds();
            result = *result.start().min(range.start())..=*result.end().max(range.end());
        }
        result
    }
}

impl ReportItem {
    fn distance(&self) -> u64 {
        self.sensor.distance(self.beacon)
    }

    fn coverage_x_bounds(&self) -> RangeInclusive<i64> {
        let dist = self.distance() as i64;
        (self.sensor.x - dist)..=(self.sensor.x + dist)
    }

    fn coverage_x_bounds_at_y(&self, y: i64) -> Option<RangeInclusive<i64>> {
        let dist = self.distance() as i64;
        let diff_y = y.abs_diff(self.sensor.y) as i64;

        if diff_y <= dist {
            Some((self.sensor.x - dist + diff_y)..=(self.sensor.x + dist - diff_y))
        } else {
            None
        }
    }
}

impl FromStr for ReportItem {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"Sensor at x=([-0-9]+), y=([-0-9]+): closest beacon is at x=([-0-9]+), y=([-0-9]+)"
            )
            .unwrap();
        }

        let cap = RE.captures(s).unwrap();

        Ok(Self {
            sensor: Point::new(cap[1].parse().unwrap(), cap[2].parse().unwrap()),
            beacon: Point::new(cap[3].parse().unwrap(), cap[4].parse().unwrap()),
        })
    }
}

impl Point {
    fn new(x: i64, y: i64) -> Point {
        Point { x, y }
    }

    fn distance(&self, p: Point) -> u64 {
        self.x.abs_diff(p.x) + self.y.abs_diff(p.y)
    }
}
