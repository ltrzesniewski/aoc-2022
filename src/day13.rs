use crate::common::get_input_lines;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Clone)]
enum Value {
    Number(i32),
    List(Vec<Value>),
}

#[allow(dead_code)]
pub fn run() {
    let values = get_input_lines()
        .iter()
        .filter(|i| !i.is_empty())
        .map(|i| i.parse().unwrap())
        .collect::<Vec<Value>>();

    let result = part1(&values);
    println!("Result (part 1): {result}");

    let result = part2(&values);
    println!("Result (part 2): {result}");
}

fn part1(values: &[Value]) -> usize {
    let mut result = 0;

    for i in 0..(values.len() / 2) {
        let a = &values[2 * i];
        let b = &values[2 * i + 1];

        println!("{}", a);
        println!("{}", b);
        println!();

        if a <= b {
            result += i + 1;
        }
    }

    result
}

fn part2(values: &[Value]) -> usize {
    let mut values = values.iter().cloned().collect::<Vec<_>>();

    let dividers = ["[[2]]", "[[6]]"]
        .iter()
        .map(|i| i.parse().unwrap())
        .collect::<Vec<Value>>();

    values.extend(dividers.clone());
    values.sort_unstable();

    dividers
        .iter()
        .map(|d| values.binary_search(d).unwrap() + 1)
        .fold(1, |a, i| a * i)
}

impl FromStr for Value {
    type Err = ();

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        fn consume(mut s: &mut &str) -> Value {
            if s.chars().next().unwrap() == '[' {
                *s = &s[1..];
                let mut items = vec![];
                loop {
                    match s.chars().next().unwrap() {
                        ']' => {
                            *s = &s[1..];
                            return Value::List(items);
                        }
                        ',' => *s = &s[1..],
                        _ => items.push(consume(&mut s)),
                    }
                }
            } else {
                let len = s.find(|c: char| !c.is_numeric()).unwrap_or(s.len());
                let result = Value::Number(s[..len].parse().unwrap());
                *s = &s[len..];
                result
            }
        }

        Ok(consume(&mut s))
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a.cmp(b),
            (Value::List(a), Value::List(b)) => {
                for (a, b) in a.iter().zip(b) {
                    let cmp = a.cmp(b);
                    if cmp != Ordering::Equal {
                        return cmp;
                    }
                }
                a.len().cmp(&b.len())
            }
            (Value::Number(_), Value::List(_)) => Value::List(vec![self.clone()]).cmp(other),
            (Value::List(_), Value::Number(_)) => self.cmp(&Value::List(vec![other.clone()])),
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Value {}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(value) => {
                write!(f, "{}", value)?;
            }
            Value::List(values) => {
                write!(f, "[")?;
                for (i, value) in values.iter().enumerate() {
                    if i != 0 {
                        write!(f, ",")?;
                    }
                    write!(f, "{}", value)?;
                }
                write!(f, "]")?;
            }
        };
        Ok(())
    }
}
