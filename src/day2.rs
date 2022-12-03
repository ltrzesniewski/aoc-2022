use std::io::stdin;

enum Shape {
    Rock,
    Paper,
    Scissors,
}

struct Round {
    pub opponent: Shape,
    pub player: Shape,
}

enum Outcome {
    Draw,
    Win,
    Loss,
}

pub fn run() {
    let score = stdin()
        .lines()
        .filter_map(|i| parse_round(&i.unwrap()))
        .map(|i| (play(&i), i))
        .fold(0, |score, (outcome, round)| {
            score + shape_score(round.player) + outcome_score(outcome)
        });

    println!("Score: {}", score);
}

fn parse_round(line: &str) -> Option<Round> {
    let line = line.split_whitespace().collect::<Vec<_>>();
    if line.len() != 2 {
        return None;
    }
    Some(Round {
        opponent: match line[0] {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scissors,
            _ => panic!(),
        },
        player: match line[1] {
            "X" => Shape::Rock,
            "Y" => Shape::Paper,
            "Z" => Shape::Scissors,
            _ => panic!(),
        },
    })
}

fn play(round: &Round) -> Outcome {
    match (&round.player, &round.opponent) {
        (Shape::Rock, Shape::Rock) => Outcome::Draw,
        (Shape::Rock, Shape::Paper) => Outcome::Loss,
        (Shape::Rock, Shape::Scissors) => Outcome::Win,
        (Shape::Paper, Shape::Rock) => Outcome::Win,
        (Shape::Paper, Shape::Paper) => Outcome::Draw,
        (Shape::Paper, Shape::Scissors) => Outcome::Loss,
        (Shape::Scissors, Shape::Rock) => Outcome::Loss,
        (Shape::Scissors, Shape::Paper) => Outcome::Win,
        (Shape::Scissors, Shape::Scissors) => Outcome::Draw,
    }
}

fn shape_score(shape: Shape) -> i32 {
    match &shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    }
}
fn outcome_score(outcome: Outcome) -> i32 {
    match &outcome {
        Outcome::Draw => 3,
        Outcome::Win => 6,
        Outcome::Loss => 0,
    }
}
