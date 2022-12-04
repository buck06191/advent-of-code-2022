mod types;

use std::fs;
use std::str::FromStr;

use clap::Parser;
use types::Outcome;

use crate::types::Choice;

const WIN: u8 = 6;
const DRAW: u8 = 3;
const LOSE: u8 = 0;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn determine_score(opponent: &Choice, me: &Choice) -> u8 {
    match opponent {
        Choice::Rock => match me {
            Choice::Rock => 1 + DRAW,
            Choice::Paper => 2 + WIN,
            Choice::Scissors => 3 + LOSE,
        },
        Choice::Paper => match me {
            Choice::Rock => 1 + LOSE,
            Choice::Paper => 2 + DRAW,
            Choice::Scissors => 3 + WIN,
        },
        Choice::Scissors => match me {
            Choice::Rock => 1 + WIN,
            Choice::Paper => 2 + LOSE,
            Choice::Scissors => 3 + DRAW,
        },
    }
}

fn determine_score_by_outcome(opponent: &Choice, outcome: &Outcome) -> u8 {
    match opponent {
        Choice::Rock => match outcome {
            Outcome::Draw => 1 + DRAW,
            Outcome::Win => 2 + WIN,
            Outcome::Lose => 3 + LOSE,
        },
        Choice::Paper => match outcome {
            Outcome::Lose => 1 + LOSE,
            Outcome::Draw => 2 + DRAW,
            Outcome::Win => 3 + WIN,
        },
        Choice::Scissors => match outcome {
            Outcome::Win => 1 + WIN,
            Outcome::Lose => 2 + LOSE,
            Outcome::Draw => 3 + DRAW,
        },
    }
}

pub fn handler(input: &str) -> u32 {
    let lines = input.trim().lines();
    let score = lines
        .map(|line: &str| -> u32 {
            let choices: Vec<Choice> = line
                .split(" ")
                .map(|choice: &str| -> Choice {
                    let c = Choice::from_str(choice).unwrap();
                    c
                })
                .collect();

            determine_score(&choices[0], &choices[1]) as u32
        })
        .sum();

    score
}

pub fn handler_2(input: &str) -> u32 {
    let lines = input.trim().lines();
    let score = lines
        .map(|line: &str| -> u32 {
            let mut split = line.split(" ");

            let choices = (
                Choice::from_str(split.next().unwrap()).unwrap(),
                Outcome::from_str(split.next().unwrap()).unwrap(),
            );

            determine_score_by_outcome(&choices.0, &choices.1) as u32
        })
        .sum();

    score
}

fn main() {
    let args = Cli::parse();

    let p = args.path.as_path();

    println!("In file {:?}", p);

    let contents = fs::read_to_string(p).expect("Should have been able to read the file");

    let total_score = handler(&contents);

    let desired_score = handler_2(&contents);

    println!("part_1: {}", total_score);

    println!("part_2: {}", desired_score);
}

#[cfg(test)]

mod tests {

    use super::*;

    const TEST_INPUT: &str = "A Y
B X
C Z
";
    #[test]
    fn part_1() {
        let result: u32 = handler(TEST_INPUT);

        assert_eq!(result, 15);
    }

    #[test]
    fn part_2() {
        let result: u32 = handler_2(TEST_INPUT);

        assert_eq!(result, 12);
    }
}
