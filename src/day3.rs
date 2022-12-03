use std::collections::HashSet;
use std::io::stdin;

#[allow(dead_code)]
pub fn run() {
    let result = stdin()
        .lines()
        .map(|i| i.unwrap())
        .map(|i| {
            let compartments = i.split_at(i.len() / 2);
            let a = compartments.0.bytes().collect::<HashSet<_>>();
            let b = compartments.1.bytes().collect::<HashSet<_>>();
            a.intersection(&b).copied().next().unwrap()
        })
        .map(|i| {
            if (b'a'..=b'z').contains(&i) {
                (i - b'a' + 1) as i32
            } else if (b'A'..=b'Z').contains(&i) {
                (i - b'A' + 27) as i32
            } else {
                panic!()
            }
        })
        .sum::<i32>();

    println!("Result: {}", result);
}
