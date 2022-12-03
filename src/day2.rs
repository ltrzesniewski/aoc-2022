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

#[allow(dead_code)]
pub fn run() {
    let input = stdin().lines().map(|i| i.unwrap()).collect::<Vec<_>>();

    let score = input
        .iter()
        .filter_map(|i| parse_round_part1(&i))
        .map(|i| outcome_score(play(&i)) + shape_score(i.player))
        .sum::<i32>();

    println!("Score (part 1): {}", score);

    let score = input
        .iter()
        .filter_map(|i| parse_round_part2(&i))
        .map(|i| outcome_score(play(&i)) + shape_score(i.player))
        .sum::<i32>();

    println!("Score (part 2): {}", score);
}

fn parse_round_part1(line: &str) -> Option<Round> {
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

fn parse_round_part2(line: &str) -> Option<Round> {
    let line = line.split_whitespace().collect::<Vec<_>>();
    if line.len() != 2 {
        return None;
    }

    let opponent = match line[0] {
        "A" => Shape::Rock,
        "B" => Shape::Paper,
        "C" => Shape::Scissors,
        _ => panic!(),
    };

    let outcome = match line[1] {
        "X" => Outcome::Loss,
        "Y" => Outcome::Draw,
        "Z" => Outcome::Win,
        _ => panic!(),
    };

    let player = match (&opponent, &outcome) {
        (Shape::Rock, Outcome::Draw) => Shape::Rock,
        (Shape::Rock, Outcome::Win) => Shape::Paper,
        (Shape::Rock, Outcome::Loss) => Shape::Scissors,
        (Shape::Paper, Outcome::Draw) => Shape::Paper,
        (Shape::Paper, Outcome::Win) => Shape::Scissors,
        (Shape::Paper, Outcome::Loss) => Shape::Rock,
        (Shape::Scissors, Outcome::Draw) => Shape::Scissors,
        (Shape::Scissors, Outcome::Win) => Shape::Rock,
        (Shape::Scissors, Outcome::Loss) => Shape::Paper,
    };

    Some(Round { opponent, player })
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
    match shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    }
}

fn outcome_score(outcome: Outcome) -> i32 {
    match outcome {
        Outcome::Draw => 3,
        Outcome::Win => 6,
        Outcome::Loss => 0,
    }
}
