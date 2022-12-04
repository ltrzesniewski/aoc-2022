use lazy_static::lazy_static;
use regex::Regex;
use std::io::stdin;
use std::ops::RangeInclusive;

struct InputItem(RangeInclusive<i32>, RangeInclusive<i32>);

#[allow(dead_code)]
pub fn run() {
    let input = stdin()
        .lines()
        .map(|i| i.unwrap().as_str().into())
        .collect::<Vec<InputItem>>();

    let result = input.iter().filter(|i| i.is_fully_contained()).count();

    println!("Result (part 1): {}", result);
}

impl InputItem {
    pub fn is_fully_contained(&self) -> bool {
        self.0.start() <= self.1.start() && self.0.end() >= self.1.end()
            || self.1.start() <= self.0.start() && self.1.end() >= self.0.end()
    }
}

impl From<&str> for InputItem {
    fn from(line: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^([0-9]+)-([0-9]+),([0-9]+)-([0-9]+)$").unwrap();
        }

        let cap = RE.captures(line).unwrap();

        Self(
            cap[1].parse::<i32>().unwrap()..=cap[2].parse::<i32>().unwrap(),
            cap[3].parse::<i32>().unwrap()..=cap[4].parse::<i32>().unwrap(),
        )
    }
}
