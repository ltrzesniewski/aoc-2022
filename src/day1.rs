use std::io::stdin;

pub fn part1() {
    let groups = stdin()
        .lines()
        .map(|line| line.unwrap().parse::<i32>())
        .fold(vec![0], |mut list, item| {
            if let Ok(value) = item {
                *list.last_mut().unwrap() += value;
            } else {
                list.push(0);
            }
            list
        });

    let max = groups.iter().max().unwrap();

    println!("Max calories: {}", max);
}
