use crate::common::get_input_lines;

#[derive(Copy, Clone)]
struct Number {
    value: isize,
    orig_index: usize,
}

struct File {
    numbers: Vec<Number>,
}

#[allow(dead_code)]
pub fn run() {
    let file = File::parse(get_input_lines());

    let result = part1(file);
    println!("Result (part 1): {result}");
}

fn part1(mut file: File) -> isize {
    file.mix();

    let zero_index = file.index_of_value(0);
    file.get_value_at(zero_index + 1000)
        + file.get_value_at(zero_index + 2000)
        + file.get_value_at(zero_index + 3000)
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

    fn index_of_value(&self, value: isize) -> usize {
        self.numbers.iter().position(|i| i.value == value).unwrap()
    }

    fn get_value_at(&self, index: usize) -> isize {
        self.numbers[index % self.numbers.len()].value
    }
}
