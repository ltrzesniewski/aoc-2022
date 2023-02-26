use crate::common::get_input_lines;

#[derive(Copy, Clone)]
struct Number {
    value: isize,
    orig_index: usize,
}

#[derive(Clone)]
struct File {
    numbers: Vec<Number>,
}

#[allow(dead_code)]
pub fn run() {
    let file = File::parse(get_input_lines());

    let result = part1(file.clone());
    println!("Result (part 1): {result}");

    let result = part2(file);
    println!("Result (part 2): {result}");
}

fn part1(mut file: File) -> isize {
    file.mix();
    file.get_result()
}

fn part2(mut file: File) -> isize {
    for mut n in file.numbers.iter_mut() {
        n.value *= 811589153;
    }

    for _ in 0..10 {
        file.mix()
    }

    file.get_result()
}

impl File {
    fn parse(lines: Vec<String>) -> File {
        let numbers = lines
            .iter()
            .filter_map(|i| i.parse().ok())
            .enumerate()
            .map(|(i, n)| Number {
                value: n,
                orig_index: i,
            })
            .collect();

        File { numbers }
    }

    fn mix(&mut self) {
        let numbers = &mut self.numbers;

        for iter in 0..numbers.len() {
            let index = numbers.iter().position(|i| i.orig_index == iter).unwrap();
            let number = numbers[index];

            let new_index = (index as isize + number.value).rem_euclid(numbers.len() as isize - 1);
            let new_index = new_index as usize;

            numbers.remove(index);
            numbers.insert(new_index, number);
        }
    }

    fn get_result(&self) -> isize {
        let zero_index = self.index_of_value(0);
        self.get_value_at(zero_index + 1000)
            + self.get_value_at(zero_index + 2000)
            + self.get_value_at(zero_index + 3000)
    }

    fn index_of_value(&self, value: isize) -> usize {
        self.numbers.iter().position(|i| i.value == value).unwrap()
    }

    fn get_value_at(&self, index: usize) -> isize {
        self.numbers[index % self.numbers.len()].value
    }
}
