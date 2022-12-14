use crate::common::get_input_lines;

#[allow(dead_code)]
pub fn run() {
    let mut list = get_input_lines()
        .iter()
        .map(|line| line.parse::<i32>())
        .fold(vec![0], |mut list, item| {
            if let Ok(value) = item {
                *list.last_mut().unwrap() += value;
            } else {
                list.push(0);
            }
            list
        });

    let max = list.iter().max().unwrap();
    println!("Max calories: {}", max);

    list.sort_unstable_by(|a, b| b.cmp(a));
    let first3 = list.iter().take(3).sum::<i32>();
    println!("Calories of first 3 elves: {}", first3);
}
