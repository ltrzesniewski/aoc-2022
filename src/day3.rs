use crate::common::get_input_lines;
use std::collections::HashSet;

#[allow(dead_code)]
pub fn run() {
    let input = get_input_lines();

    let result = input
        .iter()
        .map(|i| {
            let compartments = i.split_at(i.len() / 2);
            let a = compartments.0.bytes().collect::<HashSet<_>>();
            let b = compartments.1.bytes().collect::<HashSet<_>>();
            a.intersection(&b).copied().next().unwrap()
        })
        .map(|i| get_score(i))
        .sum::<i32>();

    println!("Result (part 1): {}", result);

    let result = input
        .iter()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|group| {
            let item = group
                .iter()
                .map(|i| i.bytes().collect::<HashSet<u8>>())
                .fold(None::<HashSet<u8>>, |acc, i| {
                    Some(if let Some(set) = acc {
                        set.intersection(&i).copied().collect::<HashSet<u8>>()
                    } else {
                        i
                    })
                })
                .into_iter()
                .next()
                .unwrap()
                .into_iter()
                .next()
                .unwrap();

            get_score(item)
        })
        .sum::<i32>();

    println!("Result (part 2): {}", result);
}

fn get_score(i: u8) -> i32 {
    if (b'a'..=b'z').contains(&i) {
        (i - b'a' + 1) as i32
    } else if (b'A'..=b'Z').contains(&i) {
        (i - b'A' + 27) as i32
    } else {
        panic!()
    }
}
