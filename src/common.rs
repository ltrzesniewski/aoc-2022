use std::io::stdin;

pub fn get_input_lines() -> Vec<String> {
    println!("Enter input:");
    stdin().lines().map(|i| i.unwrap()).collect::<Vec<_>>()
}
